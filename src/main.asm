.MODEL SMALL
.STACK



_DATAS SEGMENT PARA PUBLIC 'DATA' USE16

_DATAS ENDS

INCLUDE mem.asm
INCLUDE pixmap.asm

       
_CODE SEGMENT PARA PUBLIC 'CODE' USE16
 ASSUME CS:_CODE, DS:_DATAS

_START:
;initial DS
	mov ax, _DATAS
        mov ds,ax

	call initMem
         
        mov ax, 100h
        call getMem

        mov ax, 0FFFh
        call getMem

        mov ax, 0F00h
        call getMem
		

	mov ah, 00h
	mov al, 13h
	int 10h

;    call MemAlloc
;   call LineTests
   call LoadPixmap
;   call VGAPalette

    mov ah, 0
    int 16h

    mov ax, 03h
    int 10h

    mov ax, 4C00h
    int 21h
_CODE ENDS

END _START



