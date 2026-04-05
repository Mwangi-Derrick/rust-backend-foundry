# Navigating the Rust Masterclass Repository

This guide will help you set up and use the navigation scripts to easily explore this deeply nested Rust Masterclass repository. Choose your platform below: PowerShell (Windows) or Bash (Linux/Mac).

---

## 📍 Navigation Scripts Overview

Two custom scripts are included to make navigation effortless:

- **`rust_nav.ps1`** — For PowerShell (Windows users)
- **`rust_nav.sh`** — For Bash/Zsh (Linux/Mac users)

Both scripts provide identical functionality: list all modules/lessons, and jump directly to any lesson with a simple command.

---

# 🪟 WINDOWS: PowerShell Setup

## 🚀 What You'll Get

After setup, you'll be able to use the `nav` command from anywhere in your terminal:

```powershell
nav list                  # List all modules and lessons
nav goto 1.1             # Jump to Module 1, Lesson 1
nav goto 6.2             # Jump to Module 6, Lesson 2
nav help                 # Show all commands
```

## 🛠️ Setup Instructions

### Step 1: Allow Script Execution (One-Time)

Open **PowerShell as Administrator** and run:

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Confirm with `Y` when prompted. This allows PowerShell scripts to run from your profile.

### Step 2: Edit Your PowerShell Profile

Open your profile in a text editor:

```powershell
notepad $PROFILE
```

**If the file doesn't exist:** Click "Create" when Notepad offers to create it.

**Add these lines to the end of the file:**

```powershell
# =====================================================
# Rust Masterclass Navigation Function
# =====================================================
# YOU MUST UPDATE THIS PATH TO MATCH YOUR SYSTEM
$RUST_NAV_SCRIPT = "C:\Users\$env:USERNAME\Documents\projects\rust-backend-foundry\rust_nav.ps1"

function nav {
    if (Test-Path $RUST_NAV_SCRIPT) {
        & $RUST_NAV_SCRIPT @args
    } else {
        Write-Host "Error: rust_nav.ps1 not found at $RUST_NAV_SCRIPT" -ForegroundColor Red
        Write-Host "Please update the path in your PowerShell profile." -ForegroundColor Yellow
    }
}

# =====================================================
```

**⚠️ IMPORTANT:** Verify the path matches your actual rust-backend-foundry location. If it's elsewhere, update the `$RUST_NAV_SCRIPT` path accordingly.

### Step 3: Reload Your PowerShell Profile

Either:
- Restart PowerShell entirely, OR
- Run this command in your current session:

```powershell
. $PROFILE
```

### Step 4: Verify Setup

Test the setup by running:

```powershell
nav help
```

You should see help information. If you get an error about the script not being found, double-check the path in Step 2.

---

## ✨ PowerShell Usage Examples

### List All Modules and Lessons

```powershell
nav list
```

**Output Example:**
```
Rust Masterclass Curriculum:
  01: Core Foundations
    01.1: Variables, Mutability & Shadowing
    01.2: Primitive Types & Type Inference
    01.3: String vs str vs slices
    01.4: Control Flow, Loops & Match Statements
  02: Ownership, Borrowing & Lifetimes
    02.1: Ownership Model
    02.2: Move Semantics vs Clone vs Copy
    ...
```

### Jump to a Specific Lesson

```powershell
nav goto 1.1                # Module 1, Lesson 1
nav goto 6.2                # Module 6, Lesson 2
nav goto 10.5               # Module 10, Lesson 5
```

The script accepts flexible input formats:
- `nav goto 1.1` ✅
- `nav goto 01.01` ✅
- `nav goto 1-1` ✅

### Get Help

```powershell
nav help
```

---

# 🐧 LINUX/MAC: Bash/Zsh Setup

## 🚀 What You'll Get

After setup, you'll be able to use the `nav` command from anywhere in your terminal:

```bash
nav list                  # List all modules and lessons
nav goto 1.1             # Jump to Module 1, Lesson 1
nav goto 6.2             # Jump to Module 6, Lesson 2
nav help                 # Show all commands
```

## 🛠️ Setup Instructions

### Step 1: Make the Script Executable

First, ensure the script has execute permissions:

```bash
chmod +x ~/Documents/projects/rust-backend-foundry/rust_nav.sh
```

**Adjust the path if needed** if rust-backend-foundry is in a different location.

