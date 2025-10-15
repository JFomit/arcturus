bits    16

CANARY  equ 0xFEEDDEAD

extern	_exit

section	.text
global main
main:
    push    bp
    mov     bp,     sp

    call    example

    mov     sp,     bp
    pop     bp
    xor     ax,     ax
    o32 ret

global  example
example:
    mov     eax,    CANARY
    push    eax

    mov     ah,     02h
    mov     dl,     'J'
    int     21h

    pop     eax
	cmp		eax,	CANARY
	je		.E

	mov		ah,		09h
	mov		dx,		abort_msg
	int		21h

	mov		eax,	8
	call	dword _exit

.E:	ret


section	.data
abort_msg	db	"Stack smash detected.", 13, 10, '$'
