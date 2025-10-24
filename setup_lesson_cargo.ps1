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

            # 1. Move main.rs from root to src/main.rs if it exists in root
            if (Test-Path $mainRsInRootPath) {
                if (-not (Test-Path $srcDirPath)) {
                    New-Item -ItemType Directory -Path $srcDirPath | Out-Null
                    Write-Host "    Created src/ directory" -ForegroundColor DarkGreen
                }
                if ($Force -or -not (Test-Path $mainRsInSrcPath)) {
                    Move-Item -Path $mainRsInRootPath -Destination $mainRsInSrcPath -Force
                    Write-Host "    Moved main.rs to src/main.rs" -ForegroundColor DarkGreen
                } else {
                    Write-Host "    src/main.rs already exists. Skipping move." -ForegroundColor DarkYellow
                }
            } elseif (-not (Test-Path $mainRsInSrcPath)) {
                Write-Host "    Warning: No main.rs found for lesson $lessonName. Skipping Cargo.toml creation." -ForegroundColor DarkYellow
                continue # Skip if no main.rs to build
            }

            # 2. Read main.rs content to detect dependencies
            $mainRsContent = Get-Content -Path $mainRsInSrcPath -Raw -Encoding UTF8
            $lessonDependencies = New-Object System.Collections.ArrayList # Use ArrayList to store unique dependency strings
            $lessonDevDependencies = @{ "criterion" = "criterion = { workspace = true }" } # Always include criterion as dev-dependency

            # Define a mapping of keywords/patterns to dependencies
            # Using an array of objects to avoid duplicate keys in a HashTable and allow multiple patterns for one dep
            $dependencyMap = @(
                @{ Pattern = "tokio::"; DependencyString = "tokio = { workspace = true }" },
                @{ Pattern = "anyhow::"; DependencyString = "anyhow = { workspace = true }" },
                @{ Pattern = "async_trait::"; DependencyString = "async-trait = { workspace = true }" },
                @{ Pattern = "tracing::"; DependencyString = "tracing = { workspace = true }" },
                @{ Pattern = "tracing_subscriber::"; DependencyString = "tracing-subscriber = { workspace = true }" },
                @{ Pattern = "rand::"; DependencyString = "rand = { workspace = true }" },
                @{ Pattern = "rayon::"; DependencyString = "rayon = { workspace = true }" },
                @{ Pattern = "pyo3::"; DependencyString = "pyo3 = { workspace = true }" },
                @{ Pattern = "wasm_bindgen::"; DependencyString = "wasm-bindgen = { workspace = true }" },
                @{ Pattern = "worker::"; DependencyString = "worker = { workspace = true }" },
                @{ Pattern = "lapin::"; DependencyString = "lapin = { workspace = true }" },
                @{ Pattern = "async_nats::"; DependencyString = "async-nats = { workspace = true }" },
                @{ Pattern = "sqlx::"; DependencyString = "sqlx = { workspace = true }" },
                @{ Pattern = "thiserror::"; DependencyString = "thiserror = { workspace = true }" },
                @{ Pattern = "std::error::Error"; DependencyString = "thiserror = { workspace = true }" }, # For custom errors
                @{ Pattern = "std::panic"; DependencyString = "anyhow = { workspace = true }" } # Often used with anyhow for context
                # Removed redundant detection for std::fs:: and std::io:: as tokio:: covers async I/O
            )

            foreach ($depMapping in $dependencyMap) {
                if ($mainRsContent -match $depMapping.Pattern) {
                    $depEntry = $depMapping.DependencyString
                    # Add only if not already present
                    if (-not ($lessonDependencies -contains $depEntry)) {
                        $null = $lessonDependencies.Add($depEntry)
                    }
                }
            }

            # 3. Generate Cargo.toml for the lesson
            if ($Force -or -not (Test-Path $cargoTomlPath)) {
                $depsString = ($lessonDependencies | Sort-Object) -join "`n"
                # Criterion as a dev-dependency is always added
                $devDepsString = ($lessonDevDependencies.Values | Sort-Object) -join "`n"

                $cargoTomlContent = @"
[package]
name = "$($lessonName.ToLower() -replace '-', '_')"
version = "0.1.0"
edition = "2021"

[dependencies]
$depsString

[dev-dependencies]
$devDepsString

[[bench]]
name = "$($lessonName.ToLower() -replace '-', '_')_benchmark"
harness = false
"@
                Set-Content -Path $cargoTomlPath -Value $cargoTomlContent -Encoding UTF8
                Write-Host "    Created Cargo.toml with specific dependencies." -ForegroundColor DarkGreen
            } else {
                Write-Host "    Cargo.toml already exists. Skipping." -ForegroundColor DarkYellow
            }

            # 4. Add to workspace members (relative path)
            $relativePath = Join-Path -Path $moduleDir.Name -ChildPath $lessonDir.Name
            $relativePath = $relativePath -replace '\\', '/' # Ensure forward slashes
            Write-Host "    DEBUG: Adding member: $relativePath" -ForegroundColor DarkGray # DEBUG PRINT

            if (-not ($workspaceMembers -contains $relativePath)) {
                $workspaceMembers += $relativePath
            }
        }
    }

    # Update root Cargo.toml with workspace members
    Write-Host "
