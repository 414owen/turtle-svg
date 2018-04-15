#!/usr/bin/env bash

dependencies="bc ffmpeg convert cargo"
DIR=$1

for dep in $dependencies; do
    echo "Checking whether '$dep' is installed"
    if ! type $dep 2> /dev/null > /dev/null; then
        $DIR/color.sh "'$dep' not found, it is required by this script, exiting!" 3
        exit 1
    fi
done

$DIR/color.sh "All dependencies found!" "2"
exit 0
