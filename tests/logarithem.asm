; This program calcualtes the result of a log `base` on `x`
; the result is stored in bx.
; if `x` is 0, bx value is -1 (0xffff)
org 0x100

; leave if the `x` number is 0
mov bx, -1
cmp word [x], 0
je LEAVE

mov ax, 1
mov bx, 0


LOOP:
    cmp ax, [x]
    jbe LEAVE
    mul word [base]
    inc bx
    jmp LOOP
LEAVE:
    hlt

base     dw 10
x        dw 10000

