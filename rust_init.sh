#!/bin/bash

# --- 1. Define the destination directories ---
DESTINATION_DIRS="01-variables-mutability-types 02-ownership-secret-sauce 03-mutable-borrowing-reference 04-structs-impl-blocks 05-enums-pattern-matching 06-traits-generics 07-error-handling 08-custom-errors 09-file-io-error-handling 10-Async-and-Concurrency-with-Tokio"

# --- 2. The For Loop ---
for dir in $DESTINATION_DIRS; do
    
    if [ -d "$dir" ]; then
        echo "Attempting to initialize rust in '$dir'..."
        
        # Remove the first 3 characters and convert to lowercase
        PROJECT_NAME=$(echo "${dir:3}" | tr '[:upper:]' '[:lower:]')
        
        cd "$dir" || { echo "ERROR: Could not change into $dir"; exit 1; }
        
        cargo init --name "$PROJECT_NAME"
        
        if [ $? -eq 0 ]; then
            echo "✅ Finished initializing rust in $dir (Package: $PROJECT_NAME)"
        else
            echo "❌ Failed to initialize rust in $dir. Check cargo output above."
        fi
        
        cd ..
        
    else
        echo "⚠️ Directory '$dir' not found or is not a directory. Skipping."
    fi

done

echo "🎉 All initialization attempts complete."