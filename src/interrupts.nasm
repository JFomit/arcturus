bits	16

global	old_int3
global	old_int1

global	set_interrupt_handlers
global	remove_interrupt_handlers
; DosTarget
extern	DOS_TARGET
; __cdecl fn break_handler() -> ()
extern	break_handler
; __cdecl fn step_over_handler() -> ()
extern	step_over_handler

section	.text

set_interrupt_handlers:
	push	bx
	push	es
	; saving old int3 handler
	mov		ah,				35h ; Get interrupt handler
	mov 	al,				0x3
	int 	21h
	mov 	[old_int3],		bx
	mov 	[old_int3+2],	es
	; setting new int3 handler
	; ds is already set
	mov		dx,				int3_handler
	mov 	ah,				25h ; Set interrupt handler
	mov 	al,				0x3
	int 	21h

	pop		es
	pop		bx
	ret

remove_interrupt_handlers:
	push	ds

	; restoring old int3 handler
	mov 	[old_int3],		dx
	mov 	[old_int3+2],	ds

	mov 	ah,				25h ; Set interrupt handler
	mov 	al,				0x3
	int 	21h
	
	pop		ds
	ret

int3_handler:
	; flags - 36
	; CS:IP - 32
	; 28   24   20   16   12                    8    4        0
	; EAX, ECX, EDX, EBX, ESP (original value), EBP, ESI, and EDI
	pushad
	dec			word [esp+32]					; fixing eip to point before the int3

	; pushf
	; call		 dword [cs:old_int3]
	; saving registers to DOS_TARGET
	mov			eax,				[esp+28]
	mov			[DOS_TARGET+0],		eax			; eax
	mov			eax,				[esp+24]
	mov			[DOS_TARGET+4],		eax			; ecx
	mov			eax,				[esp+20]
	mov			[DOS_TARGET+8],		eax			; edx
	mov			eax,				[esp+16]
	mov			[DOS_TARGET+12],	eax			; ebx
	mov			eax,				[esp+12]
	mov			[DOS_TARGET+16],	eax			; esp
	mov			eax,				[esp+8]
	mov			[DOS_TARGET+20],	eax			; ebp
	mov			eax,				[esp+4]
	mov			[DOS_TARGET+24],	eax			; esi
	mov			eax,				[esp+0]
	mov			[DOS_TARGET+28],	eax			; edi

	movzx		eax,				word [esp+36]
	mov			[DOS_TARGET+36],	eax			; eflags
	movzx		eax,				word [esp+34]
	mov			[DOS_TARGET+40],	ax			; cs
	movzx		eax,				word [esp+32]
	mov			[DOS_TARGET+32],	eax			; eip

	mov			ax,					ss
	mov			[DOS_TARGET+42],	ax			; ss
	mov			ax,					ds
	mov			[DOS_TARGET+44],	ax			; ds
	mov			ax,					es
	mov			[DOS_TARGET+46],	ax			; es
	mov			ax,					fs
	mov			[DOS_TARGET+48],	ax			; fs
	mov			ax,					gs
	mov			[DOS_TARGET+50],	ax			; gs

	call  		dword break_handler

	popad
	iret
int1_handler:
	iret

section .data

old_int3	dd	0
old_int1	dd	0
hexChars	db	"0123456789abcdef",0

section	.text

global	printDword
; ARGS: edx -- number to be printed
; CLOBBERS: eflags,edx
printDword:
	push	eax
	push	ebx
	push	ecx
	push	esi
	push	edx

	mov		ah,	2
	mov		dl,	13
	int		21h
	mov		dl,	10
	int		21h
	
	pop		edx

	mov		cx, 	8 ; tetrade count
	mov		ax, 	0x0200
	xor		bx, 	bx

.loop rol 	edx, 	4 ; rotate 1 tetrade
	mov		bl, 	dl ; copy 4 LSBs to bl
	and		bx, 	0x000f
	mov		si, 	hexChars
	add		si, 	bx
	mov		dl, 	[si]

	int 	0x21

	dec 	cx
	jnz 	.loop 
	push	edx

	mov		ah,	2
	mov		dl,	13
	int		21h
	mov		dl,	10
	int		21h

	pop		edx

	pop 	esi
	pop 	ecx
	pop 	ebx
	pop 	eax

	ret
