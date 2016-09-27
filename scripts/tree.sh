#!/bin/bash

FRAMES=$1
FRAMERATE=$2
DIR=$3

# This looks terrible, sorry, this is how it works:
# Turtle graphics are printed, which are piped into the interpreter
# The interpreter spits out svgs
# The svgs are converted to pngs by ImageMagick
# The pngs are encoded into h.264 encoded video using FFmpeg

# All this happens without writing a single frame to disk
# Everything occurs in memory (or swap, if you need it)
# Praise be unto Unix pipes and complicated shell scripts

read -e -p "What color would you like your leaves to be? " -i "#494" leaf_color
read -e -p "What color would you like your branches to be? " -i "#963" branch_color
read -e -p "What branching factor would you like to use? " -i "6" branch_factor
read -e -p "How many times would you like to branch? (warning, super-duper recursive!) " -i "6" iterations
read -e -p "What angle would you like to use between branches? (dispersion factor) " -i "25" base_angle
read -e -p "What length would you like the trunk to be? (affects other branches) " -i "270" trunk_length
read -e -p "What would you like the ratio between branches to be? " -i "0.8" branch_ratio
read -e -p "Select some keyframes (keyframes are skewing angles, interspersed with booleans which specify whether to ease in / out the keyframes): " -i "0 true true 10 true true -2 true true 20 true true 5 true true 40 true true 0" keyframe_input

echo "Alright, generating keyframes"
KEYFRAMES=`$DIR/keyframes.sh $FRAMES "$keyframe_input"`
$DIR/color.sh "Generated $FRAMES keyframes" 2

{ for i in $KEYFRAMES; do
    $DIR/../target/release/tree -c --leaf-color $leaf_color --branch-color $branch_color -b $branch_factor -i $iterations -a $base_angle -l $trunk_length -r $branch_ratio -p 960,1080 -s $i | $DIR/../target/release/turtle-svg -w 1920 -h 1080 | convert svg: png:- 
  done
} | ffmpeg -hwaccel vaapi -f image2pipe -r $FRAMERATE -vcodec png -i - -c:v libx264 -pix_fmt yuv420p -preset medium -crf 18 $DIR/../out.mp4
