#!/usr/bin/env bash

FRAMES=$1
FRAMERATE=$2
DIR=$3
KEYFRAMES=`$DIR/keyframes.sh $FRAMES "0 false false 180"`
#echo "$FRAMES $FRAMERATE

{ for i in $KEYFRAMES; do
    $DIR/../target/release/spiral -g 2 -a $i -i 270 | $DIR/../target/release/turtle-svg -w 1920 -h 1080 | convert svg: png:- 
  done
} | ffmpeg -hwaccel vaapi -f image2pipe -r $FRAMERATE -vcodec png -i - -c:v libx264 -pix_fmt yuv420p -preset medium -crf 18 $DIR/../out.mp4
