	.8086
	.MODEL TINY
	.CODE
	.DATA

LineTests proc
;SVGA 
;     mov ax, 4f02h
;     mov bx, 105h
;     int 10h


;   [--]  [0-]  [+-]   ---> [x-0+ y-0+]
;    7\    |0  /1
;      \   |  / 
;[-0] 6---[00] ----2 [+0]
;      /   |  \
;     /5   |4  \3
;   [-+]  [0+]  [++]

;Test lines 0,1,2,3,4,5,6,8

   jmp no_random_test
;Random test 
   MOV AH, 00h  ; interrupts to get system time        
   INT 1AH      ; CX:DX now hold number of clock ticks since midnight      
   push dx
   push cx

    mov bx, 0FFFFh
_next:
    cmp bx, 0000h
    jbe OK_random    	
    dec bx  
    push bx 

    call GetRandom
    mov cx, ax
;    mov si, ax ; line 0 4 test

    call GetRandom 	
    mov dx, ax      ; y0

    call GetRandom 	
    mov si, ax    ; x1
    add si,150

    call GetRandom 	
    mov di, ax        ; y1
    add di, 50

    call GetRandom 	
    mov bp, ax     ; color
    call DrawLine
    
    pop bx
    jmp _next
    OK_random:

    pop bx
    pop bp
    MOV AH, 00h  ; interrupts to get system time        
    INT 1AH      ; CX:DX now hold number of clock ticks since midnight      
    sub cx, bx
    sub dx, bp
no_random_test:
;    jmp square_test
;--- ENDOF Random test

;Test -1
    mov cx, 0     ; x0
    mov dx, 0      ; y0
    mov si, 20    ; x1
    mov di, 100        ; y1
    mov bp, 05h     ; color
    call DrawLine


;Test 0
    mov cx, 320/2    ; x0
    mov dx, 0      ; y0
    mov si, 320/2    ; x1
    mov di, 200/2        ; y1
    mov bp, 08h     ; color
    call DrawLine

;Test 1
    mov cx, 320/2    ; x0
    mov dx, 200/2    ; y0
    mov si, 320      ; x1
    mov di, 0        ; y1
    mov bp, 09h     ; color
    call DrawLine

;Test 2
    mov cx, 320/2 
    mov dx, 200/2 
    mov si, 320   
    mov di, 200/2 
    mov bp, 0Ah  
    call DrawLine

;Test 3
    mov cx, 320/2 
    mov dx, 200/2 
    mov si, 320   
    mov di, 200
    mov bp, 0Bh  
    call DrawLine

;Test 4
    mov cx, 320/2 
    mov dx, 200/2 
    mov si, 320/2   
    mov di, 200
    mov bp, 0Ch  
    call DrawLine

;Test 5
    mov cx, 320/2 
    mov dx, 200/2 
    mov si, 0   
    mov di, 200
    mov bp, 0Dh  
    call DrawLine

;Test 6
    mov cx, 320/2 
    mov dx, 200/2 
    mov si, 0   
    mov di, 200/2
    mov bp, 0Eh  
    call DrawLine

;Test 7
    mov cx, 320/2 
    mov dx, 200/2 
    mov si, 0   
    mov di, 0
    mov bp, 0Fh  
    call DrawLine

;Test colors map
    mov cx, 10 
    mov dx, 10    
    mov si, 60    
    mov di, 10   
    mov bp, 08h 
    call DrawLine

    mov cx, 10 
    mov dx, 20    
    mov si, 60    
    mov di, 20   
    mov bp, 09h 
    call DrawLine

    mov cx, 10 
    mov dx, 30    
    mov si, 60    
    mov di, 30   
    mov bp, 0Ah 
    call DrawLine

    mov cx, 10 
    mov dx, 40    
    mov si, 60    
    mov di, 40   
    mov bp, 0Bh 
    call DrawLine

    mov cx, 10 
    mov dx, 50    
    mov si, 60    
    mov di, 50   
    mov bp, 0Ch 
    call DrawLine

    mov cx, 10 
    mov dx, 60    
    mov si, 60    
    mov di, 60   
    mov bp, 0Dh 
    call DrawLine

    mov cx, 10 
    mov dx, 70    
    mov si, 60    
    mov di, 70   
    mov bp, 0Eh 
    call DrawLine

    mov cx, 10 
    mov dx, 80    
    mov si, 60    
    mov di, 80   
    mov bp, 0Fh 
    call DrawLine

