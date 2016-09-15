# Turtle Graphics -> SVG
A turtle graphics interpreter that outputs SVG

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
```

