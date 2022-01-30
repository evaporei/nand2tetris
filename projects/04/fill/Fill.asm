// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

// Algorithm:
// // infinite loop
// 'start: loop {
// 	// zero if not pressed
// 	let input = keyboard();
// 
// 	// draws once at the whole screen for a keypress
// 	for pixel in SCREEN..KBD {
// 		if input == 0 {
// 			draw(pixel, WHITE);
// 		} else {
// 			draw(pixel, BLACK);
// 		}
// 	}
// }

// vars
// - keyboard: will hold the keyboard input for the whole drawing loop
// - pixel: holds the pixel, it's the incremental `i` of the drawing loop

// infinite loop, look at the end of the file
(STARTLOOP)
// setup:
@KBD
D=A
// save in DRAWLOOP address the value of KBD,
// aka 1 address after the end of the screen
@DRAWLOOP
M=D

// let's read the keyboard for real
@KBD
D=M

// save the keyboard input in a variable
@keyboard
M=D

// point to the beginning of the screen memory map
@SCREEN
D=A
// @24573 // for debugging

// pixel=i
@pixel
M=D

// draw loop:
(DRAWLOOP)

// point to the next memory map address
// set in D in the end of the last loop
// iteration

// get keyboard press
@keyboard
D=M

// should write BLACK or WHITE?
@WHITE
D;JEQ

// get the pixel to write that got set
// in the last loop iteration
@pixel
A=M
// black line
M=-1

// escape white draw
@ELSE
0;JMP

(WHITE)
// get the pixel to write that got set
// in the last loop iteration
@pixel
A=M
// white line
M=0

(ELSE)
// store next pixel location
@pixel
// go to next screen memory map address
M=M+1
// store current pixel in D to do a jump
D=M

// if pixel == end of screen, stop drawing
// it will go back to the start of the file
// to read the keyboard again
@DRAWLOOP
M-D;JNE

// infinite loop
// end of program
// back to the beginning
@STARTLOOP
0;JMP
