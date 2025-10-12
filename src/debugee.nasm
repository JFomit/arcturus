bits    16

global main
main:
    push    bp
    mov     bp,    sp

    call    dword example

    pop     bp
    o32 ret

global  example
example:
    push    bp
    mov     bp,    sp

    mov     ah,     02h
    mov     dl,     'J'
    int     21h
    
    pop     bp
    o32 retd
