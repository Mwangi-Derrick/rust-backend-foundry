# Navigating the Rust Masterclass Repository

This guide will help you use the `rust_nav.ps1` PowerShell script to easily navigate this deeply nested Rust Masterclass repository.

## 🚀 The `rust_nav.ps1` Script

I've created a custom PowerShell script named `rust_nav.ps1` in the root of this repository. This script lets you list all modules and lessons, and quickly `cd` into any specific lesson directory using a simple shorthand.

### Script Location

You can find the script here:
`C:\Users\user\Documents\projects\rust_basics\rust_nav.ps1`

## 🛠️ Setup: Making the Script Easily Accessible

For the best experience, you'll want to make this script callable directly from your PowerShell terminal without typing the full path every time. This is done by adding a function/alias to your PowerShell profile.

### Step 1: Allow Script Execution (if needed)

If you haven't already, you might need to change your PowerShell execution policy to allow scripts to run. You only need to do this once.

1. Open PowerShell as an Administrator.
2. Run the command:
    ```powershell
    Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
    ```
    Confirm with `Y` if prompted.

### Step 2: Edit Your PowerShell Profile

Your PowerShell profile is a script that runs every time PowerShell starts, allowing you to customize your environment with aliases, functions, and variables.

1. Open your PowerShell profile in a text editor:
    ```powershell
    notepad $PROFILE
    ```
    (If the file doesn't exist, `notepad` will offer to create it.)

2. Add the following lines to the end of your `$PROFILE` file. These lines define a function `rust_nav` that wraps our script and sets a convenient alias `nav` to call it.
    ```powershell
    # Rust Masterclass Navigation Script Alias
    # Ensure the path to rust_nav.ps1 is correct for your system.
    function rust_nav { & "C:\Users\user\Documents\projects\rust_basics\rust_nav.ps1" -Command $args }
    Set-Alias -Name nav -Value rust_nav
    ```
    **Important:** Double-check that the path `C:\Users\user\Documents\projects\rust_basics\rust_nav.ps1` matches the actual location of the script on your system.

3. Save and close the `$PROFILE` file.

### Step 3: Reload Your PowerShell Profile

Either restart your PowerShell terminal, or run the following command to reload your profile in the current session:

```powershell
. $PROFILE
```

(Note the `.` followed by a space before `$PROFILE`.)

## ✨ Usage: Navigating with Ease

Now that the setup is complete, you can use the `nav` alias from anywhere in your terminal.

### 1. Listing All Modules and Lessons

To get an overview of the entire curriculum, simply use the `list` command:

```powershell
nav list
```

**Expected Output Example:**
```
Rust Masterclass Curriculum:
  01: Core Foundations
    01.1: Variables, Mutability & Shadowing
    01.2: Primitive Types & Type Inference
    ...
  02: Ownership, Borrowing & Lifetimes
    02.1: Ownership Model
    02.2: Move Semantics vs Clone vs Copy
    ...
```

### 2. Going Directly to a Specific Lesson

To jump straight into any lesson, use the `goto` command followed by the module and lesson number in `M.L` format:

*   **Example: Go to Module 1, Lesson 1**
    ```powershell
    nav goto 1.1
    ```
    This will change your current directory to: 
    `C:\Users\user\Documents\projects\rust_basics\Module-01-Core-Foundations\Lesson-01-1-Variables-Mutability-Shadowing`

*   **Example: Go to Module 10, Lesson 5**
    ```powershell
    nav goto 10.5
    ```
    This will take you to: 
    `C:\Users\user\Documents\projects\rust_basics\Module-10-Advanced-Concurrency-Patterns\Lesson-10-5-Graceful-Shutdowns-with-select`

*   **Flexible Input:** The `goto` command is smart enough to handle various formats:
    *   `nav goto 1.1`
    *   `nav goto 01.01`
    *   `nav goto 1-1`
    *   `nav goto 01-1`

### 3. Getting Help

If you ever forget the commands, just type:

```powershell
nav help
```

## 🎯 How This Enhances Your Learning

By simplifying navigation, you can now:

-   **Focus on Content:** Spend less time battling the file system and more time delving into the `main.rs` code and `analysis.md` explanations.
-   **Rapid Context Switching:** Quickly jump between related lessons to review concepts or see how they evolve across modules.
-   **Fluid Exploration:** Encourage curiosity! Easily explore lessons that pique your interest without tedious `cd` commands.

Happy Rusting!
