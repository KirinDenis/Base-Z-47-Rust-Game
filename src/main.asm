	.8086
	.MODEL TINY
	.CODE
	.DATA
	.STARTUP
	ORG 100h

main proc

    mov ah, 00h
    mov al, 13h
    int 10h

;   call LineTests
    call LoadBitmap
;   call VGAPalette

    mov ah, 0
    int 16h

    mov ax, 03h
    int 10h


    mov ax, 4C00h
    int 21h
main endp

INCLUDE bitmap.asm
INCLUDE VGAPal.asm
INCLUDE test.asm

end 
