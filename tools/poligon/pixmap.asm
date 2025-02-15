.MODEL SMALL 	


INCLUDE fio.asm

_CODE SEGMENT PARA PUBLIC 'CODE' USE16
pseg dw 0
poffset dw -1

 ASSUME CS:_CODE, DS:_DATAS

; stack x,y 
;
;

LoadPixmap proc

;---Palette 
	
        pop cx
	pop ax
        pop si

	push cx
	push ds
        push si
        push ax
        cmp poffset,-1
        jne _notzero
	mov dx,offset ds:bmpFileName
        call LoadFile
        mov poffset,dx
        mov pseg, ds 
        jmp   _okdx
_notzero:
       mov dx, pseg
       mov ds,dx
       mov dx, poffset	
_okdx:

        mov di, dx
        mov bx, word ptr ds:[di]
        mov cx, word ptr ds:[di+2]
        add di,4
              
        mov dx,320
        sub dx,bx

	pop ax ; y
        mov si,320
        push dx
        imul si  
        pop dx
        pop si ; x 
        add si, ax

        xor ah, ah
        
_nextBitMap:
        cmp cx, 0
        je _ok_draw
        dec cx
	push bx
_nextPixel:
	cmp bx,0
        je _nextLine
        dec bx
	mov al, ds:[di]
        mov es:[si],al
        inc di
        inc si
        jmp _nextPixel     
_nextLine:
        pop bx
        add si,dx
        jmp _nextBitMap
_ok_draw:
        pop ds
	ret
LoadPixmap endp
_CODE ENDS



_DATAS SEGMENT PARA PUBLIC 'DATA' USE16

bmpFileName db "res\logo1.pix", 00h
bmpfileHandle dw 00h


;BitMap bugger
;tagBITMAPFILEHEADER 
     bfType dw 00h
     bfSize dd 00h
bfReserved1 dw 00h
bfReserved2 dw 00h
bfOffBits   dd 00h

;tagBITMAPINFOHEADER
biSize dd  00h  
biWidth dd 00h    
biHeight dd 00h    
biPlanes dw 00h    
biBitCount dw 00h     
biCompression dd  00h   
biSizeImage dd  00h   
biXPelsPerMeter dd 00h    
biYPelsPerMeter dd 00h     
biClrUsed dd   00h  
biClrImportant dd   00h  

_DATAS ENDS
