# Shared helpers for the CI scripts in this folder.

<#
.SYNOPSIS
Publishes a name=value pair to later workflow steps (via GITHUB_ENV) and echoes it.
When run outside GitHub Actions it only echoes, so the scripts can be tried locally.
#>
function Set-PipelineValue {
    param (
        [Parameter(Mandatory)] [string] $Name,
        [AllowEmptyString()] [string] $Value
    )

    Write-Host "$Name=$Value"
    if ($env:GITHUB_ENV) {
        Add-Content -Path $env:GITHUB_ENV -Value "$Name=$Value"
    }
}

<#
.SYNOPSIS
Formats a boolean the way the workflow compares it ('true' / 'false').
#>
function ConvertTo-PipelineBool {
    param ([bool] $Value)
    if ($Value) { 'true' } else { 'false' }
}
