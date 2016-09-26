#!/bin/bash

if ! ./dependencies.sh; then
    exit 0
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if ! ./build.sh "$DIR"; then
    exit 0
fi

if ! ./overwrite.sh $DIR; then
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
        ./tree.sh
        break
    elif [ "$mode" = "Spiral" ]; then
        ./spiral.sh
        break
    else
        clear
        echo "Invalid option"
    fi
done


