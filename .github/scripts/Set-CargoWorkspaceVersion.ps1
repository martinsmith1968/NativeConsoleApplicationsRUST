<#
.SYNOPSIS
Sets the version in the [workspace.package] section of Cargo.toml.

.EXAMPLE
./Set-CargoWorkspaceVersion.ps1 -Version 1.8.26304
#>
param (
    [Parameter(Mandatory)] [string] $Version,
    [string] $Path = 'Cargo.toml'
)

$ErrorActionPreference = 'Stop'

Write-Host "Setting workspace version in '$Path' to '$Version'"

$lines = Get-Content $Path

$section = ''
$found = $false
for ($i = 0; $i -lt $lines.Length; $i++) {
    $line = $lines[$i].Trim()

    if ($line -match '^\s*\[.*\]\s*$') {
        $section = $line.Trim('[', ']')
    }

    if ($section -eq 'workspace.package' -and $line -match '^\s*version\s*=\s*".*"\s*$') {
        Write-Host "Found version line: $($i): '$line'"
        $lines[$i] = "version    = `"$Version`""
        Write-Host "  Set version line: $($i): '$($lines[$i])'"
        $found = $true
        break
    }
}

if (-not $found) {
    Write-Error "No version found in [workspace.package] of $Path"
    exit 1
}

Set-Content -Path $Path -Value $lines

Write-Host
Write-Host "$Path :"
Get-Content $Path
