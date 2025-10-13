bits    16

extern  printDword

global main
main:
    push    bp
    mov     bp,     sp

    ; mov     edx,    [esp]
    ; call    printDword
    call    example
    ; mov     edx,    [esp]
    ; call    printDword

    mov     sp,     bp
    pop     bp
    xor     ax,     ax
    o32 ret

global  example
example:
    mov     ah,     02h
    mov     dl,     'J'
    int     21h
    
    ret
