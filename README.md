# Turtle SVG

A turtle graphics interpreter that outputs SVG

## Example

Generated with the spiral program (included, found in target/release/ once  
built), with these arguments:

```bash
./spiral -i 270 -g 3 -a 121 | ./turtle-svg -w 1000 -h 1000 > spiral.svg
```

![Rust -> Turtle -> SVG](http://owenowen.netsoc.ie/res/turtle/spiral.svg)

I also made a video from 12000 values for 'angle' between 0 and 180, [here](https://www.youtube.com/watch?v=fY_KRJhCVKk).

## To build

```bash
cargo build --release
```

This will put turtle-svg and all generator programs in `target/release/`.

## Commands

All commands must be on a newline. All parameters must be space-separated.  
This is not a forgiving interpreter. If your script is wrong, expect a line  
number, but not much else. Everything is rendered onto a 500 * 500 SVG canvas.  

```
// Lift up pen (disables drawing)
pu

// Lower pen down (enables drawing)
pd

// Move forward (x: float|int) pixels
// eg. fd 100
fd x

// Turn left (x: float|int) degrees
// eg. lt 180
lt x

// Turn right (x float|int) degrees
// eg. rt 45
rt x

// Set pen colour to (x: hex|rgb|rgba)
// eg. pc #ffaabb
pc x

// Set pen size to (x: float|int)
// eg. ps 10
ps x

// Draw circle of radius (x: float|int)
// eg ci 100
ci 100
```

## How to use

### From a Turtle script

By default, the program reads from stdin and outputs to stdout. If you  
have a script file called 'test.turt' you can create an svg like this  
(unix-like only):

```bash
./turtle-svg < test.turt > out.svg
```

Alternatively, you can specify an input and output file with '-i' and  
'-o'.

```bash
./turtle-svg -i test.turt -o out.svg
```

### With a turtle script generator

If you have a program that generates turtle script, you can simply pipe  
the output into turtle-svg:

```bash
python my-fancy-script.py | ./turtle-svg > out.svg
```
