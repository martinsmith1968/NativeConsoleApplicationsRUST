<#
.SYNOPSIS
Copies the built executables and the README into a folder ready to be zipped.

.EXAMPLE
./Copy-ReleaseFiles.ps1 -BinaryNames 'bannertext,hashcalc' -SourceFolder target/release -Platform windows -Destination Collect
#>
param (
    [Parameter(Mandatory)] [string] $BinaryNames,
    [Parameter(Mandatory)] [string] $SourceFolder,
    [Parameter(Mandatory)] [ValidateSet('windows', 'linux')] [string] $Platform,
    [Parameter(Mandatory)] [string] $Destination,
    [string] $Readme = 'README.md'
)

$ErrorActionPreference = 'Stop'

$executable_extension = if ($Platform -eq 'windows') { '.exe' } else { '' }

if (-not (Test-Path $Destination -PathType Container)) {
    New-Item -Path $Destination -ItemType Directory | Out-Null
}

Copy-Item $Readme $Destination

foreach ($project in $BinaryNames.Split(',')) {
    if ([string]::IsNullOrWhiteSpace($project)) {
        continue
    }

    $source = Join-Path $SourceFolder "$($project.Trim())$executable_extension"
    Write-Host "Copying project to '$Destination': $source"
    Copy-Item $source $Destination
}