### Step 2: Edit Your Shell Profile

Depending on your shell, edit the appropriate file:

**For Bash:**
```bash
nano ~/.bashrc
```

**For Zsh:**
```bash
nano ~/.zshrc
```

**Add these lines to the end:**

```bash
# =====================================================
# Rust Masterclass Navigation Alias
# =====================================================
# YOU MUST UPDATE THIS PATH TO MATCH YOUR SYSTEM
export RUST_NAV_SCRIPT="$HOME/Documents/projects/rust-backend-foundry/rust_nav.sh"

alias nav='source "$RUST_NAV_SCRIPT"'

# =====================================================
```

**⚠️ IMPORTANT:** Verify the path matches your actual rust-backend-foundry location. If it's elsewhere, update the `RUST_NAV_SCRIPT` path accordingly.

**To save and exit (in nano):**
- Press `Ctrl + X`
- Press `Y` to confirm
- Press `Enter` to save

### Step 3: Review Your Shell Profile (Optional)

Verify your changes were saved:

```bash
# For Bash:
tail ~/.bashrc

# For Zsh:
tail ~/.zshrc
```

### Step 4: Reload Your Shell Profile

Either:
- Restart your terminal entirely, OR
- Run this command in your current session:

```bash
# For Bash:
source ~/.bashrc

# For Zsh:
source ~/.zshrc
```

### Step 5: Verify Setup

Test the setup by running:

```bash
nav help
```

You should see help information. If you get an error, double-check the path in Step 2.

---

## ✨ Bash/Zsh Usage Examples

### List All Modules and Lessons

```bash
nav list
```

**Output Example:**
```
Rust Masterclass Curriculum:
  01: Core Foundations
    01.1: Variables, Mutability & Shadowing
    01.2: Primitive Types & Type Inference
    01.3: String vs str vs slices
    01.4: Control Flow, Loops & Match Statements
  02: Ownership, Borrowing & Lifetimes
    02.1: Ownership Model
    02.2: Move Semantics vs Clone vs Copy
    ...
```

### Jump to a Specific Lesson

```bash
nav goto 1.1                # Module 1, Lesson 1
nav goto 6.2                # Module 6, Lesson 2
nav goto 10.5               # Module 10, Lesson 5
```

The script accepts flexible input formats:
- `nav goto 1.1` ✅
- `nav goto 01.01` ✅
- `nav goto 1-1` ✅

### Get Help

```bash
nav help
```

---

## 🎯 How Navigation Enhances Your Learning

By setting up these scripts, you can:

- **⚡ Focus on Content:** Spend less time navigating folders and more time diving into `main.rs` code and `analysis.md` explanations.
- **🔄 Rapid Context Switching:** Quickly jump between related lessons to review concepts or see how they evolve.
- **🧭 Fluid Exploration:** Explore lessons freely without tedious directory commands.
- **⏱️ Save Time:** Get to your next lesson in seconds instead of clicking through deep folder hierarchies.

---

## 🐛 Troubleshooting

### PowerShell: "Script cannot be loaded because running scripts is disabled"

**Solution:** Run Step 1 again:
```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### PowerShell: `nav` command not found

**Solution:** 
1. Verify the path in your profile is correct
2. Reload your profile: `. $PROFILE`
3. Restart PowerShell if reloading didn't work

### Bash/Zsh: "command not found"

**Solution:**
1. Verify the path in ~/.bashrc or ~/.zshrc is correct
2. Ensure the script is executable: `chmod +x ~/Documents/projects/rust-backend-foundry/rust_nav.sh`
3. Reload your profile: `source ~/.bashrc` or `source ~/.zshrc`

### Both: "Lesson not found" error

**Solution:** Use `nav list` to see all available lessons, then verify the module and lesson numbers.

---

## 🎓 Example Learning Flow with Navigation

```powershell
# Windows PowerShell Example
nav list                    # See all lessons
nav goto 1.1               # Jump to Module 1, Lesson 1
cargo run                  # Run the code
# Read analysis.md, experiment, then:
nav goto 1.2               # Move to next lesson
```

```bash
# Linux/Mac Bash Example
nav list                    # See all lessons
nav goto 1.1               # Jump to Module 1, Lesson 1
cargo run                  # Run the code
# Read analysis.md, experiment, then:
nav goto 1.2               # Move to next lesson
```

---

Happy Rusting! 🦀
