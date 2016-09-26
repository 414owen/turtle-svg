#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
if ! $DIR/dependencies.sh $DIR; then
    exit 0
fi


if ! $DIR/build.sh "$DIR"; then
    exit 0
fi

if ! $DIR/overwrite.sh $DIR; then
    exit 0
fi

echo "How many frames do you want this video to be?"
read frames

echo "At what frame rate?"
read framerate

echo "What animation would you like to produce?"
MODES="Spiral Tree"
select mode in $MODES; do
    if [ "$mode" = "Tree" ]; then
        $DIR/tree.sh $frames $framerate $DIR
        break
    elif [ "$mode" = "Spiral" ]; then
        $DIR/spiral.sh $frames $framerate $DIR
        break
    else
        clear
        echo "Invalid option"
    fi
done


