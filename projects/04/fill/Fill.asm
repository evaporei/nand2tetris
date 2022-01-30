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

// R0 will hold the keyboard input for the whole drawing loop
// R1 holds the pixel, it's the incremental `i` of the drawing loop

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

// put in register 0 the keyboard value
@R0
M=D

// point to the beginning of the screen memory map
@SCREEN
D=A
// @24573 // for debugging

// R1=i
@R1
M=D

// draw loop:
(DRAWLOOP)

// point to the next memory map address
// set in D in the end of the last loop
// iteration

// get keyboard press from R0
@R0
D=M

// should write BLACK or WHITE?
@WHITE
D;JEQ

// get the pixel to write that got set in R1
// in the last loop iteration
@R1
A=M
// black line
M=-1

// escape white draw
@ELSE
0;JMP

(WHITE)
// get the pixel to write that got set in R1
// in the last loop iteration
@R1
A=M
// white line
M=0

(ELSE)
// store next pixel in R1
@R1
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
