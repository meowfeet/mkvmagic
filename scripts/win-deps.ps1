$PSDefaultParameterValues = @{
    '*:ErrorAction' = 'Stop'
    '*:Force' = $true
}

$url = 'https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip'

$root = Split-Path $PSScriptRoot
$bin = Join-Path $root 'bin'
$out = Join-Path $bin 'out'
$zip = Join-Path $out 'ffmpeg.zip'

try {
    Remove-Item $bin -Recurse -ErrorAction SilentlyContinue
    New-Item -ItemType Directory $out | Out-Null

    $wc = New-Object Net.WebClient
    $wc.DownloadFile($url, $zip)

    Expand-Archive $zip $out

    foreach ($name in 'ffmpeg.exe', 'ffprobe.exe') {
        $exe = Get-ChildItem $out -Recurse -Filter $name | Select-Object -First 1

        if (!$exe) {
            throw "$name not found"
        }

        Copy-Item $exe.FullName $bin
    }

    Remove-Item $out -Recurse
}
catch {
    Write-Host "failed: $($_.Exception.Message)"
    exit 1
}
