	.8086
	.MODEL TINY
	.CODE
        .DATA
	
VGAPalette proc
	mov ax, 0A000h
        mov es,ax

	mov ax,0
        mov bx,0
        mov cx,0

next_color:
        cmp ax,15
        jne _next_x
        cmp bx,15
        je OK
        mov ax,0   
        inc bx
        jmp next_rect
_next_x:
        inc ax
next_rect:
        inc cx
        
        push ax
        push bx
        push cx

        mov bp,cx

        shl ax,3
        shl bx,3
       
        mov cx,ax
        mov dx,bx

        mov si,ax
        add si,8
        mov di,bx
        add di,8
        
        
        call DrawRect
        
        pop cx
        pop bx
        pop ax
        jmp next_color
OK:
	ret
VGAPalette endp

INCLUDE drawrect.asm
end

