<#
.SYNOPSIS
  Builds launcher\AcLauncher.exe with the C# compiler that ships with the
  .NET Framework - no SDK, no NuGet, no project file.

.DESCRIPTION
  The launcher draws its own window, so it needs no artwork to run. If a
  skin\ folder is present, its PNGs are embedded and used instead; nothing
  of the sort ships with this project. Owners of the game who want the
  original look can export the retail launcher's bitmaps themselves into
  skin\ using these names, all optional:

      main.png gamestart.png login.png exit.png min.png web_launcher.png
      Check.png Uncheck.png total_progress.png file_progress.png icon.ico

  Keep the source C# 5 compatible (no string interpolation, no ?., no nameof).

.EXAMPLE
  .\launcher\build.ps1
#>
[CmdletBinding()]
param()

$skin = Join-Path $PSScriptRoot 'skin'
$images = 'main', 'gamestart', 'login', 'exit', 'min', 'web_launcher', 'Check', 'Uncheck', 'total_progress', 'file_progress'

$fw = Join-Path $env:WINDIR 'Microsoft.NET\Framework64\v4.0.30319'
if (-not (Test-Path (Join-Path $fw 'csc.exe'))) { $fw = Join-Path $env:WINDIR 'Microsoft.NET\Framework\v4.0.30319' }
$csc = Join-Path $fw 'csc.exe'
if (-not (Test-Path $csc)) { throw "csc.exe not found under $env:WINDIR\Microsoft.NET - install the .NET Framework 4.x" }

$res = @()
$icon = @()
foreach ($n in $images) {
  $p = Join-Path $skin "$n.png"
  if (Test-Path $p) { $res += "/resource:$p,skin.$n.png" }
}
$icoPath = Join-Path $skin 'icon.ico'
if (Test-Path $icoPath) {
  $res += "/resource:$icoPath,skin.icon.ico"
  $icon += "/win32icon:$icoPath"
}
if ($res.Count -eq 0) { "no skin\ images found - building the plain launcher" }
else { "embedding $($res.Count) skin file(s) from $skin" }

$out = Join-Path $PSScriptRoot 'AcLauncher.exe'
& $csc /nologo /target:winexe /optimize+ /platform:anycpu32bitpreferred `
  /out:"$out" `
  /win32manifest:"$PSScriptRoot\app.manifest" `
  @icon `
  /r:System.Windows.Forms.dll /r:System.Drawing.dll /r:System.Web.Extensions.dll `
  @res `
  "$PSScriptRoot\AcLauncher.cs"
if ($LASTEXITCODE -ne 0) { throw "csc failed ($LASTEXITCODE)" }
"built $out ($((Get-Item $out).Length) bytes)"
