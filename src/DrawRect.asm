	.8086
	.MODEL TINY
	.CODE
	.DATA

; cx - x0
; dx - y0
; si - x1
; di - y1
DrawRect proc
        mov ax,di
        sub ax,dx
_next_line:
        cmp ax,0
        je ok_rect        
        dec ax
        push ax
	push cx
	push dx
 	push si
        push di

	add dx,ax
        mov di,dx
        call DrawLine

        pop di
        pop si
        pop dx
        pop cx
        pop ax
        jmp _next_line

ok_rect:
	ret
DrawRect endp

INCLUDE DrawLine.asm

end

