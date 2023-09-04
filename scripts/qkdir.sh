#!/bin/bash

qkdir() {
    # Get the directory from the Rust program
    local dir_to_navigate
    dir_to_navigate=$("$1")
    
    # Check if the directory was found and is not empty
    if [[ -n "$dir_to_navigate" ]]; then
        cd "$dir_to_navigate" || return
    else
        echo "Directory not found."
    fi
}
