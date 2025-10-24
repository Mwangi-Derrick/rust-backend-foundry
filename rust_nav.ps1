# rust_nav.ps1
# A PowerShell script to easily navigate the Rust Masterclass repository.

function Invoke-RustNav {
    [CmdletBinding()]
    Param(
        [Parameter(Mandatory=$true, Position=0)]
        [ValidateSet("list", "goto", "help")]
        [string]$Command,

        [Parameter(Position=1)]
        [string]$Target = ""
    )

    $scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Definition
    $rootPath = $scriptPath # Assuming the script is in the repo root

    function Get-LessonPath {
        Param(
            [string]$ModuleNum,
            [string]$LessonNum
        )
        $modulePattern = "Module-$ModuleNum-*"
        $lessonPattern = "Lesson-$ModuleNum-$LessonNum-*"

        $moduleDir = Get-ChildItem -Path $rootPath -Directory -Filter $modulePattern | Select-Object -First 1
        if (-not $moduleDir) {
            Write-Host "Error: Module $ModuleNum not found." -ForegroundColor Red
            return $null
        }

        $lessonDir = Get-ChildItem -Path $moduleDir.FullName -Directory -Filter $lessonPattern | Select-Object -First 1
        if (-not $lessonDir) {
            Write-Host "Error: Lesson $ModuleNum.$LessonNum not found in module '$($moduleDir.Name)'." -ForegroundColor Red
            return $null
        }
        return $lessonDir.FullName
    }

    switch ($Command) {
        "list" {
            Write-Host "Rust Masterclass Curriculum:" -ForegroundColor Green
            $moduleDirs = Get-ChildItem -Path $rootPath -Directory -Filter "Module-*" | Sort-Object Name
            foreach ($moduleDir in $moduleDirs) {
                # Corrected: Use ${1} and ${2} for backreferences
                $moduleName = $moduleDir.Name -replace "Module-(\d+)-?(.+)", "${1}: ${2}"
                Write-Host "  $moduleName" -ForegroundColor Cyan

                $lessonDirs = Get-ChildItem -Path $moduleDir.FullName -Directory -Filter "Lesson-*" | Sort-Object Name
                foreach ($lessonDir in $lessonDirs) {
                    # Corrected: Use ${1}, ${2}, and ${3} for backreferences
                    $lessonName = $lessonDir.Name -replace "Lesson-(\d+)-(\d+)-?(.+)", "  ${1}.${2}: ${3}"
                    Write-Host "    $lessonName" -ForegroundColor Yellow
                }
            }
        }
        "goto" {
            if (-not $Target) {
                Write-Host "Error: 'goto' command requires a target (e.g., 'goto 1.1')." -ForegroundColor Red
                return
            }
            
            # Normalize target format (e.g., 1.1, 01.01, 1-1, 01-1)
            if ($Target -match "^(\d+)[\\.-](\d+)$") {
                $moduleNum = $Matches[1].PadLeft(2, '0')
                $lessonNum = $Matches[2].PadLeft(1, '0') # Lessons can be 1.1, 1.10, 1.100
            } else {
                Write-Host "Error: Invalid target format. Use M.L (e.g., 1.1, 01.01)." -ForegroundColor Red
                return
            }

            $fullPath = Get-LessonPath -ModuleNum $moduleNum -LessonNum $lessonNum
            if ($fullPath) {
                Write-Host "Navigating to: $fullPath" -ForegroundColor Green
                Set-Location $fullPath
            }
        }
        "help" {
            Write-Host "\nUsage: Invoke-RustNav <command> [target]" -ForegroundColor Green
            Write-Host "Commands:" -ForegroundColor Green
            Write-Host "  list        : Lists all modules and lessons in the repository."
            Write-Host "  goto <M.L>  : Changes directory to the specified lesson. M is module number, L is lesson number (e.g., 'goto 1.1', 'goto 03.2')."
            Write-Host "  help        : Displays this help message."
            Write-Host "\nExample:" -ForegroundColor Green
            Write-Host "  Invoke-RustNav list"
            Write-Host "  Invoke-RustNav goto 02.5"
            Write-Host "\nTo make 'rust_nav' an alias, add this to your PowerShell profile:" -ForegroundColor DarkYellow
            Write-Host "  function rust_nav { Invoke-RustNav @args }"
            Write-Host "  Set-Alias -Name nav -Value rust_nav"
        }
    }
}

# To make it easier to use, you can add an alias to your PowerShell profile.
# For example, add the following to your $PROFILE file:
# function rust_nav { Invoke-RustNav @args }
# Set-Alias -Name nav -Value rust_nav
# Then you can just type 'nav list' or 'nav goto 1.1'