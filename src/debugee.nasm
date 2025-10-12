bits    16

global main
main:
    push    bp
    mov     bp,     sp

    call    example

    mov     sp,     bp
    pop     bp
    xor     ax,     ax
    ret

global  example
example:
    mov     ah,     02h
    mov     dl,     'J'
    int     21h
    
    ret
