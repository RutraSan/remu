org 100h
xor ax, ax
mov es, ax

; get right interupt address
mov bx, 33h
mov ax, 4
mul bx
mov bx, ax

; set the interupt
mov ax, cs
mov es:[bx], ax
add bx, 2
mov word es:[bx], power

; call the `power` program
mov ax, 2
mov cx, 8
int 33h
hlt

; calcualte the power of ax by cx
power:
    push bx ; save bx
    mov bx, ax
    dec cx
    loop: 
        dec cx
        mul bx
    cmp cx, 0
    jne loop
    pop bx ; retrive bx
    iret