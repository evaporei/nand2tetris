// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// 0 * 0 = 0
// 1 * 0 = 0
// 0 * 2 = 0
// 3 * 1 = 3
// 2 * 4 = 8
// 6 * 7 = 42

// let R0, R1, R2 = 0;



// Put your code here.

// R2 = 0
@R2
M=0

// validate that R0 and/or R1 are not <= 0
@R0
D=M

@END
D;JLE

@R1
D=M

@END
D;JLE

// i = R1
@i
M=D

(LOOP)
@R0
D=M

// R2 = R2 + R0
@R2
M=M+D

// i = i - 1
@i
M=M-1
D=M

// if i == 0, stop loop
@LOOP
D;JNE

// end of program,
// infinite loop
(END)
@END
0;JMP
