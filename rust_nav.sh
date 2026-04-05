#!/bin/bash

# rust_nav.sh
# A bash script to easily navigate the Rust Masterclass repository.

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Function to get lesson path
get_lesson_path() {
    local module_num="$1"
    local lesson_num="$2"
    
    # Pad module number with leading zero if needed (2 digits)
    local module_num_padded=$(printf "%02d" "$module_num")
    
    # Find module directory
    local module_dir=$(find "$SCRIPT_DIR" -maxdepth 1 -type d -name "Module-${module_num_padded}-*" | head -n 1)
    
    if [[ -z "$module_dir" ]]; then
        echo -e "\033[0;31mError: Module $module_num not found.\033[0m" >&2
        return 1
    fi
    
    # Find lesson directory
    local lesson_dir=$(find "$module_dir" -maxdepth 1 -type d -name "Lesson-${module_num_padded}-${lesson_num}-*" | head -n 1)
    
    if [[ -z "$lesson_dir" ]]; then
        echo -e "\033[0;31mError: Lesson $module_num.$lesson_num not found in module '$(basename "$module_dir")'.\033[0m" >&2
        return 1
    fi
    
    echo "$lesson_dir"
}

# Function to list all modules and lessons
list_lessons() {
    echo -e "\033[0;32mRust Masterclass Curriculum:\033[0m"
    
    # Find all module directories and sort them
    local module_dirs=$(find "$SCRIPT_DIR" -maxdepth 1 -type d -name "Module-*" | sort)
    
    for module_dir in $module_dirs; do
        local module_name=$(basename "$module_dir")
        
        # Extract module number and title
        if [[ "$module_name" =~ ^Module-([0-9]+)(?:-(.+))?$ ]]; then
            local module_number="${BASH_REMATCH[1]}"
            local module_title="${BASH_REMATCH[2]}"
            module_title=$(echo "$module_title" | tr '-' ' ')
            echo -e "\033[0;36m  ${module_number}: ${module_title}\033[0m"
        else
            echo -e "\033[0;36m  $module_name\033[0m"
        fi
        
        # Find all lesson directories in this module and sort them
        local lesson_dirs=$(find "$module_dir" -maxdepth 1 -type d -name "Lesson-*" | sort)
        
        for lesson_dir in $lesson_dirs; do
            local lesson_name=$(basename "$lesson_dir")
            
            # Extract lesson module number, lesson number and title
            if [[ "$lesson_name" =~ ^Lesson-([0-9]+)-([0-9]+)(?:-(.+))?$ ]]; then
                local lesson_module_number="${BASH_REMATCH[1]}"
                local lesson_number="${BASH_REMATCH[2]}"
                local lesson_title="${BASH_REMATCH[3]}"
                lesson_title=$(echo "$lesson_title" | tr '-' ' ')
                echo -e "\033[0;33m    ${lesson_module_number}.${lesson_number}: ${lesson_title}\033[0m"
            else
                echo -e "\033[0;33m    $lesson_name\033[0m"
            fi
        done
    done
}

# Function to navigate to a lesson
goto_lesson() {
    local target="$1"
    
    if [[ -z "$target" ]]; then
        echo -e "\033[0;31mError: 'goto' command requires a target (e.g., 'goto 1.1').\033[0m" >&2
        return 1
    fi
    
    # Parse target (supports both M.L and M-L formats)
    if [[ "$target" =~ ^([0-9]+)[\.-]([0-9]+)$ ]]; then
        local module_num="${BASH_REMATCH[1]}"
        local lesson_num="${BASH_REMATCH[2]}"
    else
        echo -e "\033[0;31mError: Invalid target format. Use M.L (e.g., 1.1, 01.01).\033[0m" >&2
        return 1
    fi
    
    local lesson_path=$(get_lesson_path "$module_num" "$lesson_num")
    
    if [[ -n "$lesson_path" ]]; then
        echo -e "\033[0;32mNavigating to: $lesson_path\033[0m"
        cd "$lesson_path" || return 1
    fi
}

# Function to show help
show_help() {
    echo -e "\033[0;32mUsage: $(basename "$0") <command> [target]\033[0m"
    echo -e "\033[0;32mCommands:\033[0m"
    echo "  list        : Lists all modules and lessons in the repository."
    echo "  goto <M.L>  : Changes directory to the specified lesson. M is module number, L is lesson number (e.g., 'goto 1.1', 'goto 03.2')."
    echo "  help        : Displays this help message."
    echo -e "\033[0;32mExample:\033[0m"
    echo "  $(basename "$0") list"
    echo "  $(basename "$0") goto 02.5"
    echo -e "\n\033[0;33mTo make 'rust_nav' an alias, add this to your ~/.bashrc or ~/.zshrc:\033[0m"
    echo "  alias rustnav='. $SCRIPT_DIR/rust_nav.sh'"
    echo "  # Or source it to change directory:"
    echo "  alias nav='source $SCRIPT_DIR/rust_nav.sh'"
}

# Main command handling
case "$1" in
    list)
        list_lessons
        ;;
    goto)
        goto_lesson "$2"
        ;;
    help|--help|-h)
        show_help
        ;;
    "")
        show_help
        ;;
    *)
        echo -e "\033[0;31mError: Unknown command '$1'.\033[0m" >&2
        show_help
        exit 1
        ;;
esac