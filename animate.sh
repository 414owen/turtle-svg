#!/bin/bash

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

#---------------
# Builds
#---------------

echo "Testing whether you have release versions of turtle and spiral compiled"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "$DIR"

already_built=true

if [ ! -d $DIR/target/release ]; then
    echo "Target directory not found, creating it now"
    mkdir -p target/releaseA
fi

if [ ! -f $DIR/target/release/spiral ]; then
    echo "Spiral binary not found"
    already_built=false
elif [ ! -f $DIR/target/release/turtle-svg ]; then
    echo "Turtle binary not found"
    already_built=false
fi

if ! $already_built; then
    echo "Attempting to compile turtle-svg and spiral"
    if cargo build --release; then
        echo "Build succesful"
    else
        echo "Couldn't build project, exiting!"
        exit 1
    fi
else 
    echo "Already built"
fi

if [ -f $DIR/out.mp4 ]; then
    read -p "out.mp4 already exists, overwrite it? (y/n) " -n 1 -r
    echo    # (optional) move to a new line
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        rm $DIR/out.mp4
    else
        exit 0
    fi
fi

# This is the logic that makes the spiral changes slow down (to 0)
# when the spiral reaches 90 degree angles. This function produces 
# numbers between 0 and 180. It's a sinusoidal generator, with 90 
# being at the peak of the sine wave. It slows down until it's at
# 90, then speeds up again.

pi=$(echo "scale=10; 4*a(1)" | bc -l)
halfpi=$(echo "scale=10; $pi / 2" | bc -l)
sineish () {
    points=$1
    echo $points
    step=`echo "scale=10; $pi / $points" | bc -l`
    echo $step
    echo $pi

    for i in `seq 0 $step $halfpi`; do
        echo `echo "scale=5; s($i) * 90" | bc -l`
    done
    for i in `seq $halfpi -$step 0`; do
        echo `echo "scale=5; 180 - (s($i) * 90)" | bc -l`
    done
}

echo "How many frames do you want this video to be?"
read frames

echo "At what frame rate?"
read framerate

PROGRESSIONS="linear sinosoidal"
echo "Linear or sinosoidal progression?"
select progression in $PROGRESSIONS; do
    if [ "$progression" = "linear" ]; then
        keyframes=`seq 0 $(echo "scale=5; 180 / $frames" | bc -l) 180`
        break
    elif [ "$progression" = "sinosoidal" ]; then
        keyframes=`sineish $frames`
        break
    else
        clear
        echo "Invalid option"
    fi
done

# This looks terrible, sorry, this is how it works:
# Turtle graphics are printed, which are piped into the interpreter
# The interpreter spits out svgs
# The svgs are converted to pngs by ImageMagick
# The pngs are encoded into h.264 encoded video using FFmpeg

# All this happens without writing a single frame to disk
# Everything occurs in memory (or swap, if you need it)
# Praise be unto Unix pipes and complicated shell scripts

{ for i in $keyframes; do
    $DIR/target/release/spiral -g 2 -a $i -i 270 | $DIR/target/release/turtle-svg -w 1000 -h 1000 | convert svg: png:- 
  done
} | ffmpeg -hwaccel vaapi -f image2pipe -r $framerate -vcodec png -i - -c:v libx264 -pix_fmt yuv420p -preset medium -crf 18 out.mp4
