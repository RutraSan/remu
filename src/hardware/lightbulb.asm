; the interrupt code
cmp ah, 0
jz POWER
cmp ah, 1
jz COLOR
iret ; doing nothing
POWER:
    out 0xbb, al
    iret 
COLOR:
    out 0xbc, al
    iret