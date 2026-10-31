<#
.SYNOPSIS
Decides how this CI run is versioned, tagged and released.

.DESCRIPTION
Publishes is_primary_branch, is_pull_request_build, is_release_candidate_branch,
should_tag, should_release, release_is_draft, release_is_prerelease and release_modifier.

  primary branch push  -> tag, full release,       modifier ''
  release candidate    -> draft release,           modifier '-alpha'
  pull request         -> pre-release,             modifier '-beta'
  anything else        -> draft release,           modifier '-ci'

.EXAMPLE
./Get-PipelineConditions.ps1 -Ref refs/heads/main -EventName push -PrimaryBranch main -ReleaseCandidatePrefix rc/
#>
param (
    [Parameter(Mandatory)] [string] $Ref,
    [Parameter(Mandatory)] [string] $EventName,
    [Parameter(Mandatory)] [string] $PrimaryBranch,
    [Parameter(Mandatory)] [string] $ReleaseCandidatePrefix
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

$is_primary_branch           = $Ref -eq "refs/heads/$PrimaryBranch"
$is_pull_request_build       = $EventName -eq 'pull_request'
$is_release_candidate_branch = $Ref -like "refs/heads/$ReleaseCandidatePrefix*"

$should_tag = $is_primary_branch -and -not $is_pull_request_build

$should_release        = $true
$release_is_draft      = $true
$release_is_prerelease = $false
$release_modifier      = '-ci'

if ($is_primary_branch) {
    $release_is_draft      = $false
    $release_is_prerelease = $false
    $release_modifier      = ''
}
if ($is_release_candidate_branch) {
    $release_modifier = '-alpha'
}
if ($is_pull_request_build) {
    $release_is_draft      = $false
    $release_is_prerelease = $true
    $release_modifier      = '-beta'
}

Set-PipelineValue 'is_primary_branch'           (ConvertTo-PipelineBool $is_primary_branch)
Set-PipelineValue 'is_pull_request_build'       (ConvertTo-PipelineBool $is_pull_request_build)
Set-PipelineValue 'is_release_candidate_branch' (ConvertTo-PipelineBool $is_release_candidate_branch)
Set-PipelineValue 'should_tag'                  (ConvertTo-PipelineBool $should_tag)
Set-PipelineValue 'should_release'              (ConvertTo-PipelineBool $should_release)
Set-PipelineValue 'release_is_draft'            (ConvertTo-PipelineBool $release_is_draft)
Set-PipelineValue 'release_is_prerelease'       (ConvertTo-PipelineBool $release_is_prerelease)
Set-PipelineValue 'release_modifier'            $release_modifier
