# Turtle -> SVG

A turtle graphics interpreter that outputs SVG

## Example

Generated with the spirals program (included), with these arguments:

```bash
./spiral -i 200 -l 10 -a 91 | ./turtle-svg > spiral.svg
```

![Rust -> Turtle -> SVG](http://owenowen.netsoc.ie/res/turtle/spiral.svg)

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
// Move forward (x: int) pixels
// eg. fd 100
fd x

// Turn left (x: int|float) degrees
// eg. lt 180
lt x

// Turn right (x int|float) degrees
// eg. rt 45
rt x

// Set pen colour to (x: hex-string)
// eg. pc #ffaabb
pc x

// Set pen size to (x: int)
// eg. ps 10
ps x

// Lift up pen (disables drawing)
pu

// Lower pen down (enables drawing)
pd
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
