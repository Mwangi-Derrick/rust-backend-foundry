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

    $rootPath = $PSScriptRoot

    function Get-LessonPath {
        Param(
            [string]$ModuleNum,
            [string]$LessonNum
        )
        $modulePattern = "Module-$ModuleNum-*"
        $lessonPattern = "Lesson-$ModuleNum-$LessonNum-*"

        $moduleDir = Get-ChildItem -Path $rootPath -Filter $modulePattern | Where-Object {$_.PSIsContainer} | Select-Object -First 1
        if (-not $moduleDir) {
            Write-Host "Error: Module $ModuleNum not found." -ForegroundColor Red
            return $null
        }

        $lessonDir = Get-ChildItem -Path $moduleDir.FullName -Filter $lessonPattern | Where-Object {$_.PSIsContainer} | Select-Object -First 1
        if (-not $lessonDir) {
            Write-Host "Error: Lesson $ModuleNum.$LessonNum not found in module '$($moduleDir.Name)'." -ForegroundColor Red
            return $null
        }
        return $lessonDir.FullName
    }

    switch ($Command) {
        "list" {
            Write-Host "Rust Masterclass Curriculum:" -ForegroundColor Green
            $moduleDirs = Get-ChildItem -Path $rootPath -Filter "Module-*" | Where-Object {$_.PSIsContainer} | Sort-Object Name
            foreach ($moduleDir in $moduleDirs) {
                if ($moduleDir.Name -match "^Module-(\d+)(?:-(.+))?$") {
                    $moduleNumber = $Matches[1]
                    $moduleTitle = if ($Matches.ContainsKey(2)) { $Matches[2] -replace '-', ' ' } else { "" }
                    # Corrected: Use ${} to delimit variable names
                    $moduleName = "  ${moduleNumber}: $moduleTitle"
                } else {
                    $moduleName = "  " + $moduleDir.Name # Fallback if regex doesn't match
                }
                Write-Host $moduleName -ForegroundColor Cyan

                $lessonDirs = Get-ChildItem -Path $moduleDir.FullName -Filter "Lesson-*" | Where-Object {$_.PSIsContainer} | Sort-Object Name
                foreach ($lessonDir in $lessonDirs) {
                    if ($lessonDir.Name -match "^Lesson-(\d+)-(\d+)(?:-(.+))?$") {
                        $lessonModuleNumber = $Matches[1]
                        $lessonNumber = $Matches[2]
                        $lessonTitle = if ($Matches.ContainsKey(3)) { $Matches[3] -replace '-', ' ' } else { "" }
                        # Corrected: Use ${} to delimit variable names
                        $lessonName = "    ${lessonModuleNumber}.${lessonNumber}: $lessonTitle"
                    } else {
                        $lessonName = "    " + $lessonDir.Name # Fallback
                    }
                    Write-Host $lessonName -ForegroundColor Yellow
                }
            }
        }
        "goto" {
            if (-not $Target) {
                Write-Host "Error: 'goto' command requires a target (e.g., 'goto 1.1')." -ForegroundColor Red
                return
            }
            
            if ($Target -match "^(\d+)[\.-](\d+)$") {
                $moduleNum = $Matches[1].PadLeft(2, '0')
                $lessonNum = $Matches[2].PadLeft(1, '0')
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
            Write-Host "  . 'C:\Users\user\Documents\projects\rust_basics\rust_nav.ps1'"
            Write-Host "  Set-Alias -Name nav -Value Invoke-RustNav"
        }
    }
}

# To make it easier to use, you can add an alias to your PowerShell profile.
# For example, add the following to your $PROFILE file:
# . 'C:\Users\user\Documents\projects\rust_basics\rust_nav.ps1'
# Set-Alias -Name nav -Value Invoke-RustNav
# Then you can just type 'nav list' or 'nav goto 1.1'