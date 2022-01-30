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
// [x] make a part of the screen black
// [x] make write black loop
// [x] stop at the final address
// [x] graceful stop hehe
// [ ] make a part of the screen black if keyboard pressed
// [ ] make the whole screen blink based of keyboard input

// setup:
@KBD
D=A
// save in LOOP address the value of KBD,
// aka 1 address after the end of the screen
@LOOP
M=D

// point to the beginning of the screen memory map
@SCREEN
// @24573 // for debugging
D=A

// loop:
(LOOP)

// point to the next memory map address
// set in D in the end of the last loop
// iteration
A=D

// black line
M=-1

// go to next screen memory map address
D=D+1

// stop at the end of the screen memory map
@LOOP
M-D;JNE

// infinite loop
// end of program
@END
0;JMP
