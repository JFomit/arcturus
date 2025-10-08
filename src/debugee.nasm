
global main
main:
    push    bp
    mov     bp,    sp

    int3

    pop     bp
    ret
