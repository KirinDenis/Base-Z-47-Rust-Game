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
;ษออออออป
;บ      บ 
;บ      บ 
;บ      บ 
;บ      บ 
;ศออออออผ    
        mov ax, 0B800h    
        mov es, ax 

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


        mov di, 80*2 * 11        ; move at first row for shadow
        add di, 59 * 2           ; move at begin column of shadow
        mov ah, 01110001b        ; set background color 
        and ah, 10000111b        ; set mask
        mov cx, 10               ; set how many rows
        inc di                   ; go to next columns 
        inc di

_RightShadow:
        mov es:[di],ax
        inc di                   ; move to next column of shadow
        inc di
        mov es:[di],ax           ; print shadow
        dec di                   ; go back to first column
        dec di
        add di, 80 * 2           ; go to next row
        loop _RightShadow


        mov di, 80*2*20          ; move at first row for shadow in bottom
        add di, 21 * 2           ; move at begin of shadow
        mov ah, 7Fh              ; set background color
        and ah, 10000111b
        mov cx, 38               ; set how many columns
        inc di                   ; go to next columns 
        inc di

_BottomShadow:
        mov es:[di],ax
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
  ret
buf db 0

end