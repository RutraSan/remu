org 100h
mov ax, 0x100
push 0xabcd
push ax
shr ax, 8
hlt
section .data
var1 DW 0x100
var2 DW 0x90F