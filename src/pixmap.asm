	.8086
	.MODEL TINY
	.CODE
	

LoadPixmap proc

;---Palette 

	mov dx,offset ds:bmpFileName
        call LoadFile
        mov di, dx
        mov bx, word ptr ds:[di]
        mov cx, word ptr ds:[di+2]
        add di,4
              
        mov dx,320
        sub dx,bx

        mov si, 0A000h
        mov es, si
        xor si, si
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


	ret
LoadPixmap endp
.DATA
bmpFileName db "res\logo1.pix", 00h
bmpfileHandle dw 00h
INCLUDE fio.asm

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

end