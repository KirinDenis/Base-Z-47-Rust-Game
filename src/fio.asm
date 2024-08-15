	.8086
	.MODEL TINY
	.CODE
	.DATA

;-> dx offset of file name ASCIIZ
;<- CF if error
;<- ds:dx data
LoadFile proc

	mov ax,3D00h ; open for input	
	int 21h
	mov bx,ax ; ax = bx = opened file handle

	mov ax, 4202h ; get file size
	int 21h       ; bx handle

; ?X:DX
; ?X low
; DX hight
        mov cx,ax 
        push cx ;file size
	push bx

	shr cx,4 ;11

	mov ax, 4A00h ; ask DOS to extend segment       
	mov bx, cx         
	int 21h  


	mov ax, 4800h ; ask new segment memory block       
	mov bx, cx         
	int 21h  
        jc _error          
        mov ds,ax ;data segmenta

        pop bx ; file descriptor



	mov ax,4200h ;file offset to begining of file 
	xor dx,dx 
	xor cx,cx 
	int 21h  

        pop cx

	mov ax, 3F00h  ;load bitmap data from file  ds:dx
        ;lea dx, bfType
        int 21h

_error:
	mov ax, 3E00h  ;close file 
        int 21h  

LoadFile endp
end
