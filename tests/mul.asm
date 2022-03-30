; Thig program calculates the multiplication of 2 numbers
; without the use of MUL instruction

mov ax, 0
mov cx, [num2]

LOOP:
dec cx
add ax, [num1]
cmp cx, [zero]
jnz LOOP
mov [result], ax
hlt
db 10
zero   dw 0
num1   dw 4
num2  dw 5
result dw 0