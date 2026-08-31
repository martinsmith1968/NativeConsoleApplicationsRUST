function Build-AppsList {
    param (
        [string]$base_path_name
    )

    $allfiles = @{}
    if (Test-Path -Path $base_path_name -PathType Container) {
        $allfiles = Get-ChildItem -Path $base_path_name -File -Filter "*.exe" -Recurse
        Write-Host "Found $($allfiles.Length) candidate executables"
    }    

    $allfiles = $allfiles | Where-Object { -Not $_.FullName.contains("deps") } | Where-Object { -Not $_.FullName.contains("_build-") }

    $apps = @{}

    foreach ($file in $allfiles | Where-Object { $_.FullName.contains("release") }) {
        $apps[$file.Name] = $file
    }

    foreach ($file in $allfiles | Where-Object { $_.FullName.contains("debug") }) {
        if ( $apps.ContainsKey($file.Name) ) {
            continue
        }
        $file_name_only = [System.IO.Path]::GetFileNameWithoutExtension($file.Name)
        $apps[$file_name_only] = $file
    }

    return $apps
}

#---------------------------------------------------------------------------------------------------

function Search-AppByName {
    param (
        [hashtable]$apps,
        [string]$app_name
    )
    Write-Host "Looking for app : $($app_name)..." -ForegroundColor Yellow
    if ( $apps.ContainsKey($app_name) ) {
        return $apps[$app_name]
    }
    return $null
}

#---------------------------------------------------------------------------------------------------

function Start-AppAndCaptureOutput {
    [OutputType([string])]
    param(
        [alias("exe")]
        [string]$executable_name,

        [alias("params")]
        [string[]]$parameters             = @( ),

        [alias("shell")]
        [bool]$use_shell_execute          = $false,

        [alias("env")]
        [hashtable]$environment_variables = @{ }
    )

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName               = $executable_name
    if ($parameters -ne $null) {
        $parameters | ForEach-Object { $psi.ArgumentList.Add($_) }
    }
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError  = $true
    $psi.UseShellExecute        = $use_shell_execute
    $psi.CreateNoWindow         = $true
    if ($environment_variables -ne $null) {
        foreach ($h in $environment_variables.GetEnumerator()) {
            $psi.EnvironmentVariables[$h.Name] = $h.Value
        }
    }

    $proc = [System.Diagnostics.Process]::Start($psi)
    $stdout = $proc.StandardOutput.ReadToEnd()
    $proc.WaitForExit()

    return $stdout
}
