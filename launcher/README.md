# The launcher

> Part of an educational project with no connection to WebZen; see the
> disclaimer in the top-level README. The launcher ships no game files and no
> artwork - it works on an install you already own.

A single-file Windows launcher that prepares a client install, makes sure your
account exists, and starts the game pointed at an ACEmu server. It replaces the
retail launcher, which cannot work: it talked to services that no longer exist,
and the client refuses to start without something standing in for it.

```powershell
.\launcher\build.ps1
```

That compiles `AcLauncher.cs` with the C# compiler that ships with the .NET
Framework (`csc.exe` from `%WINDIR%\Microsoft.NET\Framework\v4.0.30319`). No
SDK, no NuGet, no project file. The cost of that is the language level: the
source has to stay C# 5, so no string interpolation, no `?.`, no `nameof`.

Settings live in `launcher.ini` next to the exe and are written when you tick
*Remember*: game folder, server, username, region, language, the two ports, and
the two optional client tweaks.

## What it does to a client install

Everything below is reversible with the RESTORE button, which also turns the
optional tweaks off.

### 1. Three byte patches

Each one is verified before and after - the launcher reads the bytes it expects
and refuses to touch a file that holds anything else - and the original is kept
next to it as `<file>.orig`.

| name | file | why |
|---|---|---|
| `skip-hshield` | `System\ArcticCombat.exe` | NOPs the call into HackShield, which cannot reach its update server and so fails closed |
| `no-packet-crypto` | `System\BatteryNet.dll` | two `jz` → `jmp`, skipping the encrypt/decrypt wrappers, so the lobby protocol goes over the wire in plaintext |
| `no-web-browser` | `System\WinDrv.dll` | returns early from the embedded-browser call, which crashes on current Windows |

`no-packet-crypto` is the one that matters. ACEmu does not implement the
client's packet cipher, so an unpatched client cannot talk to it. This is the
only sense in which the client is "modified": three files, three known byte
sequences, backups kept, and `--restore` puts them all back.

### 2. `System\Test.ini`

The redirect. The client's main menu reads a login server list from this file
before falling back to its built-in list, so writing

```ini
[TEST]
SERVER_COUNT=1
SERVER_IP[0]=192.168.1.10
SERVER_PORT[0]=55000
IP=192.168.1.10
Port=55000
```

is enough to make it dial your machine. Both forms are written because the same
function reads either.

### 3. The hosts file

Some paths resolve the retail login host names directly instead of going
through `Test.ini`, so three names are mapped to the server's address in
`%WINDIR%\System32\drivers\etc\hosts`. This is the only step that needs
administrator rights, and the launcher asks for elevation only when a line is
actually missing - it re-launches itself as a helper that writes just those
lines and exits.

### 4. The account

`POST /ensure` against the login server's launcher endpoint (port 55001 by
default) with the username and password from the form. The reply says whether
the account was `ok`, `created`, `password_set`, `wrong` or `no_password`, and
the launcher reports that in its log. There is no registration page anywhere
else; this is it.

### 5. Starting the game

The client expects to be started by a launcher and checks the argument blob it
is given. The launcher builds one with the account, region and language, then
starts `ArcticCombat.exe`.

## Running two clients on one machine

Useful for testing, and the reason `tools/ac-close-mutex.ps1` exists. The game
holds a named mutex for single-instance detection and a second copy exits
immediately. The launcher closes that handle in every running client before
starting another, from inside the launcher process, using the same approach the
script takes.

## Optional tweaks

Two checkboxes, both off by default, both described in
`../docs/CLIENT-TWEAKS.md`:

- **Modern renderer** - installs a Direct3D 9 wrapper into `System\`. The
  wrapper is third-party software and is not in this repository; the login
  server serves it from an extras folder you populate yourself, and the option
  reports as unavailable if you have not.
- **Performance tweaks** - merges three engine ini sections into
  `System\ArcticCombat.ini`. Removing the tick removes exactly those sections.

## Headless use

For scripting and for checking a machine without clicking through the window.
Each mode writes its log to the optional report file as well as stdout.

```text
AcLauncher.exe --check [report.txt]
AcLauncher.exe --patch   <gamedir> [report.txt]
AcLauncher.exe --restore <gamedir> [report.txt]
AcLauncher.exe --renderer <gamedir> on|off [report.txt]
AcLauncher.exe --tweaks   <gamedir> on|off [report.txt]
```

`--check` is read-only: patch state per file, whether `Test.ini` and the hosts
entries are current, and whether the login and admin services answer.

## Appearance

The launcher paints itself: flat dark panels, drawn buttons, drawn checkboxes.
No artwork ships with this project.

If you own the game and would rather have the original look, put PNGs in
`launcher\skin\` before running `build.ps1` and they are embedded as resources
and used instead. The loaders all return `null` on a missing or unreadable
file, so a partial set works too - anything absent falls back to the drawn
version. `build.ps1` picks up `skin\*.png`, uses `skin\icon.ico` as the exe
icon if present, and otherwise prints `no skin\ images found - building the
plain launcher`. The folder is in `.gitignore`.
