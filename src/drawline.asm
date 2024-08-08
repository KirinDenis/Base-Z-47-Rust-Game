	.8086
	.MODEL TINY
	.CODE
	.DATA

; cx - x0
; dx - y0
; si - x1
; di - y1
DrawLine proc
    mov ax,0A000h
    mov es,ax
;Check is 0 or 4 code line 
    cmp cx,si
    jne not_line04
    cmp dx,di
    jb line04    
    xchg dx,di
; (320 * y0 + x0) + 320 -> repeat dy
line04:
    sub di, dx ; di is dy and count

    ;*320
    mov ax,dx
    shl dx,8
    shl ax,6
    add ax,dx
    add ax,cx
    mov si,ax  
    
    mov ax,bp
next_pixel04:
    mov es:[si],al
    cmp di,0
    jz end_drawind0426
    dec di
    add si,320
    jmp next_pixel04
    
not_line04:

;Check is 2 or 6 code line 
    cmp dx,di
    jne not_line26
    cmp cx,si
    jg line26    
    xchg cx,si
; (320 * y0 + x0) + 320 -> repeat dy
line26:
    sub cx, si ; si is dx and count    

    ;*320
    mov di,dx
    shl dx,8
    shl di,6
    add di,dx
    add di,si

    mov ax, bp
    cld ;rep stosb direction flag down - move forward
rep stosb

end_drawind0426:
    ret
    
not_line26:
;normalize
    push cx
    push dx
    push si
    push di
     
    cmp cx, si
    jb x1_high
    xchg cx,si	
;    mov xinc1, -1
;    mov xIncDec1, 049h ; dec cx 
     mov xIncDec1[0], 04Fh     

;    mov xinc2, -1
;    mov xIncDec2, 049h ; dec cx 
     mov xIncDec2[0], 04Fh     
    jmp x0_x1_ok
x1_high:
;    mov xinc1, 1
;    mov xIncDec1, 041h ; inc cx 
     mov xIncDec1[0], 047h     

;    mov xinc2, 1
;    mov xIncDec2, 041h ; inc cx 
     mov xIncDec2[0], 047h     

x0_x1_ok:
    cmp dx, di
    jb y1_high
    xchg dx,di	 	
;    mov yinc1, -1
;    mov yIncDec1, 04Ah ; dec dx 
     mov yIncDec1[0], 081h     
     mov yIncDec1[1], 0EFh     
     mov yIncDec1[2], 040h     
     mov yIncDec1[3], 001h     

;    mov yinc2, -1
;    mov yIncDec2, 04Ah ; dec dx 
     mov yIncDec2[0], 081h     
     mov yIncDec2[1], 0EFh     
     mov yIncDec2[2], 040h     
     mov yIncDec2[3], 001h     

    jmp y0_y1_ok
y1_high:
;    mov yinc1, 1
;    mov yIncDec1, 042h ; inc dx 
     mov yIncDec1[0], 081h     
     mov yIncDec1[1], 0C7h     
     mov yIncDec1[2], 040h     
     mov yIncDec1[3], 001h     

;    mov yinc2, 1
;    mov yIncDec2, 042h ; inc dx 
     mov yIncDec2[0], 081h     
     mov yIncDec2[1], 0C7h     
     mov yIncDec2[2], 040h     
     mov yIncDec2[3], 001h     

y0_y1_ok:

    sub si, cx
    mov _dx, si  ; dx = x1 - x0 ;num

    sub di, dx  ; dy = y1 - y0 ; denum
    mov _dy, di

    pop di
    pop si
    pop dx
    pop cx 	
;---ENDOF normalize

ok_compare:
; dx <-> dy direction
    mov ax, _dy 
    cmp ax, _dx   ; ax = dy
    jg dyBigger   ; dy < dx
    cmp cx, si
    jb OK_x0
    ;mov xinc1, -1 ; OR 1 direction
    ;mov xIncDec1, 049h ; dec cx 
     mov xIncDec1[0], 04Fh     
OK_x0:
    ;mov yinc2, 0
    ;mov yIncDec2, 090h ; nop by default
     mov yIncDec2[0], 090h     
     mov yIncDec2[1], 090h     
     mov yIncDec2[2], 090h     
     mov yIncDec2[3], 090h     


    mov numadd, ax ;numadd = dy 
    mov ax, _dx
    mov denumerator, ax ; dx
    mov numpixels, ax ; dx
    shr ax,1
    mov numerator, ax; ax ; dx /2
    jmp draw
dyBigger:
;    mov xinc2, 0
;    mov xIncDec2, 090h ; nop by defaylt
     mov xIncDec2[0], 090h     
     mov xIncDec2[1], 090h     
     mov xIncDec2[2], 090h     
     mov xIncDec2[3], 090h    



;    mov yinc1, 0
;    mov yIncDec1, 090h ; nop by default
     mov yIncDec1[0], 090h     
     mov yIncDec1[1], 090h     
     mov yIncDec1[2], 090h     
     mov yIncDec1[3], 090h     

    mov denumerator, ax ; dy

    mov numpixels, ax ;dy

    shr ax,1
    mov numerator, ax ; dy /2
    mov ax, _dx
    mov numadd, ax ;numadd = dx     	
    add numpixels, ax


 ;   add numpixels, ax
draw:     

;    cx  = x0
;    dx  = y0
     mov bx,bp ;bx = color

     mov bp, numadd
     mov si, denumerator

     mov add_numadd_value, bp	
     mov sub_denumerator_value, si
     mov cmp_denumerator_value, si

     mov bp, numerator	
     
     mov si, numpixels
     

     jmp $ + 2 ; reset_prefetch_queue

    ;first pixel offset
    ;di = (y * 320) + x	= pixel offset
    mov di,dx
    mov ax,dx
    shl ax,8
    shl di,6
    add di,ax
    add di,cx

BresenhamLoop:
    cmp  si, 0 ; numpixels
    je EndLineDrawing
    dec si


    mov es:[di], bl

noyinc:

    mov ax, bp
    ;add ax, bp
add_numadd_inst db 05h
add_numadd_value dw 0000h
   
    mov bp, ax
;    cmp ax, si
cmp_denumerator_inst db 03Dh
cmp_denumerator_value dw 0000h
    jb inc2

;    sub ax, si
sub_denumerator_inst db 02Dh
sub_denumerator_value dw 0000h

    mov bp, ax

    ;xIncDec1 db 90h ; no operation by default
    ;add cx,xinc1
    xIncDec1 db 90h, 90h, 90h, 90h

    ;yIncDec1 db 90h
    ;add dx,yinc1
    yIncDec1 db 90h, 90h, 90h, 90h
    jmp BresenhamLoop    
inc2:		
    ;xIncDec2 db 90h ; no operation by default
    ;add cx, xinc2
    xIncDec2 db 90h, 90h, 90h, 90h

    ;yIncDec2 db 90h
    ;add dx, yinc2
    yIncDec2 db 90h, 90h, 90h, 90h
    jmp BresenhamLoop
EndLineDrawing:
    ret
DrawLine endp

_dx dw 0
_dy dw 0

numerator dw 0
denumerator dw 0
numadd dw 0
numpixels dw 0

;xinc1 dw  0
;xinc2 dw  0

;yinc1 dw  0
;yinc2 dw  0

end
