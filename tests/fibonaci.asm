; this code calculates the N item of the fibonacci
; sequence. The N item should be moved to CX
org 0x100
mov cl, [item]
cmp cl, 2
jbe LEAVE

LOOP:
    dec cx
    mov ax, [val1]
    mov bx, [val2]
    mov [val2], ax
    add ax, bx
    mov [val1], ax
    cmp cl, [zero]
    jnz LOOP
hlt

LEAVE:
hlt

zero db 0
item db 13
val1 dw 1
val2 dw 0