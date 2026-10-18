# Checks the Scoop manifest scripts/release/package_managers.py wrote, as Scoop takes it: Scoop
# installs `hpr` from a copy of the manifest whose URLs name this run's own archives, served from
# this machine, since the release the manifest points at doesn't exist yet. The hash and
# extract_dir stay as written, so the install also checks them, and the installed `hpr` must
# print the manifest's version. The release workflow runs it on Windows (issue #434).
#
# Scoop's installer is pinned to a commit of ScoopInstaller/Install; the Scoop it installs is its
# current one, as a user's would be. The job holds no secrets and can't write to the repository.
#
# Usage: scripts/release/smoke-scoop.ps1 <manifest> <archives folder>

param(
  [Parameter(Mandatory = $true)][string]$Manifest,
  [Parameter(Mandatory = $true)][string]$Archives
)
$ErrorActionPreference = 'Stop'
$installer = 'https://raw.githubusercontent.com/ScoopInstaller/Install/1e2f334083d609986d8c8bc9e31ae8e87c39fab4/install.ps1'
$port = 8765

$script = Join-Path $env:RUNNER_TEMP 'install-scoop.ps1'
Invoke-RestMethod -Uri $installer -OutFile $script
& $script -RunAsAdmin
$env:PATH = "$env:USERPROFILE\scoop\shims;$env:PATH"

$folder = (Resolve-Path $Archives).Path
$server = Start-Process python -PassThru -NoNewWindow -ArgumentList @(
  '-m', 'http.server', "$port", '--bind', '127.0.0.1', '--directory', $folder)
try {
  for ($i = 0; ; $i++) {
    try { Invoke-WebRequest -Uri "http://127.0.0.1:$port/" -UseBasicParsing | Out-Null; break }
    catch { if ($i -ge 30) { throw 'smoke-scoop: the local file server did not start' }; Start-Sleep 1 }
  }

  $json = Get-Content $Manifest -Raw | ConvertFrom-Json
  $release = '^https://github\.com/nrdptel/fusionspace-eridanus/releases/download/v[^/]+/'
  foreach ($arch in $json.architecture.PSObject.Properties) {
    if ($arch.Value.url -notmatch $release) { throw "smoke-scoop: $($arch.Name)'s URL isn't a release download" }
    $arch.Value.url = $arch.Value.url -replace $release, "http://127.0.0.1:$port/"
  }
  $local = Join-Path $env:RUNNER_TEMP 'fusionspace-hpr.json'
  $json | ConvertTo-Json -Depth 10 | Set-Content -Path $local -Encoding utf8

  scoop install $local
  if ($LASTEXITCODE) { throw "smoke-scoop: scoop install exited with $LASTEXITCODE" }
  $printed = hpr --version
  if ($LASTEXITCODE) { throw "smoke-scoop: hpr --version exited with $LASTEXITCODE" }
  Write-Output $printed
  if ($printed -notmatch [regex]::Escape(" $($json.version) ")) {
    throw "smoke-scoop: hpr printed '$printed', not version $($json.version)"
  }
} finally {
  Stop-Process -Id $server.Id -ErrorAction SilentlyContinue
}
