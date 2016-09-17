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

rm $DIR/out.mp4

if ! type bc > /dev/null; then
    echo "Please install 'bc', exiting"
    exit 1
fi

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


{ for i in `sineish 500`; do
    $DIR/target/release/spiral -g 2 -a $i -i 200 | $DIR/target/release/turtle-svg | convert svg: png:- 
  done 
} | ffmpeg -f image2pipe -r 30 -vcodec png -i - -vcodec libx264 out.mp4

