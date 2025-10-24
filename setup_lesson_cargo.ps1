# setup_lesson_cargo.ps1
# A PowerShell script to set up Cargo projects for each lesson in the Rust Masterclass.

function Invoke-SetupLessonCargo {
    [CmdletBinding()]
    Param(
        [Parameter(Mandatory=$false)]
        [switch]$Force # Overwrite existing Cargo.toml and src/main.rs
    )

    $rootPath = $PSScriptRoot
    $workspaceCargoTomlPath = Join-Path $rootPath "Cargo.toml"

    if (-not (Test-Path $workspaceCargoTomlPath)) {
        Write-Host "Error: Root Cargo.toml not found at $workspaceCargoTomlPath. Please ensure it exists." -ForegroundColor Red
        return
    }

    Write-Host "Starting Cargo project setup for all lessons..." -ForegroundColor Green

    $moduleDirs = Get-ChildItem -Path $rootPath -Filter "Module-*" | Where-Object {$_.PSIsContainer} | Sort-Object Name
    $workspaceMembers = @()

    foreach ($moduleDir in $moduleDirs) {
        $lessonDirs = Get-ChildItem -Path $moduleDir.FullName -Filter "Lesson-*" | Where-Object {$_.PSIsContainer} | Sort-Object Name

        foreach ($lessonDir in $lessonDirs) {
            $lessonPath = $lessonDir.FullName
            $lessonName = $lessonDir.Name # e.g., Lesson-01-1-Variables-Mutability-Shadowing
            $cargoTomlPath = Join-Path $lessonPath "Cargo.toml"
            $srcDirPath = Join-Path $lessonPath "src"
            $mainRsInRootPath = Join-Path $lessonPath "main.rs"
            $mainRsInSrcPath = Join-Path $srcDirPath "main.rs"

            Write-Host "  Setting up lesson: $lessonName" -ForegroundColor Cyan

            # 1. Create Cargo.toml for the lesson
            if ($Force -or -not (Test-Path $cargoTomlPath)) {
                # Corrected Cargo.toml template: Merged [features] sections
                $cargoTomlContent = @"
[package]
name = "$($lessonName.ToLower() -replace '-', '_')"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = { workspace = true }
async-trait = { workspace = true }
tokio = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
rand = { workspace = true }
rayon = { workspace = true }
pyo3 = { workspace = true }
wasm-bindgen = { workspace = true }
worker = { workspace = true }
lapin = { workspace = true }
async-nats = { workspace = true }
sqlx = { workspace = true }
thiserror = { workspace = true }

# Dev dependencies (e.g., for benchmarking)
[dev-dependencies]
criterion = { workspace = true }

[features]
default = []
ffi_python = ["pyo3"]
ffi_wasm = ["wasm-bindgen", "worker"]
message_brokers = ["lapin", "async-nats"]
sql_features = ["sqlx"]
error_handling = ["thiserror"]
bench = ["criterion"]
"@
                Set-Content -Path $cargoTomlPath -Value $cargoTomlContent
                Write-Host "    Created Cargo.toml" -ForegroundColor DarkGreen
            } else {
                Write-Host "    Cargo.toml already exists. Skipping." -ForegroundColor DarkYellow
            }

            # 2. Create src directory
            if (-not (Test-Path $srcDirPath)) {
                New-Item -ItemType Directory -Path $srcDirPath | Out-Null
                Write-Host "    Created src/ directory" -ForegroundColor DarkGreen
            } else {
                Write-Host "    src/ directory already exists. Skipping." -ForegroundColor DarkYellow
            }

            # 3. Move main.rs from root to src/main.rs
            if (Test-Path $mainRsInRootPath) {
                if ($Force -or -not (Test-Path $mainRsInSrcPath)) {
                    Move-Item -Path $mainRsInRootPath -Destination $mainRsInSrcPath -Force
                    Write-Host "    Moved main.rs to src/main.rs" -ForegroundColor DarkGreen
                } else {
                    Write-Host "    src/main.rs already exists. Skipping move." -ForegroundColor DarkYellow
                }
            }

            # 4. Add to workspace members (relative path)
            $relativePath = (Get-Item $lessonPath).BaseName
            if (-not ($workspaceMembers -contains $relativePath)) {
                $workspaceMembers += $relativePath
            }
        }
    }

    # Update root Cargo.toml with workspace members
    Write-Host "\nUpdating root Cargo.toml with workspace members..." -ForegroundColor Green
    $workspaceCargoTomlContent = Get-Content -Path $workspaceCargoTomlPath -Raw
    $membersString = ($workspaceMembers | ForEach-Object { "`"$_`"" }) -join ", `n    "
    $updatedContent = $workspaceCargoTomlContent -replace "members = \[\s*\]", "members = [`n    $membersString`n]"
    Set-Content -Path $workspaceCargoTomlPath -Value $updatedContent
    Write-Host "Root Cargo.toml updated." -ForegroundColor DarkGreen

    # Part 3: Update .gitignore
    Write-Host "\nUpdating .gitignore..." -ForegroundColor Green
    $gitignorePath = Join-Path $rootPath ".gitignore"
    $gitignoreContent = if (Test-Path $gitignorePath) { Get-Content -Path $gitignorePath -Raw } else { "" }

    if ($gitignoreContent -notmatch "^/target/$" -and $gitignoreContent -notmatch "^target/$") {
        Add-Content -Path $gitignorePath -Value "`n/target/`n" # Add /target/ if not present
        Write-Host "Added /target/ to .gitignore" -ForegroundColor DarkGreen
    }
    if ($gitignoreContent -notmatch "^Cargo.lock$") {
        Add-Content -Path $gitignorePath -Value "Cargo.lock`n" # Add Cargo.lock if not present
        Write-Host "Added Cargo.lock to .gitignore" -ForegroundColor DarkGreen
    }

    Write-Host "\nRunning cargo build --workspace to verify setup..." -ForegroundColor Green
    # This will build all projects and generate Cargo.lock at the root
    $cargoBuildResult = (cargo build --workspace --all-features 2>&1)
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Cargo build failed. Please check the output above for errors." -ForegroundColor Red
        $cargoBuildResult | Write-Host
    } else {
        Write-Host "Cargo build --workspace completed successfully!" -ForegroundColor Green
    }

    Write-Host "\nSetup complete! You can now navigate to any lesson directory and run `cargo run`." -ForegroundColor Green
}

# To make it easier to use, you can add an alias to your PowerShell profile.
# For example, add the following to your $PROFILE file:
# . 'C:\Users\user\Documents\projects\rust_basics\setup_lesson_cargo.ps1'
# Set-Alias -Name setup-cargo -Value Invoke-SetupLessonCargo
# Then you can just type 'setup-cargo'
