#!/bin/bash

#---------------
# Builds
#---------------

echo "Testing whether you have release versions of turtle and spiral compiled"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "$DIR"

already_built=true

if [ ! -d $DIR/../target/release ]; then
    echo "Target directory not found, creating it now"
    mkdir -p target/release
fi

if [ ! -f $DIR/../target/release/spiral ]; then
    echo "Spiral binary not found"
    already_built=false
elif [ ! -f $DIR/../target/release/turtle-svg ]; then
    echo "Turtle binary not found"
    already_built=false
elif [ ! -f $DIR/../target/release/turtle-svg ]; then
    echo "Tree binary not found"
    already_built=false
fi

if ! $already_built; then
    echo "Attempting to compile turtle-svg, tree, and spiral (as needed)"
    if cargo build --release; then
        echo "Build succesful"
    else
        echo "Couldn't build project, exiting!"
        exit 1
    fi
else 
    echo "Already built"
fi

exit 0
