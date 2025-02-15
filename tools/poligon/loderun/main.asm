.MODEL SMALL
.STACK

INCLUDE hero.asm



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

	mov cx,0FFFFh
	mov bx,10
next_step:
	push cx		

	mov cl,1
	call oneStep
    	mov ah, 0
        int 16h

	mov cl,2
	call oneStep
    	mov ah, 0
        int 16h

	pop cx
	loop next_step



    mov ah, 0
    int 16h

    mov ax, 03h
    int 10h

    mov ax, 4C00h
    int 21h

;->ax - y
;->bx x position
;<-bx x next position
oneStep PROC
	push cx
	mov cl,0
	push ax
	push bx		
	call DrawHero
        pop bx
	pop ax
	pop cx

        add bx,2  
	push ax
	push bx		
	call DrawHero
        pop bx
	pop ax

	ret        
oneStep ENDP	

_CODE ENDS

END _START



