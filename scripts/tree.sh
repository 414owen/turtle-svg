#!/bin/bash

FRAMES=$1
FRAMERATE=$2
DIR=$3
KEYFRAMES=`./keyframes.sh $FRAMES "0 true true 10 true true -10 true true 20 true true 5 true true 40"`

# This looks terrible, sorry, this is how it works:
# Turtle graphics are printed, which are piped into the interpreter
# The interpreter spits out svgs
# The svgs are converted to pngs by ImageMagick
# The pngs are encoded into h.264 encoded video using FFmpeg

# All this happens without writing a single frame to disk
# Everything occurs in memory (or swap, if you need it)
# Praise be unto Unix pipes and complicated shell scripts

echo "What color would you like your leaves to be? (in hex, eg. '#4b4')"
read leaf_color

echo "What color would you like your branches to be? (in hex, eg. '#963')"
read branch_color

{ for i in $KEYFRAMES; do
    $DIR/../target/release/tree -c --leaf-color $leaf_color --branch-color $branch_color -b 6 -i 6 -a 25 -l 230 -p 960,1080 -s $i | $DIR/../target/release/turtle-svg -w 1000 -h 1000 | convert svg: png:- 
  done
} | ffmpeg -hwaccel vaapi -f image2pipe -r $FRAMERATE -vcodec png -i - -c:v libx264 -pix_fmt yuv420p -preset medium -crf 18 ./../out.mp4
