@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_1
D;JEQ
@ELSE_1
0;JMP
(IF_1)
@SP
A=M-1
M=-1
@END_1
0;JMP
(ELSE_1)
@SP
A=M-1
M=0
(END_1)
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_2
D;JEQ
@ELSE_2
0;JMP
(IF_2)
@SP
A=M-1
M=-1
@END_2
0;JMP
(ELSE_2)
@SP
A=M-1
M=0
(END_2)
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_3
D;JEQ
@ELSE_3
0;JMP
(IF_3)
@SP
A=M-1
M=-1
@END_3
0;JMP
(ELSE_3)
@SP
A=M-1
M=0
(END_3)
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_4
D;JLT
@ELSE_4
0;JMP
(IF_4)
@SP
A=M-1
M=-1
@END_4
0;JMP
(ELSE_4)
@SP
A=M-1
M=0
(END_4)
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_5
D;JLT
@ELSE_5
0;JMP
(IF_5)
@SP
A=M-1
M=-1
@END_5
0;JMP
(ELSE_5)
@SP
A=M-1
M=0
(END_5)
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_6
D;JLT
@ELSE_6
0;JMP
(IF_6)
@SP
A=M-1
M=-1
@END_6
0;JMP
(ELSE_6)
@SP
A=M-1
M=0
(END_6)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_7
D;JGT
@ELSE_7
0;JMP
(IF_7)
@SP
A=M-1
M=-1
@END_7
0;JMP
(ELSE_7)
@SP
A=M-1
M=0
(END_7)
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_8
D;JGT
@ELSE_8
0;JMP
(IF_8)
@SP
A=M-1
M=-1
@END_8
0;JMP
(ELSE_8)
@SP
A=M-1
M=0
(END_8)
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=M-D
@IF_9
D;JGT
@ELSE_9
0;JMP
(IF_9)
@SP
A=M-1
M=-1
@END_9
0;JMP
(ELSE_9)
@SP
A=M-1
M=0
(END_9)
@57
D=A
@SP
A=M
M=D
@SP
M=M+1
@31
D=A
@SP
A=M
M=D
@SP
M=M+1
@53
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M+D
@112
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M-D
@SP
A=M-1
M=-M
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M&D
@82
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M|D
@SP
A=M-1
M=!M
