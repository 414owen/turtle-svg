#!/bin/bash

FRAMES=$1
FRAMERATE=$2
DIR=$3
KEYFRAMES=`./keyframes.sh $FRAMES "0 false false 180"`
#echo "$FRAMES $FRAMERATE

# This looks terrible, sorry, this is how it works:
# Turtle graphics are printed, which are piped into the interpreter
# The interpreter spits out svgs
# The svgs are converted to pngs by ImageMagick
# The pngs are encoded into h.264 encoded video using FFmpeg

# All this happens without writing a single frame to disk
# Everything occurs in memory (or swap, if you need it)
# Praise be unto Unix pipes and complicated shell scripts

{ for i in $KEYFRAMES; do
    $DIR/../target/release/spiral -g 2 -a $i -i 270 | $DIR/../target/release/turtle-svg -w 1920 -h 1080 | convert svg: png:- 
  done
} | ffmpeg -hwaccel vaapi -f image2pipe -r $FRAMERATE -vcodec png -i - -c:v libx264 -pix_fmt yuv420p -preset medium -crf 18 ./../out.mp4
