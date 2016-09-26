#!/bin/bash

#---------------
# Builds
#---------------

echo "Testing for builds"

DIR=$1

already_built=true

if [ ! -d "$DIR/../target/release" ]; then
    echo "Target directory not found, creating it now"
    mkdir -p "$DIR/../target/release"
fi

if [ ! -f "$DIR/../target/release/spiral" ]; then
    echo "Spiral binary not found"
    already_built=false
elif [ ! -f "$DIR/../target/release/turtle-svg" ]; then
    echo "Turtle binary not found"
    already_built=false
elif [ ! -f "$DIR/../target/release/turtle-svg" ]; then
    echo "Tree binary not found"
    already_built=false
fi

if ! $already_built; then
    echo "Attempting to compile turtle-svg, tree, and spiral (as needed)"
    if cargo build --release; then
        echo "Build succesful!"
    else
        echo "Couldn't build project, exiting!"
        exit 1
    fi
else 
    $DIR/color.sh "Project already built!" 2
fi

exit 0
