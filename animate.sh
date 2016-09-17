#!/bin/sh

echo "Testing whether you have release versions of turtle and spiral compiled"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "$DIR"

already_built=true

if [ ! -d $DIR/target/release ]; then
    echo "Target directory not found."
    mkdir -p target/release
fi

if [ ! -f $DIR/target/release/spiral ]; then
    echo "Spiral binary not found"
    already_built=false
fi

if [ ! -f $DIR/target/release/turtle-svg ]; then
    echo "Turtle binary not found"
    already_built=false
fi

if ! $already_built; then
    echo "Not already built, attempting build."
    if ! type cargo > /dev/null; then
        echo "You don't have cargo installed, exiting!"
        exit 1
    else 
        if cargo build --release; then
            echo "Build succesful"
        fi
    fi
else 
    echo "Already built."
fi

if [ -f $DIR/out.mp4 ]; then
    rm $DIR/out.mp4
fi

if ! type bc > /dev/null; then
    echo "Please install 'bc', exiting"
    exit 1
fi 

# This is the logic that makes the spiral changes slow down (to 0)
# when the spiral reaches 90 degree angles this function produces 
# numbers between 0 and 180 it's a sinusoidal generator, with 90 
# being at the peak of the wave, where the direction of speed-up is 
# flipped, to produce the same effect but in reverse, for the other
# 90 degrees.
pi=$(echo "scale=10; 4*a(1)" | bc -l)
halfpi=$(echo "scale=10; $pi / 2" | bc -l)
sineish () {
    points=$1
    echo $points
    step=`echo "scale=10; $pi / $points" | bc -l`
    echo $step
    echo $pi

    for i in `seq 0 $step $halfpi`; do
        echo `echo "scale=8; s($i) * 90" | bc -l`
    done
    for i in `seq $halfpi -$step 0`; do
        echo `echo "scale=8; 180 - (s($i) * 90)" | bc -l`
    done
}

echo "How many frames do you want this video to be?"
read frames

# This looks terrible, sorry, this is how it works:
# turtle graphics are printed, which are piped into the interpreter
# the interpreter spits out svgs
# The svgs are converted to pngs by ImageMagick
# The pngs are converted encoded into h.264 encoded video using FFmpeg

# All this happens without writing a single frame to disk
# Everything occurs in memory
# Praise be unto Unix pipes and complicated shell scripts

{ for i in `sineish $frames`; do
    $DIR/target/release/spiral -g 2 -a $i -i 270 | $DIR/target/release/turtle-svg -w 1000 -h 1000 | convert svg: png:- 
  done 
} | ffmpeg -f image2pipe -r 30 -vcodec png -i - -qscale:v 12  -pix_fmt yuv420p -vcodec libx264 out.mp4

