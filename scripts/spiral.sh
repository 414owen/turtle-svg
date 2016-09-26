#!/bin/bash

echo "How many frames do you want this video to be?"
read frames

echo "At what frame rate?"
read framerate

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
