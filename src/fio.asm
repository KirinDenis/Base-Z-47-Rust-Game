.MODEL SMALL


_CODE SEGMENT PARA PUBLIC 'CODE' USE16
 ASSUME CS:_CODE, DS:_DATAS

;-> dx offset of file name ASCIIZ
;<- CF if error
;<- ds:dx data
LoadFile proc

	mov ax,3D00h		; open file to read
	int 21h
	mov bx,ax		; bx opened file handle

        xor cx,cx		; prepare get file size, clean file offset
        xor dx,dx		
	mov ax, 4202h		; get file size (move file pointer to end of file)
	int 21h       		; return dx:ax file size
	                        
        mov cx,ax		; rounding file size to paragraph size (div 16_
        push cx			; store file size to stack
	push bx

        add cx,10h 		; paragrash size division round
	shr cx,4 		; divide file size to 16


	mov ax, 4A00h 		; ask DOS to extend segment       
	mov bx, cx              ; bx size of needed memory in paragraph 
	int 21h  


	mov ax, 4800h 		; ask new segment memory block       
	mov bx, cx         	; bx size of needed memory in paragraph 
	int 21h  
        jc _error          
        mov ds,ax 		; extended data segment

        pop bx 			; restore file descriptor to bx

	mov ax,4200h		; file offset to begining of file 
	xor dx,dx 		; clean file offset
	xor cx,cx 
	int 21h  

        pop cx  		; file in bytes to read file data to new memory block
	mov ax, 3F00h  		; read file to memory, memory block address ds:dx
        int 21h

_error:
	mov ax, 3E00h  		; close file 
        int 21h  

        ret

LoadFile ENDP
_CODE ENDS

_DATAS SEGMENT PARA PUBLIC 'DATA' USE16
_DATAS ENDS



