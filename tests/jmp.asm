add dx, [var]

mov cx, 5
label:
dec cx
cmp cl, [zero]
jnz label
hlt

zero db 0
var dw 0x20