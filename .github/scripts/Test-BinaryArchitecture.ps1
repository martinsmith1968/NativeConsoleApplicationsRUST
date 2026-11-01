<#
.SYNOPSIS
Checks that every executable in a folder was built for the expected architecture.

.DESCRIPTION
windows: reads the PE header's machine field from each *.exe (e.g. 0x8664 = x64, 0x014c = x86).
  See https://superuser.com/questions/358434/how-to-check-if-a-binary-is-32-or-64-bit-on-windows
linux:   reads the ELF class byte from each executable file (02 = 64-bit, 01 = 32-bit).
  See https://superuser.com/questions/791506/how-to-determine-if-a-linux-binary-file-is-32-bit-or-64-bit

Fails on the first mismatch, or if no executables are found.

.EXAMPLE
./Test-BinaryArchitecture.ps1 -Folder target/release -Platform windows -ExpectedFlag 0x8664
#>
param (
    [Parameter(Mandatory)] [string] $Folder,
    [Parameter(Mandatory)] [ValidateSet('windows', 'linux')] [string] $Platform,
    [Parameter(Mandatory)] [string] $ExpectedFlag
)

$ErrorActionPreference = 'Stop'

function Read-FileBytes {
    param ([string] $Path, [int] $Count)

    $buffer = New-Object byte[] $Count
    $stream = [System.IO.File]::OpenRead($Path)
    try {
        $read = $stream.Read($buffer, 0, $Count)
    }
    finally {
        $stream.Close()
    }
    if ($read -lt $Count) { [Array]::Resize([ref] $buffer, $read) }
    , $buffer
}

function Get-PeMachineFlag {
    param ([string] $Path)

    $PE_POINTER_OFFSET = 60
    $MACHINE_OFFSET = 4

    $data = Read-FileBytes -Path $Path -Count 4096
    $pe_header = [System.BitConverter]::ToInt32($data, $PE_POINTER_OFFSET)
    $machine = [System.BitConverter]::ToUInt16($data, $pe_header + $MACHINE_OFFSET)

    # e.g. 0x8664, 0x014c (hex digits padded to an even count)
    $hex = '{0:x}' -f $machine
    if ($hex.Length % 2 -ne 0) { $hex = '0' + $hex }
    "0x$hex"
}

function Get-ElfClassFlag {
    param ([string] $Path)

    $data = Read-FileBytes -Path $Path -Count 5
    '{0:x2}' -f $data[4]
}

Write-Host "Checking architecture for '$Folder' (expected $ExpectedFlag)"

if ($Platform -eq 'windows') {
    $files = @(Get-ChildItem -Path $Folder -File -Filter *.exe)
} else {
    $files = @(Get-ChildItem -Path $Folder -File | Where-Object {
        ($_.UnixFileMode -band [System.IO.UnixFileMode]::UserExecute) -ne 0
    })
}

if ($files.Count -eq 0) {
    Write-Host "No executable files found in '$Folder'" -ForegroundColor Red
    exit 1
}

$file_count = 0
foreach ($file in $files) {
    ++$file_count
    Write-Host "-- [$file_count]: Checking: $($file.FullName)"

    if ($Platform -eq 'windows') {
        $actual = Get-PeMachineFlag -Path $file.FullName
    } else {
        $actual = Get-ElfClassFlag -Path $file.FullName
    }

    if ($actual -ne $ExpectedFlag) {
        Write-Host "   Architecture mismatch for '$($file.FullName)'. Expected: $ExpectedFlag, Actual: $actual" -ForegroundColor Red
        exit 1
    }

    Write-Host "   Architecture matches for '$($file.FullName)'. Expected: $ExpectedFlag, Actual: $actual" -ForegroundColor Green
}
