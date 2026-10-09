<#
.SYNOPSIS
  Reads ArcticCombat.log while the game still has it open, handling its encoding.

.DESCRIPTION
  The game keeps the log open exclusively, so a normal read fails with a sharing
  violation; this opens with FileShare.ReadWrite. The log is UTF-16LE, which also
  makes plain Select-String miss everything.

  Enable logging first:
    HKCU\Software\Webzen\Battery  TempUseBatteryLog (DWORD) = 1

.EXAMPLE
  .\tools\ac-log.ps1 -Tail 40
  .\tools\ac-log.ps1 -Pattern 'BTNetMainMenu|LoginServer|Connect\('
#>
[CmdletBinding()]
param(
  [string]$Path = ".\ArcticCombat\System\ArcticCombat.log",
  [string]$Pattern,
  [int]$Tail = 0,
  [int]$Head = 0,
  [switch]$Raw
)

if (-not (Test-Path $Path)) { throw "no log at $Path (is TempUseBatteryLog set?)" }

$fs = [System.IO.File]::Open((Resolve-Path $Path).Path, [System.IO.FileMode]::Open,
                             [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
try {
  $b = New-Object byte[] $fs.Length
  $null = $fs.Read($b, 0, $b.Length)
} finally { $fs.Close() }

# UT2004-family logs are UTF-16LE, usually without a BOM
$enc =
  if ($b.Length -ge 2 -and $b[0] -eq 0xFF -and $b[1] -eq 0xFE) { [System.Text.Encoding]::Unicode }
  elseif ($b.Length -ge 2 -and $b[1] -eq 0x00)                 { [System.Text.Encoding]::Unicode }
  else                                                          { [System.Text.Encoding]::UTF8 }

$lines = ($enc.GetString($b) -replace "`0", '') -split "`r?`n"

if ($Raw)     { return $lines }
if ($Pattern) { $lines = @($lines | Where-Object { $_ -match $Pattern }) }
if ($Head)    { $lines = @($lines | Select-Object -First $Head) }
if ($Tail)    { $lines = @($lines | Select-Object -Last $Tail) }
$lines
