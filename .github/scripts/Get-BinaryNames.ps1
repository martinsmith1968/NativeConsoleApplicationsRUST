<#
.SYNOPSIS
Lists the workspace's executables and publishes them as BINARY_NAMES (comma-separated).

.DESCRIPTION
Uses `cargo metadata`, so only real binary targets are included and library crates
such as build-support and test-support are skipped. Fails if none are found.

.EXAMPLE
./Get-BinaryNames.ps1 -WorkspaceRoot .
#>
param (
    [string] $WorkspaceRoot = '.'
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

$manifest = Join-Path $WorkspaceRoot 'Cargo.toml'
$metadata = cargo metadata --no-deps --format-version 1 --manifest-path $manifest | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) {
    Write-Error "cargo metadata failed for '$manifest'"
    exit 1
}

$binaries = @()
foreach ($package in $metadata.packages | Sort-Object name) {
    $bins = @($package.targets | Where-Object { $_.kind -contains 'bin' })
    if ($bins.Count -eq 0) {
        Write-Host " - $($package.name) (library, skipped)"
        continue
    }
    foreach ($bin in $bins) {
        Write-Host " - $($package.name): $($bin.name) (binary)"
        $binaries += $bin.name
    }
}

if ($binaries.Count -eq 0) {
    Write-Error 'No binary projects found in the workspace'
    exit 1
}

Set-PipelineValue 'BINARY_NAMES' ($binaries -join ',')
