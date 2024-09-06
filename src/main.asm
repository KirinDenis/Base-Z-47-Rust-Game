.MODEL SMALL
.STACK

INCLUDE heap.asm
INCLUDE mem.asm
INCLUDE pixmap.asm

_DATAS SEGMENT PARA PUBLIC 'DATA' USE16
_DATAS ENDS

       
_CODE SEGMENT PARA PUBLIC 'CODE' USE16
 ASSUME CS:_CODE, DS:_DATAS

_START:
;initial DS
	mov ax, _DATAS
        mov ds,ax

	mov ax,0A000h
        mov es,ax

         		
	mov ah, 00h
	mov al, 13h
	int 10h


	mov ax, 100
        call GetHeap

	mov ax, 16
        call GetHeap

	mov ax, 0F000h
        call GetHeap



;   call LoadPixmap



;   call VGAPalette

    mov ah, 0
    int 16h

    mov ax, 03h
    int 10h

    mov ax, 4C00h
    int 21h


_CODE ENDS

END _START



