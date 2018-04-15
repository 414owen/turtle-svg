#!/usr/bin/env bash

DIR=$1

if [ -f $DIR/../out.mp4 ]; then
    read -p "out.mp4 already exists, overwrite it? (y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        read -p "Are you sure? out.mp4 looks like a good video to me! (y/n) " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm $DIR/../out.mp4
            exit 0
        else
            exit 1
        fi
    else
        exit 1
    fi
else
    $DIR/color.sh "out.mp4 not found" 2
fi
