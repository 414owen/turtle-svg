#!/bin/bash

#---------------
# Overwrite
#---------------

if [ -f $DIR/out.mp4 ]; then
    read -p "out.mp4 already exists, overwrite it? (y/n) " -n 1 -r
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        read -p "Are you sure? out.mp4 looks like a good video to me! (y/n)" -n 1 -r
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm $DIR/out.mp4
        fi
    else
        exit 1
    fi
fi

exit 0
