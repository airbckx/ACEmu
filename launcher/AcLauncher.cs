// ACEmu launcher - takes the place of the retail launcher, which cannot work
// without the services it was built against.
//
// What it does before starting ArcticCombat.exe:
//   1. applies the three client patches the emulator needs (skip-hshield,
//      no-packet-crypto, no-web-browser), verifying the expected bytes first
//      and keeping a .orig backup of every file it touches
//   2. writes System\Test.ini so the client dials our login server
//   3. maps the retail login host names to that server in the hosts file
//      (asks for elevation only when a line is missing)
//   4. asks the login server's launcher endpoint whether the account exists
//      and the password is right; creates it when unknown
//   5. builds the argument blob the client expects and starts the game
//
// A borderless 900x650 window drawn at the screen's DPI scale. It paints
// itself with flat panels and buttons; bitmaps present in launcher\skin\ at
// build time are used instead (see README.md). No artwork ships here.
//
// Built by build.ps1 with the C# compiler that ships with the .NET Framework,
// so the source has to stay C# 5 (no string interpolation, no ?. etc).

using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Drawing.Imaging;
using System.IO;
using System.Linq;
using System.Net;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using System.Web.Script.Serialization;
using System.Windows.Forms;
using Microsoft.Win32;

[assembly: AssemblyTitle("ACEmu Launcher")]
[assembly: AssemblyProduct("ACEmu")]
[assembly: AssemblyVersion("0.2.0.0")]

