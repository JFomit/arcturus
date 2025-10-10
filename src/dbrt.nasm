	BITS    16

; Saves contents of the registers into a buffer, pointed to by its first argument.
; The order is:
; eax, ecx, edx, ebx, esp, ebp, esi, edi, eip, eflags;
; cs, ss, ds, es, fs, gs;
; <whatever `fxsave' instruction order is>.
;
; The buffer must be at least 576 = 64 + 512 bytes long and also must be 16-byte aligned.
;
; extern "C" fn save_registers(target: *mut u8) -> ()
global  save_registers
save_registers:
	; no standard prologue & epilogue - we are saving registers, after all
	push	esp
	push	ebx
	mov		ebx,		[esp+12]	; target

	mov		[ebx],		eax			; saved eax
	mov		[ebx+4],	ecx			; saved ecx
	mov		[ebx+8],	edx			; saved edx

	pop		eax						; eax <- old ebx
	mov		[ebx+12],	eax			; saved ebx

	pop		esp
	mov		[ebx+16],	esp			; saved esp
	mov		[ebx+20],	ebp			; saved ebp
	mov		[ebx+24],	esi			; saved esi
	mov		[ebx+28],	edi			; saved edi

	mov		eax,		[esp]
	mov		[ebx+32],	eax			; saved eip

	pushfd
	mov		eax,		[esp]
	popfd
	mov		[ebx+36],	eax			; saved eflags
	
	mov		[ebx+40],	cs			; saved cs
	mov		[ebx+42],	ss			; saved ss
	mov		[ebx+44],	ds			; saved ds
	mov		[ebx+46],	es			; saved es
	mov		[ebx+48],	fs			; saved fs
	mov		[ebx+50],	gs			; saved gs

	fxsave	[ebx+64]				; saved fcw, fsw, ftw (abridged), fop, fip, fcs, fdp, fds, mxcsr, mxcsr_mask, st0-st7, xmm0-xmm7

	ret

; Restores contents of the registers from a buffer, which was filled with `save_registers(target: *mut u8)' function.
;
; extern "C" fn restore_registers(target: *mut u8) -> !
global restore_registers
restore_registers:
	; no standard prologue & epilogue - we are restoring registers, after all
	mov		ebx,	[esp+4]				; target

	fxrstor	[ebx+64]					; restore fcw, fsw, ftw (abridged), fop, fip, fcs, fdp, fds, mxcsr, mxcsr_mask, st0-st7, xmm0-xmm7

	mov		cs,		[ebx+40]			; restored cs
	mov		ss,		[ebx+42]			; restored ss
	mov		ds,		[ebx+44]			; restored ds
	mov		es,		[ebx+46]			; restored es
	mov		fs,		[ebx+48]			; restored fs
	mov		gs,		[ebx+50]			; restored gs

	mov		eax,	[ebx+36]
	push	eax
	popfd								; restored eflags

	mov		ebp,	[ebx+20]			; restored ebp
	mov		esi,	[ebx+24]			; restored esi
	mov		edi,	[ebx+28]			; restored edi

	mov		ecx,	[ebx+4]				; restored ecx
	mov		edx,	[ebx+8]				; restored edx

	mov		esp,	[ebx+16]			; restored esp, trashing everything on the current stack in the process }:)

	mov		eax,	[ebx+32]
	push	eax							; pushed the correct eip value
	mov		eax,	[ebx+12]
	push	eax							; pushed the correct ebx value
	mov		eax,	[eax]
	push	eax							; pushed the correct eax value

	pop		eax							; restored eax
	pop		ebx							; restored ebx
	ret									; restored eip


extern	OLD_INT3_HANDLER
extern	int3_handler

global set_handlers
set_handlers:
	xor		eax,				eax			; IVT at address 0
	mov		edx,				[eax+12]	; IVT int3 handler	
	mov		[OLD_INT3_HANDLER],				edx

	mov		dword [eax+12],		int3_handler

	ret

global clear_handlers
clear_handlers:
	xor		eax,				eax
	mov		edx,				[OLD_INT3_HANDLER]
	mov		[eax+12],			edx

	ret