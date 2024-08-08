	.8086
	.MODEL TINY
	.CODE
	.DATA

LoadBitmap proc

	mov ax,3D00h ; open for input
	mov dx,offset bmpFileName
	int 21h
	mov bx,ax ; ax = bx = opened file handle


	mov ax,4202h ;file offset to end of file 
	xor dx,dx 
	xor cx,cx 
	int 21h   ;dx:ax the file size, ignore ax one segment file 
        
        mov cx,ax ;number of byte to read 
        push cx 

	mov ax,4200h ;file offset to begining of file 
	xor dx,dx 
	xor cx,cx 
	int 21h  

        pop cx

	mov ax, 3F00h  ;load bitmap data from file 
        lea dx, bfType
        int 21h

	mov ax, 3E00h  ;close file 
        int 21h  

	mov bx,1
_next_palete:
        cmp bx,0
        je _ok_draw
        dec bx        

        lea di, bfType
   ;     mov ax, word ptr [bfOffBits]
   ;     add di, ax
   ;     sub cx, ax
        mov cx, 319*199
        mov si, 0A000h
        mov es, si
        xor si, si
        
_nextBitMap:
        cmp cx, 0
        je _okNextBit
        dec cx
        xor ah, ah
	mov al, ds:[di]
;        mov bx, 255
;        imul bx
;        mov bx, 63
;        div bx
        mov es:[si],al
        inc di
        inc si
        jmp _nextBitMap     
_okNextBit:
	jmp _next_palete
_ok_draw:
	ret
LoadBitmap endp

bmpFileName db "logo1.bmp", 00h
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

end