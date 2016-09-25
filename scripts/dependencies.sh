#!/bin/sh

#---------------
# Dependencies
#---------------

dependencies="bc ffmpeg convert cargo"

for dep in $dependencies; do
    echo "Checking whether '$dep' is installed"
    if ! type $dep > /dev/null; then
        echo "'$dep' not found, it is required by this script, exiting!"
        exit 1
    fi
done

echo "All dependencies found, Congratulations!"
exit 0
