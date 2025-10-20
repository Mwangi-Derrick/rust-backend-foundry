#!/bin/bash

# --- 1. Define the destination directories ---
# The script MUST be run from the parent folder (rust_basics/) 
DESTINATION_DIRS="01-variables-mutability-types 02-ownership-secret-sauce 03-mutable-borrowing-reference 04-structs-impl-blocks 05-enums-pattern-matching 06-traits-generics 07-error-handling"

# --- 2. The For Loop ---
for dir in $DESTINATION_DIRS; do
    
    # Check if the destination directory exists and is a directory
    if [ -d "$dir" ]; then
        echo "Attempting to initialize rust in '$dir'..."
        
        # Use ${dir:3} to exclude the first 3 characters (e.g., "01-")
        PROJECT_NAME="${dir:3}"
        
        # 1. Change into the specific directory
        cd "$dir" || { echo "ERROR: Could not change into $dir"; exit 1; }
        
        # 2. Run cargo init with the corrected, valid package name
        cargo init --name "$PROJECT_NAME"
        
        # Check the exit status of cargo init
        if [ $? -eq 0 ]; then
            echo "✅ Finished initializing rust in $dir (Package: $PROJECT_NAME)"
        else
            echo "❌ Failed to initialize rust in $dir. Check cargo output above."
        fi
        
        # 3. Go back to the parent directory
        cd ..
        
    else
        echo "⚠️ Directory '$dir' not found or is not a directory. Skipping."
    fi

done

echo "🎉 All initialization attempts complete."