namespace AcLauncher
{
    static class Program
    {
        [STAThread]
        static int Main(string[] args)
        {
            // Elevated helper mode: the main instance re-launches itself with
            // "runas" and this argument when the hosts file needs a change.
            if ((args.Length >= 2 && args[0] == "--fix-hosts") || (args.Length >= 1 && args[0] == "--unfix-hosts"))
            {
                try
                {
                    if (args[0] == "--fix-hosts") Hosts.Write(args[1]); else Hosts.Remove();
                    return 0;
                }
                catch (Exception e)
                {
                    MessageBox.Show(e.Message, "ACEmu launcher - hosts file", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    return 1;
                }
            }
            // Headless modes (no window, result in a text file):
            //   --check [report.txt]             read-only status: patches, Test.ini, hosts, login server
            //   --patch <gamedir> [report.txt]   apply the client patches and Test.ini to that folder
            //   --restore <gamedir> [report.txt] put the original files back (from the .orig backups)
            //   --renderer | --tweaks <gamedir> on|off [report.txt]  one client option on or off
            if (args.Length >= 1 && (args[0] == "--check" || args[0] == "--patch" || args[0] == "--restore"
                || args[0] == "--renderer" || args[0] == "--tweaks"))
            {
                var s = Settings.Load();
                var sb = new StringBuilder();
                int rc = 0;
                string report = args.Length >= 2 ? args[args.Length - 1] : Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "launcher-check.txt");
                try
                {
                    if (args[0] == "--renderer" || args[0] == "--tweaks")
                    {
                        if (args.Length < 3) throw new Exception("usage: " + args[0] + " <gamedir> on|off [report.txt]");
                        s.GameDir = Path.GetFullPath(args[1]);
                        report = args.Length >= 4 ? args[3] : Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "launcher-check.txt");
                        Action<string> log = delegate(string line) { sb.AppendLine(line); };
                        Action<int, string> quiet = delegate(int pct, string text) { };
                        bool on = args[2] == "on";
                        var api = on && args[0] == "--renderer" ? new LoginApi(Checks.ResolveIp(s.Server), s.ApiPort) : null;
                        if (args[0] == "--renderer") log(on ? Renderer.Apply(api, s.GameDir, quiet, log) : "Modern renderer off: " + Renderer.Remove(s.GameDir) + " file(s) removed");
                        else log(on ? IniTweaks.Apply(s.GameDir) : "Performance tweaks off: " + IniTweaks.Remove(s.GameDir) + " section(s) removed");
                    }
                    else if (args[0] != "--check")
                    {
                        if (args.Length < 2) throw new Exception("usage: " + args[0] + " <gamedir> [report.txt]");
                        s.GameDir = Path.GetFullPath(args[1]);
                        if (args.Length < 3) report = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "launcher-check.txt");
                        Action<string> log = delegate(string line) { sb.AppendLine(line); };
                        if (args[0] == "--patch") Checks.Prepare(s, Checks.ResolveIp(s.Server), log);
                        else Checks.Restore(s, log);
                    }
                    if (args[0] != "--renderer" && args[0] != "--tweaks") Checks.Report(s, delegate(string line) { sb.AppendLine(line); });
                }
                catch (Exception e) { sb.AppendLine("FAILED: " + e.Message); rc = 1; }
                File.WriteAllText(report, sb.ToString());
                return rc;
            }
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new MainForm());
            return 0;
        }
    }

    // -----------------------------------------------------------------------
    // settings (launcher.ini next to the exe)

    class Settings
    {
        public string GameDir = "";
        public string Server = "127.0.0.1";
        public string Username = "";
        public string Region = "US";
        public string Language = "INT";
        /// The game's login port (goes into Test.ini).
        public int LoginPort = 55000;
        /// The login server's HTTP endpoint for the launcher.
        public int ApiPort = 55001;
        public bool Remember = true;
        /// dgVoodoo2 D3D9 wrapper in System\.
        public bool ModernRenderer = false;
        /// The engine ini sections from docs/CLIENT-TWEAKS.md.
        public bool PerfTweaks = false;

        static string IniPath { get { return Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "launcher.ini"); } }

        /// Where the default game folder came from ("Steam", "registry", ...),
        /// for the status log. Empty when it was read from launcher.ini.
        public string GameDirSource = "";

        public static Settings Load()
        {
            var s = new Settings();
            s.GameDir = GameLocator.Find(out s.GameDirSource);
            if (!File.Exists(IniPath)) return s;
            foreach (var raw in File.ReadAllLines(IniPath))
            {
                var line = raw.Trim();
                int eq = line.IndexOf('=');
                if (line.StartsWith(";") || line.StartsWith("#") || eq < 0) continue;
                var key = line.Substring(0, eq).Trim();
                var val = line.Substring(eq + 1).Trim();
                int n;
                switch (key)
                {
                    case "GameDir": if (val.Length > 0) { s.GameDir = val; s.GameDirSource = ""; } break;
                    case "Server": s.Server = val; break;
                    case "Username": s.Username = val; break;
                    case "Region": s.Region = val; break;
                    case "Language": s.Language = val; break;
                    case "LoginPort": if (int.TryParse(val, out n)) s.LoginPort = n; break;
                    case "ApiPort": if (int.TryParse(val, out n)) s.ApiPort = n; break;
                    case "Remember": s.Remember = val != "0"; break;
                    case "ModernRenderer": s.ModernRenderer = val == "1"; break;
                    case "PerfTweaks": s.PerfTweaks = val == "1"; break;
                }
            }
            return s;
        }

        public void Save()
        {
            var lines = new[] {
                "; ACEmu launcher settings",
                "GameDir=" + GameDir,
                "Server=" + Server,
                "Username=" + (Remember ? Username : ""),
                "Remember=" + (Remember ? "1" : "0"),
                "ModernRenderer=" + (ModernRenderer ? "1" : "0"),
                "PerfTweaks=" + (PerfTweaks ? "1" : "0"),
                "Region=" + Region,
                "Language=" + Language,
                "LoginPort=" + LoginPort,
                "ApiPort=" + ApiPort,
            };
            File.WriteAllLines(IniPath, lines, new UTF8Encoding(false));
        }
    }

    // -----------------------------------------------------------------------
    // where is the game? Steam first (app 212370, install folder "ArcticCombat"),
    // then the registry key Steam's install script writes, then a copy next to
    // this repo, then the stock Steam path.

    static class GameLocator
    {
        public const string SteamAppId = "212370";
        public const string SteamInstallUrl = "steam://install/" + SteamAppId;

        static bool HasGame(string dir)
        {
            return !string.IsNullOrEmpty(dir) && File.Exists(Path.Combine(dir, @"System\ArcticCombat.exe"));
        }

        static string Reg(RegistryKey root, string key, string value)
        {
            try
            {
                using (var k = root.OpenSubKey(key))
                    return k == null ? null : k.GetValue(value) as string;
            }
            catch (Exception) { return null; }
        }

        /// Every Steam library folder on this machine (the main one first).
        static IEnumerable<string> SteamLibraries()
        {
            var roots = new List<string>();
            foreach (var p in new[] {
                Reg(Registry.CurrentUser, @"Software\Valve\Steam", "SteamPath"),
                Reg(Registry.LocalMachine, @"SOFTWARE\WOW6432Node\Valve\Steam", "InstallPath"),
                Reg(Registry.LocalMachine, @"SOFTWARE\Valve\Steam", "InstallPath"),
                Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ProgramFilesX86), "Steam") })
            {
                if (string.IsNullOrEmpty(p)) continue;
                var full = Path.GetFullPath(p.Replace('/', '\\'));
                if (!roots.Contains(full, StringComparer.OrdinalIgnoreCase) && Directory.Exists(full)) roots.Add(full);
            }
            var libs = new List<string>(roots);
            foreach (var root in roots)
            {
                var vdf = Path.Combine(root, @"steamapps\libraryfolders.vdf");
                if (!File.Exists(vdf)) continue;
                foreach (Match m in Regex.Matches(File.ReadAllText(vdf), "\"path\"\\s+\"([^\"]+)\""))
                {
                    var lib = Path.GetFullPath(m.Groups[1].Value.Replace("\\\\", "\\"));
                    if (!libs.Contains(lib, StringComparer.OrdinalIgnoreCase) && Directory.Exists(lib)) libs.Add(lib);
                }
            }
            return libs;
        }

        public static string Find(out string source)
        {
            foreach (var lib in SteamLibraries())
            {
                var apps = Path.Combine(lib, "steamapps");
                var manifest = Path.Combine(apps, "appmanifest_" + SteamAppId + ".acf");
                if (File.Exists(manifest))
                {
                    var m = Regex.Match(File.ReadAllText(manifest), "\"installdir\"\\s+\"([^\"]+)\"");
                    if (m.Success && HasGame(Path.Combine(apps, "common", m.Groups[1].Value)))
                    {
                        source = "Steam";
                        return Path.Combine(apps, "common", m.Groups[1].Value);
                    }
                }
                if (HasGame(Path.Combine(apps, @"common\ArcticCombat")))
                {
                    source = "Steam";
                    return Path.Combine(apps, @"common\ArcticCombat");
                }
            }
            var reg = Reg(Registry.CurrentUser, @"Software\Webzen\ArcticCombat", "path");
            if (HasGame(reg)) { source = "registry"; return Path.GetFullPath(reg); }
            var sibling = Path.GetFullPath(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, @"..\ArcticCombat"));
            if (HasGame(sibling)) { source = "next to the launcher"; return sibling; }
            // Nothing found: offer the folder next to the launcher as the
            // place to put an install, and let the player pick another.
            source = "no install found - pick one with ...";
            return Path.GetFullPath(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "ArcticCombat"));
        }
    }

    // -----------------------------------------------------------------------
    // the launcher argument blob (see launcher/README.md, "Gate 1")

    static class Blob
    {
        const string Key = "wzlAUnchEr";

        public static string Encode(params string[] fields)
        {
            var bytes = Encoding.ASCII.GetBytes(string.Join("|", fields));
            var k = Encoding.ASCII.GetBytes(Key);
            for (int i = 0; i < bytes.Length; i++) bytes[i] ^= k[i % k.Length];
            return Convert.ToBase64String(bytes);
        }
    }

    // -----------------------------------------------------------------------
    // client patches

    class Edit { public int Offset; public byte[] Expect; public byte[] Replace; }

    class Patch
    {
        public string Name;
        public string File;
        public Edit[] Edits;
    }

    enum PatchState { Unpatched, Patched, Mixed, Unknown, MissingFile }

    static class Patches
    {
        public static readonly Patch[] Required = {
            new Patch {
                // NOPs the call to UHShieldManager::RunHShield (HackShield can no longer update)
                Name = "skip-hshield", File = @"System\ArcticCombat.exe",
                Edits = new[] { new Edit { Offset = 0x00FC7C,
                    Expect  = new byte[] { 0xFF, 0x15, 0xC4, 0x04, 0x92, 0x10 },
                    Replace = new byte[] { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 } } }
            },
            new Patch {
                // jz -> jmp around SetNetBufferWithDecrypt / SetNetBufferWithEncrypt
                Name = "no-packet-crypto", File = @"System\BatteryNet.dll",
                Edits = new[] {
                    new Edit { Offset = 0x03B881, Expect = new byte[] { 0x74, 0x57 }, Replace = new byte[] { 0xEB, 0x57 } },
                    new Edit { Offset = 0x03BAB0, Expect = new byte[] { 0x74, 0x40 }, Replace = new byte[] { 0xEB, 0x40 } } }
            },
            new Patch {
                // UWindowsViewport::OpenWebInClient -> ret 0x14 (the IE control crashes on Win 11)
                Name = "no-web-browser", File = @"System\WinDrv.dll",
                Edits = new[] { new Edit { Offset = 0x01DD70,
                    Expect  = new byte[] { 0x55, 0x8B, 0xEC },
                    Replace = new byte[] { 0xC2, 0x14, 0x00 } } }
            },
        };

        public static PatchState Check(string gameDir, Patch p)
        {
            var full = Path.Combine(gameDir, p.File);
            if (!File.Exists(full)) return PatchState.MissingFile;
            var states = new List<PatchState>();
            using (var fs = File.Open(full, FileMode.Open, FileAccess.Read, FileShare.ReadWrite))
            {
                foreach (var e in p.Edits)
                {
                    var buf = new byte[e.Expect.Length];
                    fs.Seek(e.Offset, SeekOrigin.Begin);
                    int n = fs.Read(buf, 0, buf.Length);
                    if (n != buf.Length) states.Add(PatchState.Unknown);
                    else if (buf.SequenceEqual(e.Expect)) states.Add(PatchState.Unpatched);
                    else if (buf.SequenceEqual(e.Replace)) states.Add(PatchState.Patched);
                    else states.Add(PatchState.Unknown);
                }
            }
            if (states.Contains(PatchState.Unknown)) return PatchState.Unknown;
            if (states.All(s => s == PatchState.Patched)) return PatchState.Patched;
            if (states.All(s => s == PatchState.Unpatched)) return PatchState.Unpatched;
            return PatchState.Mixed;
        }

        public static string BackupOf(string gameDir, Patch p) { return Path.Combine(gameDir, p.File) + ".orig"; }

        /// Puts the original file back from its .orig backup (kept, so the
        /// next patch run reuses it). False when there is no backup.
        public static bool Restore(string gameDir, Patch p)
        {
            var full = Path.Combine(gameDir, p.File);
            var backup = BackupOf(gameDir, p);
            if (!File.Exists(backup)) return false;
            File.Copy(backup, full, true);
            return true;
        }

        /// Applies one patch. The caller has checked the state is Unpatched.
        public static void Apply(string gameDir, Patch p)
        {
            var full = Path.Combine(gameDir, p.File);
            var backup = full + ".orig";
            if (!File.Exists(backup)) File.Copy(full, backup);
            using (var fs = File.Open(full, FileMode.Open, FileAccess.Write, FileShare.None))
            {
                foreach (var e in p.Edits)
                {
                    fs.Seek(e.Offset, SeekOrigin.Begin);
                    fs.Write(e.Replace, 0, e.Replace.Length);
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // System\Test.ini - the redirect that actually matters (launcher/README.md)

    static class TestIni
    {
        public static string PathFor(string gameDir) { return Path.Combine(gameDir, @"System\Test.ini"); }

        public static string Content(string ip, int port)
        {
            return string.Join("\r\n", new[] {
                "; written by the ACEmu launcher - login server override",
                "; read by BTNetMainMenu::LoadLoginServerList_Ini (GUIWarfare.u) via GETTEST_STRING",
                "",
                "[TEST]",
                "SERVER_COUNT=1",
                "SERVER_IP[0]=" + ip,
                "SERVER_PORT[0]=" + port,
                "",
                "; single-server override form (the same function also reads these)",
                "IP=" + ip,
                "Port=" + port,
                "" });
        }

        public static bool IsCurrent(string gameDir, string ip, int port)
        {
            var p = PathFor(gameDir);
            if (!File.Exists(p)) return false;
            var text = File.ReadAllText(p);
            return Regex.IsMatch(text, @"(?m)^\s*SERVER_IP\[0\]\s*=\s*" + Regex.Escape(ip) + @"\s*$")
                && Regex.IsMatch(text, @"(?m)^\s*SERVER_PORT\[0\]\s*=\s*" + port + @"\s*$")
                && Regex.IsMatch(text, @"(?m)^\s*IP\s*=\s*" + Regex.Escape(ip) + @"\s*$")
                && Regex.IsMatch(text, @"(?m)^\s*Port\s*=\s*" + port + @"\s*$");
        }

        public static void Write(string gameDir, string ip, int port)
        {
            File.WriteAllText(PathFor(gameDir), Content(ip, port), Encoding.ASCII);
        }
    }

    // -----------------------------------------------------------------------
    // hosts file

    static class Hosts
    {
        public static readonly string[] Names = { "acuslogin01.webzen.com", "aceulogin01.webzen.com", "acsalogin01.webzen.com" };

        public static string FilePath
        {
            get { return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.System), @"drivers\etc\hosts"); }
        }

        /// (ip, names) of one hosts line, or null for comments/blank lines.
        static KeyValuePair<string, string[]>? Parse(string raw)
        {
            var line = raw;
            int hash = line.IndexOf('#');
            if (hash >= 0) line = line.Substring(0, hash);
            var parts = line.Split(new[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length < 2) return null;
            return new KeyValuePair<string, string[]>(parts[0], parts.Skip(1).ToArray());
        }

        /// The login host names that are not mapped to `ip` yet.
        public static string[] Missing(string ip)
        {
            var mapped = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
            if (File.Exists(FilePath))
            {
                foreach (var raw in File.ReadAllLines(FilePath))
                {
                    var e = Parse(raw);
                    if (e == null || e.Value.Key != ip) continue;
                    foreach (var n in e.Value.Value) mapped.Add(n);
                }
            }
            return Names.Where(n => !mapped.Contains(n)).ToArray();
        }

        /// Whether any of the three names is mapped at all (to any address).
        public static bool AnyMapped()
        {
            if (!File.Exists(FilePath)) return false;
            return File.ReadAllLines(FilePath).Select(Parse).Any(e => e != null && e.Value.Value.Any(n => Names.Contains(n, StringComparer.OrdinalIgnoreCase)));
        }

        /// The hosts file without any line for our three names.
        static List<string> WithoutOurs()
        {
            var keep = new List<string>();
            if (File.Exists(FilePath))
            {
                foreach (var raw in File.ReadAllLines(FilePath))
                {
                    var e = Parse(raw);
                    if (e != null && e.Value.Value.Any(n => Names.Contains(n, StringComparer.OrdinalIgnoreCase))) continue;
                    keep.Add(raw);
                }
            }
            while (keep.Count > 0 && keep[keep.Count - 1].Trim().Length == 0) keep.RemoveAt(keep.Count - 1);
            return keep;
        }

        /// Rewrites the hosts file: drops every existing line for our three
        /// names and appends fresh ones. Needs administrator rights.
        public static void Write(string ip)
        {
            var keep = WithoutOurs();
            keep.Add("");
            foreach (var n in Names) keep.Add(ip + "\t" + n + "\t# ACEmu");
            File.WriteAllLines(FilePath, keep, Encoding.ASCII);
        }

        /// Drops our three names from the hosts file. Needs administrator rights.
        public static void Remove()
        {
            var keep = WithoutOurs();
            keep.Add("");
            File.WriteAllLines(FilePath, keep, Encoding.ASCII);
        }

        /// Runs this exe elevated to write the hosts file. False when the UAC
        /// prompt was declined or the helper failed. `ip` null removes the lines.
        public static bool FixElevated(string ip)
        {
            var psi = new ProcessStartInfo(Application.ExecutablePath, ip == null ? "--unfix-hosts" : "--fix-hosts " + ip)
            {
                Verb = "runas",
                UseShellExecute = true,
            };
            try
            {
                using (var p = Process.Start(psi))
                {
                    p.WaitForExit();
                    return p.ExitCode == 0;
                }
            }
            catch (Win32Exception) { return false; }
        }
    }

    // -----------------------------------------------------------------------
    // the login server's launcher endpoint (acemu-login, http://server:55001)
    //
    //   GET  /ping                           -> {"ok":true}
    //   POST /ensure {"username","password"} -> {"result": ok|created|password_set|wrong|no_password}

    class LoginException : Exception { public LoginException(string m) : base(m) { } }

    class LoginApi
    {
        readonly string baseUrl;
        readonly JavaScriptSerializer js = new JavaScriptSerializer();

        public LoginApi(string host, int port) { baseUrl = "http://" + host + ":" + port; }

        public string BaseUrl { get { return baseUrl; } }

        Dictionary<string, object> Call(string method, string path, object body, int timeoutMs, out int status)
        {
            var req = (HttpWebRequest)WebRequest.Create(baseUrl + path);
            req.Method = method;
            req.Timeout = timeoutMs;
            req.Accept = "application/json";
            if (body != null)
            {
                req.ContentType = "application/json";
                var b = Encoding.UTF8.GetBytes(js.Serialize(body));
                req.ContentLength = b.Length;
                using (var s = req.GetRequestStream()) s.Write(b, 0, b.Length);
            }
            HttpWebResponse resp;
            try { resp = (HttpWebResponse)req.GetResponse(); }
            catch (WebException we)
            {
                if (we.Response == null) throw new LoginException("login server not reachable at " + baseUrl + " (" + we.Message + ") - start acemu-login");
                resp = (HttpWebResponse)we.Response;
            }
            using (resp)
            using (var r = new StreamReader(resp.GetResponseStream()))
            {
                status = (int)resp.StatusCode;
                var txt = r.ReadToEnd();
                if (string.IsNullOrWhiteSpace(txt)) return new Dictionary<string, object>();
                object o;
                try { o = js.DeserializeObject(txt); }
                catch (Exception) { return new Dictionary<string, object> { { "raw", txt } }; }
                return o as Dictionary<string, object> ?? new Dictionary<string, object> { { "value", o } };
            }
        }

        static string ErrorOf(Dictionary<string, object> v, int status)
        {
            object e;
            if (v.TryGetValue("error", out e) && e != null) return e.ToString();
            return "HTTP " + status;
        }

        public bool Ping()
        {
            try
            {
                int st;
                var v = Call("GET", "/ping", null, 1500, out st);
                return st == 200 && v.ContainsKey("ok");
            }
            catch (Exception) { return false; }
        }

        /// A JSON GET, or null on a non-200 answer.
        public Dictionary<string, object> Get(string path)
        {
            int st;
            var v = Call("GET", path, null, 8000, out st);
            return st == 200 ? v : null;
        }

        /// The account step: makes sure the account exists with this password.
        /// Returns a one-line description; throws when the game would be refused.
        public string EnsureAccount(string user, string pass)
        {
            int st;
            var v = Call("POST", "/ensure", new Dictionary<string, object> { { "username", user }, { "password", pass } }, 4000, out st);
            if (st == 404) throw new LoginException("the login server has no /ensure endpoint - rebuild and restart acemu-login");
            if (st != 200) throw new LoginException("account check failed: " + ErrorOf(v, st));
            var result = (v.ContainsKey("result") ? v["result"] as string : null) ?? "?";
            switch (result)
            {
                case "ok": return "account '" + user + "' ok";
                case "no_password": return "account '" + user + "' ok (no password set)";
                case "created": return "account '" + user + "' created" + (pass.Length == 0 ? " (no password)" : "");
                case "password_set": return "account '" + user + "' had no password - set to the one you entered";
                case "wrong": throw new LoginException("wrong password for account '" + user + "'");
                default: throw new LoginException("unexpected answer '" + result + "' from the login server");
            }
        }
    }

    // -----------------------------------------------------------------------
    // packs from the login server (GET /<pack> manifest, GET /<pack>/<name>):
    // downloaded by sha256 into a cache folder, resumable, verified.

    class PackFile { public string Name; public long Size; public string Sha256; }

    static class Packs
    {
        public static string Sha(string path)
        {
            using (var h = System.Security.Cryptography.SHA256.Create())
            using (var f = File.OpenRead(path))
                return BitConverter.ToString(h.ComputeHash(f)).Replace("-", "").ToLowerInvariant();
        }

        public static string Mb(long b) { return (b / 1048576.0).ToString("N0") + " MB"; }

        public static Dictionary<string, string> LoadState(string path)
        {
            try
            {
                if (!File.Exists(path)) return new Dictionary<string, string>();
                var o = new JavaScriptSerializer().DeserializeObject(File.ReadAllText(path)) as Dictionary<string, object>;
                var d = new Dictionary<string, string>();
                if (o != null) foreach (var kv in o) d[kv.Key] = kv.Value as string ?? "";
                return d;
            }
            catch (Exception) { return new Dictionary<string, string>(); }
        }

        public static void SaveState(string path, Dictionary<string, string> state)
        {
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            File.WriteAllText(path, new JavaScriptSerializer().Serialize(state));
        }

        /// GET /<pack>, or null when the server has no such pack.
        public static List<PackFile> Manifest(LoginApi api, string pack, out string version)
        {
            version = "";
            var v = api.Get("/" + pack);
            if (v == null || !v.ContainsKey("files")) return null;
            version = v.ContainsKey("version") ? Convert.ToString(v["version"]) : "";
            var list = new List<PackFile>();
            foreach (var o in v["files"] as object[] ?? new object[0])
            {
                var d = o as Dictionary<string, object>;
                if (d == null || !d.ContainsKey("name") || !d.ContainsKey("size") || !d.ContainsKey("sha256")) continue;
                var name = Convert.ToString(d["name"]);
                if (name.Length == 0 || name.IndexOfAny(new[] { '/', '\\', ':' }) >= 0 || name.Contains("..") || name.StartsWith(".")) continue;
                list.Add(new PackFile { Name = name, Size = Convert.ToInt64(d["size"]), Sha256 = Convert.ToString(d["sha256"]).ToLowerInvariant() });
            }
            return list;
        }

        /// One file into the cache, resuming a .part if the server supports ranges.
        static void Fetch(LoginApi api, string pack, PackFile f, string cacheDir, Action<long> progress)
        {
            var final = Path.Combine(cacheDir, f.Name);
            var part = final + ".part";
            long have = File.Exists(part) ? new FileInfo(part).Length : 0;
            if (have >= f.Size) { File.Delete(part); have = 0; }
            var req = (HttpWebRequest)WebRequest.Create(api.BaseUrl + "/" + pack + "/" + Uri.EscapeDataString(f.Name));
            req.Timeout = 30000;
            req.ReadWriteTimeout = 60000;
            if (have > 0) req.AddRange(have);
            using (var resp = (HttpWebResponse)req.GetResponse())
            {
                bool resumed = resp.StatusCode == HttpStatusCode.PartialContent && have > 0;
                if (resp.StatusCode != HttpStatusCode.OK && !resumed) throw new LoginException("download of " + f.Name + " failed: HTTP " + (int)resp.StatusCode);
                using (var input = resp.GetResponseStream())
                using (var output = new FileStream(part, resumed ? FileMode.Append : FileMode.Create, FileAccess.Write))
                {
                    long got = resumed ? have : 0;
                    var buf = new byte[256 * 1024];
                    int n;
                    while ((n = input.Read(buf, 0, buf.Length)) > 0)
                    {
                        output.Write(buf, 0, n);
                        got += n;
                        progress(n);
                    }
                    if (got != f.Size) throw new LoginException("download of " + f.Name + " ended after " + Mb(got) + " of " + Mb(f.Size));
                }
            }
            var actual = Sha(part);
            if (actual != f.Sha256) { File.Delete(part); throw new LoginException("download of " + f.Name + " is corrupt (sha256 mismatch) - try again later"); }
            if (File.Exists(final)) File.Delete(final);
            File.Move(part, final);
            File.WriteAllText(final + ".sha256", f.Sha256);
        }

        static bool Cached(PackFile f, string cacheDir)
        {
            var final = Path.Combine(cacheDir, f.Name);
            if (!File.Exists(final) || new FileInfo(final).Length != f.Size) return false;
            var side = final + ".sha256";
            return File.Exists(side) && File.ReadAllText(side).Trim().ToLowerInvariant() == f.Sha256;
        }

        /// Brings the cache up to the manifest (stage 0..100 for the download).
        /// Returns the manifest; throws when the server has no such pack.
        public static List<PackFile> Sync(LoginApi api, string pack, string label, string cacheDir, Action<int, string> stage, Action<string> log, out string version)
        {
            var files = Manifest(api, pack, out version);
            if (files == null) throw new LoginException("the server at " + api.BaseUrl + " offers no " + label + " pack");
            if (files.Count == 0) throw new LoginException("the server's " + label + " pack is empty");
            Directory.CreateDirectory(cacheDir);
            var todo = files.Where(f => !Cached(f, cacheDir)).ToList();
            long total = todo.Sum(f => f.Size), done = 0;
            if (todo.Count > 0) log(label + " " + version + ": " + todo.Count + " of " + files.Count + " files to download (" + Mb(total) + ")");
            else log(label + " " + version + ": all " + files.Count + " files cached");
            var watch = Stopwatch.StartNew();
            long lastTick = -1;
            for (int i = 0; i < todo.Count; i++)
            {
                var f = todo[i];
                int idx = i;
                Fetch(api, pack, f, cacheDir, delegate(long n)
                {
                    done += n;
                    if (watch.ElapsedMilliseconds - lastTick > 250)
                    {
                        lastTick = watch.ElapsedMilliseconds;
                        double mbps = done / 1048576.0 / Math.Max(0.001, watch.Elapsed.TotalSeconds);
                        stage((int)(done * 100 / Math.Max(1, total)), label + ": " + (idx + 1) + "/" + todo.Count + "  " + Mb(done) + " / " + Mb(total) + "  (" + mbps.ToString("N1") + " MB/s)");
                    }
                });
            }
            stage(100, label + ": files ready");
            return files;
        }
    }

    // -----------------------------------------------------------------------
    // "Modern renderer": dgVoodoo2's D3D9.dll + dgVoodoo.conf dropped into
    // System\ (docs/CLIENT-TWEAKS.md). The files come from the server's
    // "extras" pack, cached in <GameDir>\Extras\; a pre-existing foreign
    // System\<name> is kept in Extras\backup\ and put back when turned off.

    static class Renderer
    {
        const string StateFile = "applied.json";

        static string Cache(string dir) { return Path.Combine(dir, "Extras"); }
        static string Backup(string dir) { return Path.Combine(Cache(dir), "backup"); }
        static string StatePath(string dir) { return Path.Combine(Cache(dir), StateFile); }
        static string SystemDir(string dir) { return Path.Combine(dir, "System"); }

        public static bool IsApplied(string dir) { return File.Exists(StatePath(dir)); }

        public static string Apply(LoginApi api, string dir, Action<int, string> stage, Action<string> log)
        {
            string version;
            var files = Packs.Sync(api, "extras", "Modern renderer", Cache(dir), delegate(int pct, string text) { stage(pct * 70 / 100, text); }, log, out version);
            var state = Packs.LoadState(StatePath(dir));
            int installed = 0, kept = 0;
            foreach (var f in files)
            {
                var target = Path.Combine(SystemDir(dir), f.Name);
                var cached = Path.Combine(Cache(dir), f.Name);
                string cur;
                if (state.TryGetValue(f.Name, out cur) && cur == f.Sha256 && File.Exists(target) && Packs.Sha(target) == f.Sha256) { kept++; continue; }
                if (File.Exists(target) && !state.ContainsKey(f.Name))
                {
                    // not ours: keep it for when the option is turned off
                    Directory.CreateDirectory(Backup(dir));
                    var bak = Path.Combine(Backup(dir), f.Name);
                    if (!File.Exists(bak)) File.Move(target, bak);
                }
                File.Copy(cached, target, true);
                state[f.Name] = f.Sha256;
                installed++;
            }
            Packs.SaveState(StatePath(dir), state);
            stage(100, "Modern renderer installed");
            return "Modern renderer " + version + ": " + installed + " file(s) installed into System\\, " + kept + " already in place";
        }

        /// Removes what was installed and puts any backed-up file back. Offline.
        public static int Remove(string dir)
        {
            var state = Packs.LoadState(StatePath(dir));
            int n = 0;
            foreach (var name in state.Keys)
            {
                var target = Path.Combine(SystemDir(dir), name);
                if (File.Exists(target)) { File.Delete(target); n++; }
                var bak = Path.Combine(Backup(dir), name);
                if (File.Exists(bak)) File.Move(bak, target);
            }
            if (File.Exists(StatePath(dir))) File.Delete(StatePath(dir));
            return n;
        }
    }

    // -----------------------------------------------------------------------
    // "Performance tweaks": three engine sections merged into
    // System\ArcticCombat.ini by section and key (docs/CLIENT-TWEAKS.md); off
    // removes exactly those sections. The game's own sections are not touched.

    static class IniTweaks
    {
        static readonly string[][] Sections = {
            new[] { "D3D9Drv.D3D9RenderDevice", "ReduceMouseLag=False", "UseVSync=False", "UseTripleBuffering=False" },
            new[] { "Engine.LevelInfo", "MaxClientFrameRate=144.000000" },
            new[] { "Engine.PlayerController", "DefaultFOV=100.000000", "DesiredFOV=100.000000" },
        };

        static string IniPath(string dir) { return Path.Combine(dir, @"System\ArcticCombat.ini"); }

        static bool IsHeader(string line, string name)
        {
            var t = line.Trim();
            return t.Length > 2 && t[0] == '[' && t[t.Length - 1] == ']' && string.Equals(t.Substring(1, t.Length - 2).Trim(), name, StringComparison.OrdinalIgnoreCase);
        }

        static bool IsAnyHeader(string line)
        {
            var t = line.Trim();
            return t.Length > 2 && t[0] == '[' && t[t.Length - 1] == ']';
        }

        static bool IsKey(string line, string key)
        {
            int eq = line.IndexOf('=');
            return eq > 0 && string.Equals(line.Substring(0, eq).Trim(), key, StringComparison.OrdinalIgnoreCase);
        }

        /// All three sections present.
        public static bool IsApplied(string dir)
        {
            var p = IniPath(dir);
            if (!File.Exists(p)) return false;
            var lines = File.ReadAllLines(p, Encoding.Default);
            return Sections.All(sec => lines.Any(l => IsHeader(l, sec[0])));
        }

        /// Merges the sections in; idempotent. Returns a summary.
        public static string Apply(string dir)
        {
            var p = IniPath(dir);
            if (!File.Exists(p)) throw new LoginException("no System\\ArcticCombat.ini to tweak - start the game once first");
            var lines = File.ReadAllLines(p, Encoding.Default).ToList();
            int added = 0, changed = 0;
            foreach (var sec in Sections)
            {
                int head = lines.FindIndex(l => IsHeader(l, sec[0]));
                if (head < 0)
                {
                    while (lines.Count > 0 && lines[lines.Count - 1].Trim().Length == 0) lines.RemoveAt(lines.Count - 1);
                    lines.Add("");
                    lines.Add("[" + sec[0] + "]");
                    lines.AddRange(sec.Skip(1));
                    added++;
                    continue;
                }
                int end = head + 1;
                while (end < lines.Count && !IsAnyHeader(lines[end])) end++;
                int insertAt = end;
                while (insertAt > head + 1 && lines[insertAt - 1].Trim().Length == 0) insertAt--;
                foreach (var kv in sec.Skip(1))
                {
                    var key = kv.Substring(0, kv.IndexOf('='));
                    int at = -1;
                    for (int i = head + 1; i < end; i++) if (IsKey(lines[i], key)) { at = i; break; }
                    if (at >= 0) { if (lines[at].Trim() != kv) { lines[at] = kv; changed++; } }
                    else { lines.Insert(insertAt, kv); insertAt++; end++; changed++; }
                }
            }
            File.WriteAllLines(p, lines, Encoding.Default);
            return "Performance tweaks: " + added + " section(s) added, " + changed + " key(s) set in ArcticCombat.ini";
        }

        /// Removes exactly the three sections. Returns how many were there.
        public static int Remove(string dir)
        {
            var p = IniPath(dir);
            if (!File.Exists(p)) return 0;
            var lines = File.ReadAllLines(p, Encoding.Default).ToList();
            int removed = 0;
            foreach (var sec in Sections)
            {
                int head = lines.FindIndex(l => IsHeader(l, sec[0]));
                if (head < 0) continue;
                int end = head + 1;
                while (end < lines.Count && !IsAnyHeader(lines[end])) end++;
                lines.RemoveRange(head, end - head);
                while (head > 0 && head < lines.Count && lines[head].Trim().Length == 0 && lines[head - 1].Trim().Length == 0) lines.RemoveAt(head);
                removed++;
            }
            while (lines.Count > 0 && lines[lines.Count - 1].Trim().Length == 0) lines.RemoveAt(lines.Count - 1);
            lines.Add("");
            File.WriteAllLines(p, lines, Encoding.Default);
            return removed;
        }
    }

    // -----------------------------------------------------------------------
    // single-instance mutex (tools/ac-close-mutex.ps1, same code)

    static class MutexCloser
    {
        public const string MutexName = "qoxjfleoqkrskwkghkdlxld";

        [StructLayout(LayoutKind.Sequential)]
        struct SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX
        {
            public IntPtr Object; public IntPtr UniqueProcessId; public IntPtr HandleValue;
            public uint GrantedAccess; public ushort CreatorBackTraceIndex; public ushort ObjectTypeIndex;
            public uint HandleAttributes; public uint Reserved;
        }

        [DllImport("ntdll.dll")] static extern int NtQuerySystemInformation(int cls, IntPtr buf, int len, out int retLen);
        [DllImport("ntdll.dll")] static extern int NtQueryObject(IntPtr h, int cls, IntPtr buf, int len, out int retLen);
        [DllImport("kernel32.dll", SetLastError = true)] static extern IntPtr OpenProcess(uint access, bool inherit, int pid);
        [DllImport("kernel32.dll", SetLastError = true)] static extern bool DuplicateHandle(IntPtr srcProc, IntPtr srcH, IntPtr dstProc, out IntPtr dstH, uint access, bool inherit, uint options);
        [DllImport("kernel32.dll")] static extern IntPtr GetCurrentProcess();
        [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);

        const int SystemExtendedHandleInformation = 64;
        const int ObjectNameInformation = 1, ObjectTypeInformation = 2;
        const uint PROCESS_DUP_HANDLE = 0x40, DUPLICATE_CLOSE_SOURCE = 1, DUPLICATE_SAME_ACCESS = 2;

        static string QueryString(IntPtr h, int cls)
        {
            int len = 0x1000; IntPtr buf = Marshal.AllocHGlobal(len); int ret;
            try
            {
                int st = NtQueryObject(h, cls, buf, len, out ret);
                if (st != 0)
                {
                    Marshal.FreeHGlobal(buf); len = ret + 16; buf = Marshal.AllocHGlobal(len);
                    st = NtQueryObject(h, cls, buf, len, out ret);
                    if (st != 0) return null;
                }
                ushort l = (ushort)Marshal.ReadInt16(buf, 0);
                IntPtr p = Marshal.ReadIntPtr(buf, IntPtr.Size == 8 ? 8 : 4);
                if (l == 0 || p == IntPtr.Zero) return "";
                return Marshal.PtrToStringUni(p, l / 2);
            }
            finally { Marshal.FreeHGlobal(buf); }
        }

        /// Closes every Mutant handle in `pid` whose name contains `nameLike`.
        /// Returns how many were closed.
        public static int Close(int pid, string nameLike)
        {
            int closed = 0;
            int len = 1 << 20; IntPtr buf = Marshal.AllocHGlobal(len); int ret;
            int st;
            while ((st = NtQuerySystemInformation(SystemExtendedHandleInformation, buf, len, out ret)) == unchecked((int)0xC0000004))
            {
                Marshal.FreeHGlobal(buf); len = ret + (1 << 16); buf = Marshal.AllocHGlobal(len);
            }
            if (st != 0) { Marshal.FreeHGlobal(buf); throw new Exception("NtQuerySystemInformation failed: 0x" + st.ToString("X")); }
            IntPtr proc = OpenProcess(PROCESS_DUP_HANDLE, false, pid);
            if (proc == IntPtr.Zero) { Marshal.FreeHGlobal(buf); throw new Exception("OpenProcess failed: " + Marshal.GetLastWin32Error()); }
            try
            {
                long count = Marshal.ReadIntPtr(buf).ToInt64();
                int entrySize = Marshal.SizeOf(typeof(SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX));
                IntPtr first = IntPtr.Add(buf, IntPtr.Size * 2);
                for (long i = 0; i < count; i++)
                {
                    var e = (SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX)Marshal.PtrToStructure(IntPtr.Add(first, (int)(i * entrySize)), typeof(SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX));
                    if (e.UniqueProcessId.ToInt64() != pid) continue;
                    IntPtr dup;
                    if (!DuplicateHandle(proc, e.HandleValue, GetCurrentProcess(), out dup, 0, false, DUPLICATE_SAME_ACCESS)) continue;
                    try
                    {
                        if (QueryString(dup, ObjectTypeInformation) != "Mutant") continue;
                        string name = QueryString(dup, ObjectNameInformation) ?? "";
                        if (name.IndexOf(nameLike, StringComparison.OrdinalIgnoreCase) < 0) continue;
                        IntPtr dup2;
                        if (DuplicateHandle(proc, e.HandleValue, GetCurrentProcess(), out dup2, 0, false, DUPLICATE_CLOSE_SOURCE))
                        {
                            CloseHandle(dup2);
                            closed++;
                        }
                    }
                    finally { CloseHandle(dup); }
                }
            }
            finally { CloseHandle(proc); Marshal.FreeHGlobal(buf); }
            return closed;
        }
    }

    // -----------------------------------------------------------------------
    // read-only status report and the preparation steps (form, --check, --patch)

    static class Checks
    {
        public static string ResolveIp(string server)
        {
            IPAddress ip;
            if (IPAddress.TryParse(server, out ip)) return ip.ToString();
            var v4 = Dns.GetHostAddresses(server).FirstOrDefault(a => a.AddressFamily == System.Net.Sockets.AddressFamily.InterNetwork);
            if (v4 == null) throw new Exception("cannot resolve '" + server + "' to an IPv4 address");
            return v4.ToString();
        }

        /// Applies the missing client patches and writes Test.ini. Throws when a
        /// file has unexpected bytes or cannot be written.
        public static void Prepare(Settings s, string ip, Action<string> log)
        {
            foreach (var p in Patches.Required)
            {
                var state = Patches.Check(s.GameDir, p);
                switch (state)
                {
                    case PatchState.Patched: log("patch " + p.Name + ": already applied"); break;
                    case PatchState.Unpatched:
                        try { Patches.Apply(s.GameDir, p); }
                        catch (IOException e) { throw new Exception("cannot patch " + p.File + " (" + e.Message + ") - is the game running?"); }
                        log("patch " + p.Name + ": applied (" + p.File + ", backup .orig)");
                        break;
                    case PatchState.MissingFile: throw new Exception("missing " + Path.Combine(s.GameDir, p.File));
                    default: throw new Exception("patch " + p.Name + ": " + p.File + " has unexpected bytes (" + state + ") - wrong client build or half-applied patch");
                }
            }
            if (TestIni.IsCurrent(s.GameDir, ip, s.LoginPort)) log("Test.ini: points at " + ip + ":" + s.LoginPort);
            else { TestIni.Write(s.GameDir, ip, s.LoginPort); log("Test.ini: written (" + ip + ":" + s.LoginPort + ")"); }
        }

        /// Puts the client back the way it was: original files from the .orig
        /// backups, no Test.ini. The hosts lines are the caller's business
        /// (they need elevation).
        public static void Restore(Settings s, Action<string> log)
        {
            foreach (var p in Patches.Required)
            {
                var state = Patches.Check(s.GameDir, p);
                if (state == PatchState.MissingFile) { log("restore " + p.File + ": file missing, skipped"); continue; }
                if (state == PatchState.Unpatched) { log("restore " + p.File + ": already original"); continue; }
                try
                {
                    if (Patches.Restore(s.GameDir, p)) log("restore " + p.File + ": original put back from .orig");
                    else log("restore " + p.File + ": NO .orig backup - cannot restore (verify the files through Steam)");
                }
                catch (IOException e) { throw new Exception("cannot restore " + p.File + " (" + e.Message + ") - is the game running?"); }
            }
            var ini = TestIni.PathFor(s.GameDir);
            if (File.Exists(ini)) { File.Delete(ini); log("restore Test.ini: removed"); }
            else log("restore Test.ini: not present");
        }

        /// Read-only status. Returns whether the login server is listening.
        public static bool Report(Settings s, Action<string> log)
        {
            bool apiOk = false;
            log("game folder : " + (s.GameDir.Length == 0 ? "(not found - pick it with ...)" : s.GameDir + (s.GameDirSource.Length > 0 ? "  [" + s.GameDirSource + "]" : "")));
            var exe = Path.Combine(s.GameDir, @"System\ArcticCombat.exe");
            if (!File.Exists(exe)) { log("  ArcticCombat.exe not found"); }
            else
            {
                foreach (var p in Patches.Required)
                    log("  patch " + p.Name.PadRight(17) + Patches.Check(s.GameDir, p));
            }
            string ip = null;
            try { ip = ResolveIp(s.Server); } catch (Exception e) { log("server      : " + e.Message); }
            if (ip != null)
            {
                log("server      : " + s.Server + (ip != s.Server ? " (" + ip + ")" : ""));
                if (File.Exists(exe))
                    log("  Test.ini  " + (TestIni.IsCurrent(s.GameDir, ip, s.LoginPort) ? "points at " + ip + ":" + s.LoginPort : "needs update"));
                var missing = Hosts.Missing(ip);
                log("  hosts     " + (missing.Length == 0 ? "all 3 login names -> " + ip : missing.Length + " of 3 login names not mapped"));
                var api = new LoginApi(ip, s.ApiPort);
                apiOk = api.Ping();
                log("  login     " + (apiOk ? "server answers at " + api.BaseUrl : "server NOT reachable at " + api.BaseUrl + " - start acemu-login"));
            }
            return apiOk;
        }
    }

    // -----------------------------------------------------------------------
    // skin: optional bitmaps embedded as skin/*.png by build.ps1. None ship
    // with this project, and every loader returns null when a file is absent,
    // so the window falls back to plain drawn controls. Players who own the
    // game can drop the retail launcher's own bitmaps into launcher\skin\
    // before building and get the original look; see launcher\README.md.

    static class Skin
    {
        public static readonly Color Amber = Color.FromArgb(0xE3, 0xA0, 0x2A);
        public static readonly Color Text = Color.FromArgb(0xED, 0xE4, 0xCF);
        public static readonly Color Muted = Color.FromArgb(0x8D, 0x88, 0x7C);
        public static readonly Color LogText = Color.FromArgb(0xC9, 0xBF, 0xA5);
        public static readonly Color FieldBack = Color.FromArgb(0x0F, 0x0E, 0x0C);
        public static readonly Color FieldBorder = Color.FromArgb(0x3B, 0x37, 0x31);
        public static readonly Color PanelBack = Color.FromArgb(0x17, 0x16, 0x14);
        public static readonly Color Good = Color.FromArgb(0x6A, 0xC0, 0x4A);
        public static readonly Color Bad = Color.FromArgb(0xD0, 0x40, 0x30);

        /// An embedded skin bitmap, or null when this build has none.
        public static Bitmap Load(string name)
        {
            try
            {
                using (var s = Assembly.GetExecutingAssembly().GetManifestResourceStream("skin." + name))
                {
                    if (s == null) return null;
                    using (var img = new Bitmap(s)) return new Bitmap(img);
                }
            }
            catch (Exception) { return null; }
        }

        public static Icon LoadIcon()
        {
            try
            {
                using (var s = Assembly.GetExecutingAssembly().GetManifestResourceStream("skin.icon.ico"))
                    return s == null ? null : new Icon(s);
            }
            catch (Exception) { return null; }
        }

        /// The plain window background used when no skin is embedded: a dark
        /// panel with a hairline frame, at the window's 900x650 design size.
        public static Bitmap PlainBackground(float k)
        {
            int w = (int)Math.Round(900 * k), h = (int)Math.Round(650 * k);
            var bmp = new Bitmap(w, h, PixelFormat.Format32bppArgb);
            using (var g = Graphics.FromImage(bmp))
            using (var back = new SolidBrush(Color.FromArgb(0x10, 0x0F, 0x0D)))
            using (var head = new SolidBrush(PanelBack))
            using (var pen = new Pen(FieldBorder))
            {
                g.FillRectangle(back, 0, 0, w, h);
                g.FillRectangle(head, 0, 0, w, (int)Math.Round(120 * k));
                g.DrawRectangle(pen, 0, 0, w - 1, h - 1);
                using (var title = new Font("Segoe UI", 20f * k, FontStyle.Bold))
                    TextRenderer.DrawText(g, "ARCTIC COMBAT", title, new Point((int)Math.Round(28 * k), (int)Math.Round(40 * k)), Amber);
                using (var sub = new Font("Segoe UI", 9f * k))
                    TextRenderer.DrawText(g, "ACEmu launcher", sub, new Point((int)Math.Round(30 * k), (int)Math.Round(82 * k)), Muted);
            }
            return bmp;
        }

        /// The background scaled to the screen: the key colour (read from the
        /// top-left pixel) becomes transparent for the resampling and is put
        /// back afterwards, so the window's TransparencyKey still cuts the
        /// soldiers out cleanly and no pink fringe appears at the edges.
        public static Bitmap Background(Bitmap src, float k, out Color key)
        {
            key = src.GetPixel(0, 0);
            int w = (int)Math.Round(src.Width * k), h = (int)Math.Round(src.Height * k);
            var keyed = new Bitmap(src.Width, src.Height, PixelFormat.Format32bppArgb);
            using (var g = Graphics.FromImage(keyed)) g.DrawImageUnscaled(src, 0, 0);
            keyed.MakeTransparent(key);
            var pre = keyed.Clone(new Rectangle(0, 0, keyed.Width, keyed.Height), PixelFormat.Format32bppPArgb);
            var bmp = new Bitmap(w, h, PixelFormat.Format32bppArgb);
            using (var g = Graphics.FromImage(bmp))
            {
                g.Clear(Color.Transparent);
                g.InterpolationMode = k == 1f ? InterpolationMode.NearestNeighbor : InterpolationMode.HighQualityBicubic;
                g.PixelOffsetMode = PixelOffsetMode.Half;
                g.DrawImage(pre, new Rectangle(0, 0, w, h));
            }
            var rect = new Rectangle(0, 0, w, h);
            var data = bmp.LockBits(rect, ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);
            var px = new byte[data.Stride * h];
            Marshal.Copy(data.Scan0, px, 0, px.Length);
            for (int i = 0; i < px.Length; i += 4)
            {
                if (px[i + 3] < 128) { px[i] = key.B; px[i + 1] = key.G; px[i + 2] = key.R; }
                px[i + 3] = 255;
            }
            Marshal.Copy(px, 0, data.Scan0, px.Length);
            bmp.UnlockBits(data);
            return bmp;
        }
    }

    /// A button drawn from a horizontal strip of equally wide frames
    /// (normal, hover, pressed, disabled). Without a strip it draws itself:
    /// a flat amber-on-dark button with the caption, same size and behaviour.
    class ImageButton : Control
    {
        readonly Bitmap strip;
        readonly int frames;
        bool hover, down;

        public ImageButton(Bitmap strip, int frames, float k, string caption, int plainW, int plainH)
        {
            this.strip = strip;
            this.frames = frames;
            Text = caption;
            SetStyle(ControlStyles.AllPaintingInWmPaint | ControlStyles.OptimizedDoubleBuffer | ControlStyles.UserPaint
                | ControlStyles.SupportsTransparentBackColor | ControlStyles.Selectable, true);
            BackColor = Color.Transparent;
            Size = strip != null
                ? new Size((int)Math.Round((float)strip.Width / frames * k), (int)Math.Round(strip.Height * k))
                : new Size((int)Math.Round(plainW * k), (int)Math.Round(plainH * k));
            Cursor = Cursors.Hand;
            TabStop = true;
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            int idx = !Enabled ? 3 : down ? 2 : hover ? 1 : 0;
            if (strip == null)
            {
                var back = !Enabled ? Skin.PanelBack : down ? Color.FromArgb(0x5A, 0x40, 0x14) : hover ? Color.FromArgb(0x3A, 0x31, 0x22) : Color.FromArgb(0x24, 0x21, 0x1B);
                using (var b = new SolidBrush(back)) e.Graphics.FillRectangle(b, ClientRectangle);
                using (var p = new Pen(Enabled ? Skin.Amber : Skin.FieldBorder)) e.Graphics.DrawRectangle(p, 0, 0, Width - 1, Height - 1);
                TextRenderer.DrawText(e.Graphics, Text, Font, ClientRectangle, Enabled ? Skin.Text : Skin.Muted,
                    TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter);
                if (Focused && Enabled)
                    using (var pen = new Pen(Color.FromArgb(120, Skin.Amber))) e.Graphics.DrawRectangle(pen, 2, 2, Width - 5, Height - 5);
                return;
            }
            int fw = strip.Width / frames;
            if (idx >= frames) idx = 0;
            e.Graphics.InterpolationMode = InterpolationMode.HighQualityBicubic;
            e.Graphics.PixelOffsetMode = PixelOffsetMode.Half;
            e.Graphics.DrawImage(strip, new Rectangle(0, 0, Width, Height), new Rectangle(idx * fw, 0, fw, strip.Height), GraphicsUnit.Pixel);
            if (Focused && Enabled)
                using (var pen = new Pen(Color.FromArgb(90, Skin.Amber))) e.Graphics.DrawRectangle(pen, 1, 1, Width - 3, Height - 3);
        }

        protected override void OnMouseEnter(EventArgs e) { hover = true; Invalidate(); base.OnMouseEnter(e); }
        protected override void OnMouseLeave(EventArgs e) { hover = false; down = false; Invalidate(); base.OnMouseLeave(e); }
        protected override void OnMouseDown(MouseEventArgs e) { if (e.Button == MouseButtons.Left) { down = true; Focus(); Invalidate(); } base.OnMouseDown(e); }
        protected override void OnMouseUp(MouseEventArgs e)
        {
            bool fire = down && e.Button == MouseButtons.Left && ClientRectangle.Contains(e.Location);
            down = false; Invalidate();
            base.OnMouseUp(e);
            if (fire) OnClick(EventArgs.Empty);
        }
        protected override void OnKeyDown(KeyEventArgs e)
        {
            if (e.KeyCode == Keys.Space || e.KeyCode == Keys.Enter) { OnClick(EventArgs.Empty); e.Handled = true; }
            base.OnKeyDown(e);
        }
        protected override void OnEnabledChanged(EventArgs e) { Invalidate(); base.OnEnabledChanged(e); }
        protected override void OnGotFocus(EventArgs e) { Invalidate(); base.OnGotFocus(e); }
        protected override void OnLostFocus(EventArgs e) { Invalidate(); base.OnLostFocus(e); }
    }

    /// A check box drawn from the Check / Uncheck strips, or from scratch
    /// when this build has no skin.
    class SkinCheckBox : Control
    {
        readonly Bitmap on, off;
        readonly float k;
        bool hover;
        public bool Checked;

        public SkinCheckBox(Bitmap on, Bitmap off, float k)
        {
            this.on = on; this.off = off; this.k = k;
            SetStyle(ControlStyles.AllPaintingInWmPaint | ControlStyles.OptimizedDoubleBuffer | ControlStyles.UserPaint | ControlStyles.SupportsTransparentBackColor, true);
            BackColor = Color.Transparent;
            Cursor = Cursors.Hand;
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            var strip = Checked ? on : off;
            int box;
            if (strip == null)
            {
                box = (int)Math.Round(14 * k);
                var r = new Rectangle(0, (Height - box) / 2, box, box);
                using (var b = new SolidBrush(Skin.FieldBack)) e.Graphics.FillRectangle(b, r);
                using (var p = new Pen(hover && Enabled ? Skin.Amber : Skin.FieldBorder)) e.Graphics.DrawRectangle(p, r);
                if (Checked)
                    using (var p = new Pen(Enabled ? Skin.Amber : Skin.Muted, Math.Max(1.5f, 2f * k)))
                    {
                        e.Graphics.SmoothingMode = SmoothingMode.AntiAlias;
                        e.Graphics.DrawLines(p, new[] {
                            new Point(r.Left + box / 5, r.Top + box / 2),
                            new Point(r.Left + box * 2 / 5, r.Bottom - box / 4),
                            new Point(r.Right - box / 6, r.Top + box / 5) });
                    }
            }
            else
            {
                int fw = strip.Width / 4;
                box = (int)Math.Round(fw * k);
                int idx = !Enabled ? 3 : hover ? 1 : 0;
                e.Graphics.InterpolationMode = InterpolationMode.HighQualityBicubic;
                e.Graphics.DrawImage(strip, new Rectangle(0, (Height - box) / 2, box, box), new Rectangle(idx * fw, 0, fw, strip.Height), GraphicsUnit.Pixel);
            }
            TextRenderer.DrawText(e.Graphics, Text, Font, new Rectangle(box + (int)Math.Round(6 * k), 0, Width - box, Height), Enabled ? Skin.Text : Skin.Muted,
                TextFormatFlags.VerticalCenter | TextFormatFlags.Left | TextFormatFlags.NoPadding);
        }

        protected override void OnMouseEnter(EventArgs e) { hover = true; Invalidate(); base.OnMouseEnter(e); }
        protected override void OnMouseLeave(EventArgs e) { hover = false; Invalidate(); base.OnMouseLeave(e); }
        protected override void OnClick(EventArgs e) { Checked = !Checked; Invalidate(); base.OnClick(e); }
    }

    /// A small flat text button in the skin's colours (browse, check).
    class FlatButton : Control
    {
        bool hover, down;

        public FlatButton(string text)
        {
            Text = text;
            SetStyle(ControlStyles.AllPaintingInWmPaint | ControlStyles.OptimizedDoubleBuffer | ControlStyles.UserPaint | ControlStyles.Selectable, true);
            Cursor = Cursors.Hand;
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            var g = e.Graphics;
            var back = !Enabled ? Skin.PanelBack : down ? Color.FromArgb(0x4A, 0x36, 0x12) : hover ? Color.FromArgb(0x2C, 0x27, 0x1E) : Color.FromArgb(0x1E, 0x1C, 0x18);
            var border = Enabled && hover ? Skin.Amber : Skin.FieldBorder;
            using (var b = new SolidBrush(back)) g.FillRectangle(b, ClientRectangle);
            using (var p = new Pen(border)) g.DrawRectangle(p, 0, 0, Width - 1, Height - 1);
            TextRenderer.DrawText(g, Text, Font, ClientRectangle, Enabled ? (hover ? Skin.Amber : Skin.Text) : Skin.Muted,
                TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter);
        }

        protected override void OnMouseEnter(EventArgs e) { hover = true; Invalidate(); base.OnMouseEnter(e); }
        protected override void OnMouseLeave(EventArgs e) { hover = false; down = false; Invalidate(); base.OnMouseLeave(e); }
        protected override void OnMouseDown(MouseEventArgs e) { down = true; Invalidate(); base.OnMouseDown(e); }
        protected override void OnMouseUp(MouseEventArgs e) { down = false; Invalidate(); base.OnMouseUp(e); }
        protected override void OnEnabledChanged(EventArgs e) { Invalidate(); base.OnEnabledChanged(e); }
    }

    // -----------------------------------------------------------------------
    // the window

    class MainForm : Form
    {
        readonly Settings settings = Settings.Load();
        readonly float k;
        int S(int v) { return (int)Math.Round(v * k); }
        Rectangle R(int x, int y, int w, int h) { return new Rectangle(S(x), S(y), S(w), S(h)); }

        Bitmap background, banner, barTotal;
        Color keyColor;
        readonly TextBox server = new TextBox();
        readonly TextBox gameDir = new TextBox();
        readonly TextBox user = new TextBox();
        readonly TextBox pass = new TextBox { UseSystemPasswordChar = true };
        readonly TextBox status = new TextBox { Multiline = true, ReadOnly = true, ScrollBars = ScrollBars.Vertical };
        ImageButton start, login, close, minimise;
        SkinCheckBox remember, renderer, tweaks;
        FlatButton browse, check, restore;
        bool gameMissing;
        readonly Font labelFont, smallFont, fieldFont;
        bool busy;
        bool apiOnline;
        bool loggedIn;
        int progress;
        string stage = "";

        public MainForm()
        {
            using (var g = CreateGraphics()) k = g.DpiX / 96f;

            Text = "Arctic Combat - ACEmu launcher";
            FormBorderStyle = FormBorderStyle.None;
            StartPosition = FormStartPosition.CenterScreen;
            MaximizeBox = false;
            AutoScaleMode = AutoScaleMode.None;
            DoubleBuffered = true;
            Font = new Font("Segoe UI", 9.75f);
            labelFont = new Font("Segoe UI", 8f, FontStyle.Bold);
            smallFont = new Font("Segoe UI", 8f);
            fieldFont = new Font("Segoe UI", 10f);
            var icon = Skin.LoadIcon();
            if (icon != null) Icon = icon;

            var main = Skin.Load("main.png");
            if (main != null)
            {
                background = Skin.Background(main, k, out keyColor);
                BackColor = keyColor;
                TransparencyKey = keyColor;   // the artwork overhangs the window
            }
            else
            {
                background = Skin.PlainBackground(k);
                keyColor = Color.Empty;
                BackColor = Color.FromArgb(0x10, 0x0F, 0x0D);
            }
            banner = Skin.Load("web_launcher.png");
            barTotal = Skin.Load("total_progress.png");
            ClientSize = background.Size;

            // title bar buttons
            minimise = new ImageButton(Skin.Load("min.png"), 4, k, "_", 20, 18) { Location = new Point(S(846), S(129)), TabStop = false };
            close = new ImageButton(Skin.Load("exit.png"), 4, k, "X", 20, 18) { Location = new Point(S(868), S(129)), TabStop = false };
            minimise.Click += delegate { WindowState = FormWindowState.Minimized; };
            close.Click += delegate { Close(); };

            // top right: server
            Field(server, R(585, 180, 205, 24));
            // right panel: account
            Field(user, R(585, 256, 295, 26));
            Field(pass, R(585, 308, 295, 26));
            var on = Skin.Load("Check.png");
            var off = Skin.Load("Uncheck.png");
            // options, two columns
            remember = new SkinCheckBox(on, off, k) { Text = "Remember account", Bounds = R(585, 340, 150, 20), Font = smallFont, Checked = settings.Remember };
            renderer = new SkinCheckBox(on, off, k) { Text = "Modern renderer", Bounds = R(740, 340, 145, 20), Font = smallFont, Checked = settings.ModernRenderer };
            tweaks = new SkinCheckBox(on, off, k) { Text = "Performance tweaks", Bounds = R(585, 362, 150, 20), Font = smallFont, Checked = settings.PerfTweaks };
            // LOG IN under the options; START bottom right next to the progress
            // bar, enabled only once the login went through
            login = new ImageButton(Skin.Load("login.png"), 4, k, "LOG IN", 136, 40) { Location = new Point(S(666), S(404)), Font = labelFont };
            start = new ImageButton(Skin.Load("gamestart.png"), 4, k, "START", 160, 46) { Location = new Point(S(720), S(528)), Enabled = false, Font = labelFont };
            login.Click += delegate { Login(); };
            start.Click += delegate { Play(); };

            // left panel: log
            status.Bounds = R(16, 368, 549, 146);
            status.BorderStyle = BorderStyle.None;
            status.BackColor = Skin.FieldBack;
            status.ForeColor = Skin.LogText;
            status.Font = new Font("Consolas", 9f);
            status.TabStop = false;

            // bottom: game folder, check, restore, progress (left of START)
            Field(gameDir, R(16, 556, 454, 26));
            browse = new FlatButton("...") { Bounds = R(478, 556, 40, 26), Font = fieldFont };
            check = new FlatButton("CHECK") { Bounds = R(526, 556, 84, 26), Font = labelFont };
            restore = new FlatButton("RESTORE") { Bounds = R(618, 556, 86, 26), Font = labelFont };
            browse.Click += delegate
            {
                using (var d = new FolderBrowserDialog { Description = "Arctic Combat folder (the one with System\\ArcticCombat.exe)", SelectedPath = gameDir.Text })
                    if (d.ShowDialog(this) == DialogResult.OK) { gameDir.Text = d.SelectedPath; RunChecks(); }
            };
            check.Click += delegate { RunChecks(); };
            restore.Click += delegate { RestoreOriginals(); };

            Controls.AddRange(new Control[] { minimise, close, server, user, pass, remember, renderer, tweaks, login, start, status, gameDir, browse, check, restore });

            server.Text = settings.Server;
            gameDir.Text = settings.GameDir;
            user.Text = settings.Username;
            server.Leave += delegate { RunChecks(); };
            // a changed account, password or server needs a fresh login
            user.TextChanged += delegate { SetLoggedIn(false); };
            pass.TextChanged += delegate { SetLoggedIn(false); };
            server.TextChanged += delegate { SetLoggedIn(false); };
            pass.KeyDown += delegate(object o, KeyEventArgs e) { if (e.KeyCode == Keys.Enter) { e.SuppressKeyPress = true; if (loggedIn) Play(); else Login(); } };
            user.KeyDown += delegate(object o, KeyEventArgs e) { if (e.KeyCode == Keys.Enter) { e.SuppressKeyPress = true; pass.Focus(); } };
            Shown += delegate { (user.Text.Length == 0 ? user : pass).Focus(); RunChecks(true); };

            // keep the "server online" dot current without pressing CHECK
            var poll = new System.Windows.Forms.Timer { Interval = 5000 };
            poll.Tick += delegate { PollServer(); };
            poll.Start();
        }

        bool polling;

        void PollServer()
        {
            if (busy || polling) return;
            var s = Snapshot();
            if (s.Server.Length == 0) return;
            polling = true;
            Task.Run(delegate
            {
                bool on = false;
                try { on = new LoginApi(Checks.ResolveIp(s.Server), s.ApiPort).Ping(); }
                catch (Exception) { }
                SetOnline(on);
                polling = false;
            });
        }

        /// The GUID field of the launcher blob; nothing checks it, so it is
        /// simply derived from the account.
        static string GuidFor(string user) { return "acemu-" + user; }

        void Field(TextBox tb, Rectangle r)
        {
            tb.BorderStyle = BorderStyle.None;
            tb.BackColor = Skin.FieldBack;
            tb.ForeColor = Skin.Text;
            tb.Font = fieldFont;
            // the box is painted by the form; the text box sits inside it
            tb.Bounds = new Rectangle(r.X + S(6), r.Y + (r.Height - tb.PreferredHeight) / 2, r.Width - S(12), tb.PreferredHeight);
            tb.Tag = r;
        }

        // drag anywhere that is not a control (the art included)
        protected override void WndProc(ref Message m)
        {
            base.WndProc(ref m);
            const int WM_NCHITTEST = 0x84, HTCLIENT = 1, HTCAPTION = 2;
            if (m.Msg == WM_NCHITTEST && (int)m.Result == HTCLIENT) m.Result = (IntPtr)HTCAPTION;
        }

        protected override void OnPaintBackground(PaintEventArgs e)
        {
            var g = e.Graphics;
            g.DrawImageUnscaled(background, 0, 0);
            g.InterpolationMode = InterpolationMode.HighQualityBicubic;
            g.PixelOffsetMode = PixelOffsetMode.Half;

            // banner in the "web" area (skinned builds only)
            var bannerRect = R(9, 156, 562, 203);
            if (banner != null) g.DrawImage(banner, bannerRect);

            // panels behind the log and the fields
            using (var b = new SolidBrush(Skin.FieldBack))
            {
                g.FillRectangle(b, R(14, 366, 553, 150));
                foreach (var tb in new[] { server, user, pass, gameDir }) g.FillRectangle(b, (Rectangle)tb.Tag);
            }
            using (var p = new Pen(Skin.FieldBorder))
            {
                g.DrawRectangle(p, R(14, 366, 553, 150));
                foreach (var tb in new[] { server, user, pass, gameDir })
                {
                    var r = (Rectangle)tb.Tag;
                    g.DrawRectangle(p, r.X, r.Y, r.Width - 1, r.Height - 1);
                }
                // progress track, as wide as the game folder row above it
                var track = R(16, 600, 688, 6);
                g.DrawRectangle(p, track.X - 1, track.Y - 1, track.Width + 1, track.Height + 1);
            }

            // labels
            Label(g, "ARCTIC COMBAT", 36, 131, Skin.Muted, smallFont);
            Label(g, "ACEmu launcher " + Assembly.GetExecutingAssembly().GetName().Version.ToString(2), 130, 131, Skin.Muted, smallFont);
            Label(g, "SERVER", 585, 163, Skin.Amber, labelFont);
            Label(g, "USERNAME", 585, 240, Skin.Amber, labelFont);
            Label(g, "PASSWORD", 585, 292, Skin.Amber, labelFont);
            Label(g, "GAME FOLDER", 16, 538, Skin.Amber, labelFont);

            // admin service indicator next to the server field
            using (var b = new SolidBrush(apiOnline ? Skin.Good : Skin.Bad))
                g.FillEllipse(b, R(801, 188, 8, 8));
            Label(g, apiOnline ? "server online" : "server offline", 814, 184, apiOnline ? Skin.Text : Skin.Muted, smallFont);
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            var g = e.Graphics;
            g.InterpolationMode = InterpolationMode.HighQualityBicubic;
            var track = R(16, 600, 688, 6);
            if (progress > 0)
            {
                int w = (int)Math.Round(track.Width * Math.Min(progress, 100) / 100.0);
                if (barTotal != null)
                    g.DrawImage(barTotal, new Rectangle(track.X, track.Y, w, track.Height),
                        new Rectangle(0, 0, (int)Math.Round((float)barTotal.Width * w / track.Width), barTotal.Height), GraphicsUnit.Pixel);
                else
                    using (var b = new SolidBrush(Skin.Amber)) g.FillRectangle(b, track.X, track.Y, w, track.Height);
            }
            Label(g, stage, 16, 613, Skin.Muted, smallFont);
        }

        void Label(Graphics g, string text, int x, int y, Color color, Font font)
        {
            TextRenderer.DrawText(g, text, font, new Point(S(x), S(y)), color, TextFormatFlags.NoPadding);
        }

        // -------------------------------------------------------------------

        void Log(string line)
        {
            if (InvokeRequired) { BeginInvoke(new Action<string>(Log), line); return; }
            status.AppendText(line + Environment.NewLine);
        }

        void Stage(int pct, string text)
        {
            if (InvokeRequired) { BeginInvoke(new Action<int, string>(Stage), pct, text); return; }
            progress = pct;
            stage = text;
            Invalidate(R(0, 596, 712, 40));
        }

        void SetBusy(bool b)
        {
            if (InvokeRequired) { BeginInvoke(new Action<bool>(SetBusy), b); return; }
            busy = b;
            check.Enabled = restore.Enabled = browse.Enabled = login.Enabled = !b;
            start.Enabled = !b && loggedIn;
            UseWaitCursor = b;
        }

        /// START is live only after a successful LOG IN with the current
        /// account, password and server.
        void SetLoggedIn(bool v)
        {
            if (InvokeRequired) { BeginInvoke(new Action<bool>(SetLoggedIn), v); return; }
            loggedIn = v;
            start.Enabled = !busy && v;
        }

        /// Hides RESTORE while the folder holds no client.
        void SetGameMissing(bool missing)
        {
            if (InvokeRequired) { BeginInvoke(new Action<bool>(SetGameMissing), missing); return; }
            gameMissing = missing;
            restore.Visible = !missing;
        }

        void SetOnline(bool on)
        {
            if (InvokeRequired) { BeginInvoke(new Action<bool>(SetOnline), on); return; }
            apiOnline = on;
            Invalidate(R(795, 180, 100, 24));
        }

        Settings Snapshot()
        {
            settings.Server = server.Text.Trim();
            settings.GameDir = gameDir.Text.Trim().TrimEnd('\\');
            settings.Username = user.Text.Trim();
            settings.Remember = remember.Checked;
            settings.ModernRenderer = renderer.Checked;
            settings.PerfTweaks = tweaks.Checked;
            return settings;
        }

        /// Status check. With `fix`, the client files are put right first
        /// (missing patches, stale Test.ini) - the things that need no
        /// elevation and no credentials; only actual changes are logged.
        void RunChecks(bool fix)
        {
            if (busy) return;
            var s = Snapshot();
            if (s.Server.Length == 0) return;
            SetBusy(true);
            status.Clear();
            Stage(0, "");
            Task.Run(delegate
            {
                try
                {
                    bool present = File.Exists(Path.Combine(s.GameDir, @"System\ArcticCombat.exe"));
                    SetGameMissing(!present);
                    if (!present)
                    {
                        Log(s.GameDir.Length == 0
                            ? "no Arctic Combat install found - pick the folder with System\\ArcticCombat.exe using ..."
                            : "no game in " + s.GameDir + (s.GameDirSource.Length > 0 ? "  [" + s.GameDirSource + "]" : "")
                              + " - install the game first (Steam app 212370), then pick its folder with ...");
                        Log("");
                    }
                    if (fix && present)
                    {
                        try
                        {
                            Checks.Prepare(s, Checks.ResolveIp(s.Server), delegate(string line)
                            {
                                if (!line.Contains("already applied") && !line.Contains("points at")) Log("auto-fix: " + line);
                            });
                        }
                        catch (Exception e) { Log("auto-fix: " + e.Message); }
                    }
                    SetOnline(Checks.Report(s, Log));
                }
                catch (Exception e) { Log("check failed: " + e.Message); }
                finally { SetBusy(false); }
            });
        }

        void RunChecks() { RunChecks(false); }

        /// RESTORE: the original client files from the .orig backups, no
        /// Test.ini, and (after the UAC prompt) no hosts lines. The next
        /// launcher start patches again; this is for playing the retail
        /// client elsewhere or for verifying the files through Steam.
        void RestoreOriginals()
        {
            if (busy) return;
            var s = Snapshot();
            if (!File.Exists(Path.Combine(s.GameDir, @"System\ArcticCombat.exe"))) { status.Clear(); Log("not found: " + Path.Combine(s.GameDir, @"System\ArcticCombat.exe")); return; }
            var ok = MessageBox.Show(this,
                "Put the original client files back?\n\n" +
                "ArcticCombat.exe, BatteryNet.dll and WinDrv.dll are restored from their .orig backups, " +
                "System\\Test.ini is removed, and the login names are taken out of the hosts file (administrator prompt).\n\n" +
                "The launcher patches the client again the next time it starts.",
                "ACEmu launcher - restore", MessageBoxButtons.YesNo, MessageBoxIcon.Question);
            if (ok != DialogResult.Yes) return;
            SetBusy(true);
            status.Clear();
            Stage(0, "");
            Task.Run(delegate
            {
                try
                {
                    Stage(30, "Restoring the client files ...");
                    Checks.Restore(s, Log);
                    if (Renderer.IsApplied(s.GameDir)) Log("restore renderer: " + Renderer.Remove(s.GameDir) + " file(s) removed from System\\");
                    if (IniTweaks.IsApplied(s.GameDir)) Log("restore ini: " + IniTweaks.Remove(s.GameDir) + " tweak section(s) removed");
                    BeginInvoke(new Action(delegate { renderer.Checked = tweaks.Checked = false; renderer.Invalidate(); tweaks.Invalidate(); }));
                    Stage(60, "Cleaning the hosts file ...");
                    if (!Hosts.AnyMapped()) Log("restore hosts: no login names mapped");
                    else if (Hosts.FixElevated(null)) Log("restore hosts: login names removed");
                    else Log("restore hosts: NOT changed (declined or failed)");
                    Stage(100, "Original files restored");
                    Log("");
                    SetOnline(Checks.Report(s, Log));
                }
                catch (Exception e) { Log("FAILED: " + e.Message); Stage(0, "Restore failed"); }
                finally { SetBusy(false); }
            });
        }

        static readonly Regex NameRx = new Regex(@"^[A-Za-z0-9_\-\.]{1,32}$");
        static readonly Regex PassRx = new Regex(@"^[\x21-\x7B\x7D\x7E]{0,64}$"); // printable ASCII, no space, no '|'

        /// Validates the form; null when something is wrong (already logged).
        Settings Validate(out string password, out string exe)
        {
            var s = Snapshot();
            password = pass.Text;
            exe = Path.Combine(s.GameDir, @"System\ArcticCombat.exe");
            status.Clear();
            if (!NameRx.IsMatch(s.Username)) { Log("account name: letters, digits, '_' '-' '.' only (1-32)"); user.Focus(); return null; }
            if (!PassRx.IsMatch(password)) { Log("password: printable ASCII without spaces or '|' (max 64)"); pass.Focus(); return null; }
            if (s.Server.Length == 0) { Log("server is empty"); server.Focus(); return null; }
            if (s.GameDir.Length == 0) { Log("pick the folder of your Arctic Combat install with ..."); return null; }
            s.Save();
            return s;
        }

        /// The game itself is not distributed by this project: the folder has
        /// to hold a copy the player owns (Steam app 212370). Throws when it
        /// does not, with the Steam install link as the way out.
        void RequireClient(Settings s)
        {
            if (File.Exists(Path.Combine(s.GameDir, @"System\ArcticCombat.exe"))) return;
            Log("no game in " + (s.GameDir.Length == 0 ? "(no folder picked)" : s.GameDir));
            var steam = (DialogResult)Invoke(new Func<DialogResult>(() => MessageBox.Show(this,
                "No Arctic Combat install in that folder.\n\n" +
                "This project ships no game files. Install the game yourself (Steam app 212370, " +
                "https://steamdb.info/app/212370/) and point the game folder at it.\n\n" +
                "Open Steam now? It works only for accounts that already own the delisted game.",
                "ACEmu launcher - no game", MessageBoxButtons.YesNo, MessageBoxIcon.Question)));
            if (steam == DialogResult.Yes)
            {
                Process.Start(new ProcessStartInfo(GameLocator.SteamInstallUrl) { UseShellExecute = true });
                Log("Steam opened - once installed, pick that folder with ...");
            }
            throw new LoginException("no client in " + s.GameDir);
        }

        /// The optional client extras, each installed when its box is ticked
        /// and taken out again when it is not (the take-out works offline).
        /// Skipped while the game runs, because the files are open then.
        void EnsureOptions(Settings s, LoginApi api)
        {
            bool anyOn = s.ModernRenderer || s.PerfTweaks;
            bool anyApplied = Renderer.IsApplied(s.GameDir) || IniTweaks.IsApplied(s.GameDir);
            if (!anyOn && !anyApplied) return;
            if (Process.GetProcessesByName("ArcticCombat").Length > 0)
            {
                Log("options: the game is running - renderer and ini files left as they are");
                return;
            }
            // renderer 73..78
            if (s.ModernRenderer)
            {
                Stage(73, "Modern renderer: asking the server ...");
                Log(Renderer.Apply(api, s.GameDir, delegate(int pct, string text) { Stage(73 + pct * 5 / 100, text); }, Log));
            }
            else if (Renderer.IsApplied(s.GameDir))
            {
                Log("Modern renderer: off - " + Renderer.Remove(s.GameDir) + " file(s) removed from System\\");
            }
            // tweaks 79
            Stage(79, "Checking the ini tweaks ...");
            if (s.PerfTweaks) Log(IniTweaks.Apply(s.GameDir));
            else if (IniTweaks.IsApplied(s.GameDir)) Log("Performance tweaks: off - " + IniTweaks.Remove(s.GameDir) + " section(s) removed from ArcticCombat.ini");
        }

        /// LOG IN: only the account step (creates / verifies), no game start.
        void Login()
        {
            if (busy) return;
            string password, exe;
            var s = Validate(out password, out exe);
            if (s == null) return;
            SetBusy(true);
            Stage(5, "Asking the login server ...");
            Task.Run(delegate
            {
                try
                {
                    var ip = Checks.ResolveIp(s.Server);
                    var api = new LoginApi(ip, s.ApiPort);
                    Log(api.EnsureAccount(s.Username, password));
                    SetOnline(true);
                    RequireClient(s);
                    Stage(52, "Checking the client files ...");
                    Checks.Prepare(s, ip, Log);
                    EnsureOptions(s, api);
                    SetLoggedIn(true);
                    Stage(100, "Logged in - press START");
                }
                catch (Exception e) { Log("FAILED: " + e.Message); Stage(0, "Login failed"); SetLoggedIn(false); }
                finally { SetBusy(false); }
            });
        }

        void Play()
        {
            if (busy) return;
            string password, exe;
            var s = Validate(out password, out exe);
            if (s == null) return;
            SetBusy(true);
            Task.Run(delegate
            {
                try { Launch(s, password, exe); }
                catch (Exception e) { Log("FAILED: " + e.Message); Stage(0, "Start failed"); }
                finally { SetBusy(false); }
            });
        }

        void Launch(Settings s, string password, string exe)
        {
            var ip = Checks.ResolveIp(s.Server);
            Log("server " + s.Server + (ip != s.Server ? " (" + ip + ")" : ""));

            // 1. account: the login server creates unknown ones, refuses wrong passwords
            Stage(5, "Asking the login server ...");
            var api = new LoginApi(ip, s.ApiPort);
            Log(api.EnsureAccount(s.Username, password));
            SetOnline(true);

            // 2. the player's own install has to be there
            RequireClient(s);

            // 3. client patches and Test.ini
            Stage(52, "Checking the client files ...");
            Checks.Prepare(s, ip, Log);

            // 3b. renderer wrapper and ini tweaks - on or off
            EnsureOptions(s, api);

            // 4. hosts (best effort - Test.ini is what the client actually uses)
            Stage(82, "Checking the hosts file ...");
            var missing = Hosts.Missing(ip);
            if (missing.Length == 0) Log("hosts: login names already -> " + ip);
            else
            {
                Log("hosts: mapping " + string.Join(", ", missing) + " -> " + ip + " (administrator prompt)");
                if (Hosts.FixElevated(ip)) Log("hosts: updated");
                else Log("hosts: NOT updated (declined or failed) - continuing, Test.ini is enough for the login redirect");
            }

            // 5. a client already running would make the new one exit at once
            Stage(85, "Starting the game ...");
            var running = Process.GetProcessesByName("ArcticCombat");
            if (running.Length > 0)
            {
                var ok = (DialogResult)Invoke(new Func<DialogResult>(() => MessageBox.Show(this,
                    "An Arctic Combat client is already running (pid " + running[0].Id + ").\n\n" +
                    "Close its single-instance mutex so a second client can start?\n" +
                    "(Use a separate game folder such as ArcticCombat2 for the second client.)",
                    "ACEmu launcher", MessageBoxButtons.YesNo, MessageBoxIcon.Question)));
                if (ok != DialogResult.Yes) throw new Exception("not starting a second client");
                int n = 0;
                foreach (var p in running) n += MutexCloser.Close(p.Id, MutexCloser.MutexName);
                Log("closed " + n + " mutex handle(s) in the running client");
            }

            // 6. go
            var blob = Blob.Encode(s.Username, GuidFor(s.Username), password, s.Region, s.Language, "0", "0");
            var psi = new ProcessStartInfo(exe, blob) { WorkingDirectory = Path.GetDirectoryName(exe), UseShellExecute = false };
            var game = Process.Start(psi);
            Log("started ArcticCombat.exe (pid " + game.Id + ")");
            Stage(95, "Game starting ...");
            System.Threading.Thread.Sleep(3000);
            if (game.HasExited)
            {
                Log("the client exited after 3 s with code " + game.ExitCode + " - see System\\ArcticCombat.log");
                Stage(0, "The client exited");
            }
            else
            {
                Log("client is running");
                Stage(100, "Game running");
            }
        }
    }
}