Updating root Cargo.toml with workspace members..." -ForegroundColor Green
    $workspaceCargoTomlContent = Get-Content -Path $workspaceCargoTomlPath -Raw -Encoding UTF8
    $membersString = ($workspaceMembers | ForEach-Object { "`"$_`"" }) -join ", `n    "
    Write-Host "    DEBUG: Final members string:" -ForegroundColor DarkGray # DEBUG PRINT
    Write-Host "    $membersString" -ForegroundColor DarkGray # DEBUG PRINT
    $updatedContent = $workspaceCargoTomlContent -replace "members = \[[\]\s]*?", "members = [`n    $membersString`n]"
    Set-Content -Path $workspaceCargoTomlPath -Value $updatedContent -Encoding UTF8
    Write-Host "Root Cargo.toml updated." -ForegroundColor DarkGreen

    # Part 3: Update .gitignore
    Write-Host "
Updating .gitignore..." -ForegroundColor Green
    $gitignorePath = Join-Path $rootPath ".gitignore"
    $gitignoreContent = if (Test-Path $gitignorePath) { Get-Content -Path $gitignorePath -Raw -Encoding UTF8 } else { "" }

    if ($gitignoreContent -notmatch "^/target/$" -and $gitignoreContent -notmatch "^target/$") {
        Add-Content -Path $gitignorePath -Value "`n/target/`n" -Encoding UTF8 # Add /target/ if not present
        Write-Host "Added /target/ to .gitignore" -ForegroundColor DarkGreen
    } else {
        Write-Host "/target/ already in .gitignore. Skipping." -ForegroundColor DarkYellow
    }
    if ($gitignoreContent -notmatch "^Cargo.lock$") {
        Add-Content -Path $gitignorePath -Value "Cargo.lock`n" -Encoding UTF8 # Add Cargo.lock if not present
        Write-Host "Added Cargo.lock to .gitignore" -ForegroundColor DarkGreen
    } else {
        Write-Host "Cargo.lock already in .gitignore. Skipping." -ForegroundColor DarkYellow
    }
    
    Write-Host "
Running cargo build --workspace to verify setup..." -ForegroundColor Green
    # This will build all projects and generate Cargo.lock at the root
    # Use --all-targets to include benches, and --all-features to ensure optional deps are considered
    $cargoBuildResult = (Cargo build --workspace --all-targets --all-features 2>&1)
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Cargo build failed. Please check the output above for errors." -ForegroundColor Red
        $cargoBuildResult | Write-Host
    } else {
        Write-Host "Cargo build --workspace completed successfully!" -ForegroundColor Green
    }

    Write-Host "
Setup complete! You can now navigate to any lesson directory and run `cargo run`." -ForegroundColor Green
}

# To make it easier to use, you can add an alias to your PowerShell profile.
# For example, add the following to your $PROFILE file:
# . 'C:\Users\user\Documents\projects\rust_basics\setup_lesson_cargo.ps1'
# Set-Alias -Name setup-cargo -Value Invoke-SetupLessonCargo
# Then you can just type 'setup-cargo'