;Sqaure test 
square_test:
;    jmp noice_test
    mov cx, 0    ; x0
    mov dx, 0      ; y0
    mov si, 0    ; x1
    mov di, 199        ; y1
    mov bp, 0Fh     ; color
    call DrawLine


    mov cx, 1    ; x0
    mov dx, 0      ; y0
    mov si, 319    ; x1
    mov di, 0        ; y1
    mov bp, 08h     ; color
    call DrawLine

    mov cx, 0    ; x0
    mov dx, 199      ; y0
    mov si, 319    ; x1
    mov di, 199       ; y1
    mov bp, 09h     ; color
    call DrawLine

    mov cx, 319    ; x0
    mov dx, 0      ; y0
    mov si, 319    ; x1
    mov di, 199       ; y1
    mov bp, 07h     ; color
    call DrawLine
_next_test:
;---ENDOF Sqaure test 

;Noice test
noice_test:
   jmp lianer_test
   MOV AH, 00h  ; interrupts to get system time        
   INT 1AH      ; CX:DX now hold number of clock ticks since midnight      
   push dx
   push cx

    mov bx, 0FFh
next_noice:
    cmp bx, 0000h
    jbe OK_noice    	
    dec bx  
    push bx 

    call GetRandom
    mov cx, ax


    call GetRandom

    mov si,ax
    add si,320

    call GetRandom 	
    mov dx, ax      ; y0
    mov di, ax
    

    call GetRandom 	
    mov ah,0
    and al, 00011000b
     

    mov bp, ax     ; color
    call DrawLine
    
    pop bx
    jmp next_noice
 OK_noice:
    pop bx
    pop bp
    MOV AH, 00h  ; interrupts to get system time        
    INT 1AH      ; CX:DX now hold number of clock ticks since midnight      
    sub cx, bx
    sub dx, bp

lianer_test:    
    jmp no_test
    mov bx, 0FFFFh
    mov ax, 0
    mov cx, 0
lianer_next:
    cmp bx, 0000h
    jbe OK_lianer
    dec bx 


    add ax,1
    cmp ax,320
    jb ok_next_line  
    mov ax,0

 
ok_next_line:
    push ax
    push bx
    push cx 

    mov cx, 0    ; x0
    mov dx, 0    ; y0
    mov si, 60
    mov di, 100        ; y1



    push ax
    call GetRandom 	
    mov bp, ax     ; color
    pop ax

    call DrawLine
    
    pop cx
    pop bx
    pop ax
    jmp lianer_next
    OK_lianer:

    pop bx
    pop bp
    MOV AH, 00h  ; interrupts to get system time        
    INT 1AH      ; CX:DX now hold number of clock ticks since midnight      

no_test:
    ret
;---ENDOF Noice test 

LineTests endp 

;Return random at AX
GetRandom proc near
   push cx
   push dx 
   MOV AH, 00h  ; interrupts to get system time        
   INT 1AH      ; CX:DX now hold number of clock ticks since midnight      


   mov ax, dx  
   xor ax, cs:[bx]	
   add ax, cs:[bx+0100h]	
   add ax, cs:[bx+0200h]	

;   mov  cx, 10    
;   div  cx       ; here dx contains the remainder of the division - from 0 to 9
   cmp ax, 320
   jb ok_ax
   xor ah,al
   mov ah,0
ok_ax:
   cmp al,0F0h
   jb ok2
   rol al, 4
ok2:
 

   pop dx
   pop cx
   ret
getRandom endp

include DrawLine.asm

end


