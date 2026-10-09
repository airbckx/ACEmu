<#
.SYNOPSIS
  Closes a named mutex handle inside a running process, so a second copy of a
  program with a single-instance guard can start on the same machine.

.DESCRIPTION
  ArcticCombat.exe's WinMain (RVA 0x138xx) does
      CreateMutexW(NULL, TRUE, <name>); if (GetLastError()) { FindWindowW(...);
      bring it to front; return 0 }  -> exit code 1
  so a second client on the same PC exits at once. For local two-client tests
  we do not patch the binary; we close the first instance's mutex handle instead
  (DuplicateHandle with DUPLICATE_CLOSE_SOURCE), after which the next launch
  creates the mutex afresh. Run it again before every additional instance.

  Handles are enumerated with NtQuerySystemInformation(SystemExtendedHandleInformation)
  and only "Mutant" objects are name-queried, which is safe (querying pipe names
  can hang).

.EXAMPLE
  .\tools\ac-close-mutex.ps1 -ProcessName ArcticCombat -NameLike 'ArcticCombat'
  .\tools\ac-close-mutex.ps1 -ProcessId 25988 -NameLike 'Some\Mutex\Name' -List
#>
[CmdletBinding()]
param(
  [int]$ProcessId,
  [string]$ProcessName = 'ArcticCombat',
  [Parameter(Mandatory = $true)][string]$NameLike,
  [switch]$List
)

if (-not ('AcHandles' -as [type])) {
  Add-Type -Language CSharp -TypeDefinition @"
using System; using System.Runtime.InteropServices; using System.Collections.Generic; using System.Text;
public class AcHandles {
  [StructLayout(LayoutKind.Sequential)]
  public struct SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX {
    public IntPtr Object; public IntPtr UniqueProcessId; public IntPtr HandleValue;
    public uint GrantedAccess; public ushort CreatorBackTraceIndex; public ushort ObjectTypeIndex;
    public uint HandleAttributes; public uint Reserved;
  }
  [DllImport("ntdll.dll")] static extern int NtQuerySystemInformation(int cls, IntPtr buf, int len, out int retLen);
  [DllImport("ntdll.dll")] static extern int NtQueryObject(IntPtr h, int cls, IntPtr buf, int len, out int retLen);
  [DllImport("kernel32.dll", SetLastError=true)] static extern IntPtr OpenProcess(uint access, bool inherit, int pid);
  [DllImport("kernel32.dll", SetLastError=true)] static extern bool DuplicateHandle(IntPtr srcProc, IntPtr srcH, IntPtr dstProc, out IntPtr dstH, uint access, bool inherit, uint options);
  [DllImport("kernel32.dll")] static extern IntPtr GetCurrentProcess();
  [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);

  const int SystemExtendedHandleInformation = 64;
  const int ObjectNameInformation = 1, ObjectTypeInformation = 2;
  const uint PROCESS_DUP_HANDLE = 0x40, DUPLICATE_CLOSE_SOURCE = 1, DUPLICATE_SAME_ACCESS = 2;

  static string QueryString(IntPtr h, int cls) {
    int len = 0x1000; IntPtr buf = Marshal.AllocHGlobal(len); int ret;
    try {
      int st = NtQueryObject(h, cls, buf, len, out ret);
      if (st != 0) { Marshal.FreeHGlobal(buf); buf = Marshal.AllocHGlobal(ret + 16); len = ret + 16; st = NtQueryObject(h, cls, buf, len, out ret); if (st != 0) return null; }
      // UNICODE_STRING { ushort Length; ushort Max; IntPtr Buffer; } at start of both infos
      ushort l = (ushort)Marshal.ReadInt16(buf, 0); IntPtr p = Marshal.ReadIntPtr(buf, IntPtr.Size == 8 ? 8 : 4);
      if (l == 0 || p == IntPtr.Zero) return "";
      return Marshal.PtrToStringUni(p, l / 2);
    } finally { Marshal.FreeHGlobal(buf); }
  }

  public static List<string> Run(int pid, string nameLike, bool close) {
    var outp = new List<string>();
    int len = 1 << 20; IntPtr buf = Marshal.AllocHGlobal(len); int ret;
    int st;
    while ((st = NtQuerySystemInformation(SystemExtendedHandleInformation, buf, len, out ret)) == unchecked((int)0xC0000004)) {
      Marshal.FreeHGlobal(buf); len = ret + (1 << 16); buf = Marshal.AllocHGlobal(len);
    }
    if (st != 0) { outp.Add("NtQuerySystemInformation failed: 0x" + st.ToString("X")); return outp; }
    IntPtr proc = OpenProcess(PROCESS_DUP_HANDLE, false, pid);
    if (proc == IntPtr.Zero) { outp.Add("OpenProcess failed: " + Marshal.GetLastWin32Error()); return outp; }
    long count = Marshal.ReadIntPtr(buf).ToInt64();
    int entrySize = Marshal.SizeOf(typeof(SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX));
    IntPtr first = IntPtr.Add(buf, IntPtr.Size * 2);
    for (long i = 0; i < count; i++) {
      var e = (SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX)Marshal.PtrToStructure(IntPtr.Add(first, (int)(i * entrySize)), typeof(SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX));
      if (e.UniqueProcessId.ToInt64() != pid) continue;
      IntPtr dup;
      if (!DuplicateHandle(proc, e.HandleValue, GetCurrentProcess(), out dup, 0, false, DUPLICATE_SAME_ACCESS)) continue;
      try {
        string type = QueryString(dup, ObjectTypeInformation);
        if (type != "Mutant") continue;
        string name = QueryString(dup, ObjectNameInformation);
        if (name == null) name = "";
        bool match = name.IndexOf(nameLike, StringComparison.OrdinalIgnoreCase) >= 0;
        outp.Add((match ? "MATCH " : "      ") + "0x" + e.HandleValue.ToInt64().ToString("X") + "  " + name);
        if (match && close) {
          IntPtr dup2;
          if (DuplicateHandle(proc, e.HandleValue, GetCurrentProcess(), out dup2, 0, false, DUPLICATE_CLOSE_SOURCE)) { CloseHandle(dup2); outp.Add("closed 0x" + e.HandleValue.ToInt64().ToString("X") + " in pid " + pid); }
          else outp.Add("close failed: " + Marshal.GetLastWin32Error());
        }
      } finally { CloseHandle(dup); }
    }
    CloseHandle(proc); Marshal.FreeHGlobal(buf);
    return outp;
  }
}
"@
}

if (-not $ProcessId) {
  $p = Get-Process -Name $ProcessName -ErrorAction SilentlyContinue | Sort-Object StartTime | Select-Object -First 1
  if (-not $p) { throw "no process named $ProcessName" }
  $ProcessId = $p.Id
}
Write-Host "process $ProcessId, looking for Mutant handles matching '$NameLike'"
[AcHandles]::Run($ProcessId, $NameLike, -not $List) | ForEach-Object { Write-Host "  $_" }
