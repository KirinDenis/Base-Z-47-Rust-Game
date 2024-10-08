.MODEL SMALL

_DATAS SEGMENT PARA PUBLIC 'DATA' USE16

pose0	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b

pose1	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00001000b
	db 00011100b
	db 00011100b
	db 00011000b
	db 01111010b
	db 11011110b
	db 00011000b
	db 00111100b
	db 01100110b
	db 11000110b
	db 11000000b

pose2	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000000b
	db 00000100b
	db 00001110b
	db 00001110b
	db 00011100b
	db 01111100b
	db 11011010b
	db 00011011b
	db 00011100b
	db 11110110b
	db 00000110b


_DATAS ENDS

_CODE SEGMENT PARA PUBLIC 'CODE' USE16
 ASSUME CS:_CODE, DS:_DATAS

;-> ax - y
;-> bx - x
;-> cl pose 
DrawHero PROC  
	xor ch,ch	
	shl cl,4	
	mov di, offset ds:pose0
        add di,cx 
        mov dx,320
        mul dx
        add bx,ax       
	mov cx,16	
_next_line:
        push cx
	mov cx,8
        mov al, byte ptr ds:[di]
next_pixel:        
	test al, 10000000b	
        jnz _pixel
	mov byte ptr es:[bx], 000h
	jmp _ok
_pixel: 	
        mov byte ptr es:[bx], 00Fh
_ok:
	shl al,1
        inc bx
        loop next_pixel
	pop cx
        inc di
	add bx,320-8
        loop _next_line
        ret        
DrawHero ENDP
_CODE ENDS