#!/bin/bash

# --- 1. Define the source folder ---
SOURCE_FOLDER="./01-variables-mutability-types" 

# --- 2. Define the destination directories ---
# !!! FIX: Directories must be separated by SPACES, not slashes !!!
DESTINATION_DIRS="02-ownership-secret-source 03-mutable-borrwing-reference 04-structs-impl-blocks 05-enums-pattern-matching"

# --- 3. The For Loop ---
for dir in $DESTINATION_DIRS; do
    
    # Check if the destination directory exists and is a directory
    if [ -d "$dir" ]; then
        echo "Copying contents of '$SOURCE_FOLDER' into '$dir'..."
        
        # The key command: 'cp -r' for recursive copy, and '/.' to copy contents
        cp -r "$SOURCE_FOLDER/." "$dir/"
        
        echo "✅ Finished copying to $dir"
    else
        # This is the message you would have seen for the single, long path.
        echo "⚠️ Destination directory '$dir' does not exist or is not a directory. Skipping."
    fi

done

echo "🎉 All copy operations complete."