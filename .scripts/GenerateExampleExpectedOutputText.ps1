$app_output_path = Join-Path $PSScriptRoot -ChildPath ".." "target" "release"
$temp_base_path  = [System.IO.Path]::GetTempPath()
$run_id          = [System.Guid]::NewGuid().ToString("N")
$temp_run_path   = Join-Path -Path $temp_base_path -ChildPath $run_id

Write-Host "App Output Path : $($app_output_path)"
Write-Host "Temp Path       : $($temp_base_path)"
Write-Host "Run Path        : $($temp_run_path)"


# Start

$allfiles = Get-ChildItem -Path $app_output_path -File -Filter "*.exe"
Write-Host "Found $($allfiles.Length) candidate executables"

# Build Apps List
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

function Search-AppByName {
    param (
        [string]$app_name
    )
    Write-Host "Looking for app : $($app_name)..." -ForegroundColor Yellow
    if ( $apps.ContainsKey($app_name) ) {
        return $apps[$app_name]
    }
    return $null
}


function Set-ExpectedOutput {
    param (
        [string]$app_full_path,
        [string]$arguments,
        [string]$output_filename,
        [string]$expected_output_path = $null
    )

    $app_name = [System.IO.Path]::GetFileNameWithoutExtension($app_name)

    if ( [string]::IsNullOrEmpty($expected_output_path) ) {
        $expected_output_path = Join-Path -Path $PSScriptRoot -ChildPath ".." $app_name "tests" "Expectedoutput"
    }

    $parameters = $arguments.Split("|")

    Write-Host "  $($app_name) - Generating : $($output_filename)"
    & $app_full_path $parameters | Set-Content -Path (Join-Path -Path $expected_output_path -ChildPath "$($output_filename).example") -Encoding UTF8
}


# Generate For : BannerText
$app_name = "hashcalc.exe"
$app = Search-AppByName -app_name $app_name
if ( $null -ne $app ) {
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-?"                         -output_filename "Execute_with_help_request_produces_arguments_list"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|Hello World"             -output_filename "Execute_with_text_only_default_algorithm_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|Hello World|-a|sha256"   -output_filename "Execute_with_text_only_sha256_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|Hello World|-a|sha512"   -output_filename "Execute_with_text_only_sha512_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|Hello World|-a|sha1"     -output_filename "Execute_with_text_only_sha1_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|Hello World|-a|md5"      -output_filename "Execute_with_text_only_md5_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|Hello World|-a|base64"   -output_filename "Execute_with_text_only_base64_produces_expected_output"
}


# Generate For : Stopwatch
$app_name = "uuidgen.exe"
$app = Search-AppByName -app_name $app_name
if ( $null -ne $app ) {
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-?"                     -output_filename "Execute_with_help_request_produces_arguments_list"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid"                -output_filename "Execute_for_default_type_guid_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-c|5"           -output_filename "Execute_for_5_instance_default_type_guid_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-y"             -output_filename "Execute_for_default_type_guid_hyphenated_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-u"             -output_filename "Execute_for_default_type_guid_uppercase_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-y|-u"          -output_filename "Execute_for_default_type_guid_hyphenated_uppercase_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v6"          -output_filename "Execute_for_v6_guid_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v6|-y"       -output_filename "Execute_for_v6_guid_hyphenated_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v6|-u"       -output_filename "Execute_for_v6_guid_uppercase_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v6|-y|-u"    -output_filename "Execute_for_v6_guid_hyphenated_uppercase_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v7"          -output_filename "Execute_for_v7_guid_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v7|-y"       -output_filename "Execute_for_v7_guid_hyphenated_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v7|-u"       -output_filename "Execute_for_v7_guid_uppercase_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|guid|-v|v7|-y|-u"    -output_filename "Execute_for_v7_guid_hyphenated_uppercase_produces_expected_output"

    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|nanoid"              -output_filename "Execute_for_nanoid_produces_expected_output"
    Set-ExpectedOutput -app_full_path $app.FullName -arguments "-t|nanoid|-c|5"         -output_filename "Execute_for_5_instance_nanoid_produces_expected_output"
}


Write-Host "DONE: Expected Output Text Population Complete." -ForegroundColor Green
