; this program encrypts a string using rotate
org 0x100

; start string
mov di, string
mov si, encrypt

LOOP:
    mov al, [di]
    add al, [key]
    call CHECK_RANGE
    mov [si], al
    inc di
    inc si
    cmp di, sep
    jne LOOP
hlt

CHECK_RANGE:
    cmp al, 'z'
    jbe CONT
    sub al, [alphabet_len]
    CONT:
    cmp al, 'a'
    jae LEAVE
    add al, [alphabet_len]
    LEAVE: ret

alphabet_len db 26
key db 3
string db "assembly"
sep db "/"
encrypt db 0