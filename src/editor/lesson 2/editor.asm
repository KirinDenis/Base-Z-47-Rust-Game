.MODEL TINY 
      .CODE
      .DATA
      .STARTUP
      ORG 100H

; x = 20
; y = 10
; 80 * 2 * y + x * 2
; width  40 - 2 = 38 
; height 10 - 2 = 8

; 10 10 50 20
;
;ษออออออป
;บ      บ 
;บ      บ 
;บ      บ 
;บ      บ 
;ศออออออผ    


; Procedure Languages                        OOP Languages 
; Procedure (PROC) didn't return result      void Methods 
; Function                                   Methods return result  
; C/C++ 
; Borland Pascal and Delphi 


        mov ax, 0B800h    
        mov es, ax 

;        call DrawBox 
;call ret
        jmp _DrawBox
 _DrawBoxEndp:
        nop
        nop 
        nop
;COM file 


public void main()
{
	a1();
        mov();
        int();
        add();
        return;  
}
public void a1()
{
        c1();
        nop();
        return; 
}
public void c1()
{       
        b1();
         ....
        return;
}
   
      ... b1();

ADDR
0  int 20h ;return control to DOS
main proc
1     call a1 ; <-ip = 1
		;      jmp _a1
		;ret_a1:
2     mov 
3     int 
4     add 
5     ret ;back control to OS so our program is completed 
main end
a1 proc 
_a1:
6    call c1 
7    not  
8   ret
;    jmp ret_a1
a1 end
c1 proc 
9   call b1 
10  nop
11  nop 
12  ret
c1 end

b1 proc 
13 nop
14 nop
15 ret      
b1 end

STACK
DOS 

    
    pop 0007h 
    pop 0002h
    pop 0000h 
 

a db 100  
b db 100  
c db 100  
d db 100  
        
DrawBox PROC 
_DrawBox:
;grab video memory
        lea si,buf
        xor di,di   ;zero di register 
        mov cx, 80*25
grabnext:
        mov ax, es:[di]
        mov ds:[si],ax
        inc di
        inc di
        inc si
        inc si
        loop grabnext          

;Start draw

        mov bx, 80 * 2 * 9 ; last row offset from first row 
        
        mov di, 80*2*10 + 20 * 2 ; di now position of top left character '?'
        mov ah, 112
        mov al, 'ษ'
        mov es:[di],ax ;write the character to screen

        mov al, 'ศ'
        mov es:[di+bx],ax ; di (top left) + 9 rows = botton left 

        inc di
        inc di
        mov al, 'อ'

        mov cx, 38 ;repeat look cicle 28 times -> initialize look section
_nextHorizonatal:
        mov ah, 112
        mov es:[di],ax
        mov es:[di+bx],ax
        inc di
        inc di
        loop _nextHorizonatal ; dec cx if cx not ZERO loop to _nextHorizonatal

        mov al, 'ป'
        mov es:[di],ax

        mov al, 'ผ'
        mov es:[di+bx],ax

        mov di, 80*2*11 + 20 * 2 
        mov al, 'บ'
        mov cx, 8

_nextVertical:
        mov ah, 112
        mov es:[di],ax
        mov es:[di+2*39],ax
        add di, 80*2             ;di + 160 -> one screen row
        loop _nextVertical       ; dec cx if cx not ZERO loop to _nextVertical:
;end of drawing 

;begin filling the box
        xor al,al
        mov di, 80*2 * 11 ; move at first row inside box
        add di, 21 * 2    ; move at begin of column inside box
        mov ah, 7Fh    ; set background color
        mov cx, 38        ; set how many columns
        mov bl, 8         ; counting rows
        
_FillBox:
        mov es:[di],ax
        inc di                ; go to next column
        inc di
        cmp cx, 1             ; geting last column inside box
        je _JumpFillNextRow   ; if last column is filled going to next row
        loop _FillBox

        
_JumpFillNextRow:
        add di, 80 * 2 - 38 * 2  ; going next row and position at first column
        dec bl                   ; counting rows
        mov cx, 38               ; setting width
        cmp bl, 0                ; checking if rows are completed
        jg _FillBox 
;finish of filling the box


        mov di, 80*2 * 11  +  59 * 2 + 1; move at first row for shadow
        mov cx, 10               ; set how many rows
        inc di                   ; go to next columns 
        inc di

;[Attrs][Char][Attrs][Char][Attrs][Char][Attrs][Char][Attrs][Char][Attrs][Char]
; ^      
; DI        
_RightShadow:
        mov ah, es:[di]          ; read video memory background and foreground color 
        and ah, 10000110b        ; set mask
        mov es:[di],ah
        ;inc di                   ; move to next column of shadow
        ;inc di
        ;mov es:[di],ah           ; print shadow
        ;dec di                   ; go back to first column
        ;dec di
        add di, 80 * 2           ; go to next row

        loop _RightShadow


        mov di, 80*2*20 + 21 * 2+ 1     ; move at first row for shadow in bottom

        mov cx, 38               ; set how many columns
        inc di                   ; go to next columns 
        inc di

_BottomShadow:
        mov ah, es:[di]          ; read video memory background and foreground color 
        and ah, 10000110b        ; set mask
        mov es:[di],ah
        inc di
        inc di
        loop _BottomShadow       
   
_JmpNextRow:

        mov ah, 0
        int 16h   ;wait user press any key 
        lea si,buf
        xor di,di   ;zero di register 
        mov cx, 80*25
backnext:
        mov ax,ds:[si]
        mov es:[di],ax
        inc di
        inc di
        inc si
        inc si
        loop backnext          
        ;ret
        jmp _DrawBoxEndp
DrawBox ENDP
buf db 0

end