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
        lds di,b
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
        
	mov di, 80*2*10 + 20 * 2 ; di now position of top left character 'ษ'
	mov ah, 1Eh      
        mov al, 'ษ'
        mov es:[di],ax ;write the character to screen

        mov al, 'ศ'
        mov es:[di+bx],ax ; di (top left) + 9 rows = botton left 

        inc di
        inc di
        mov al, 'อ'

        mov cx, 38 ;repeat look cicle 28 times -> initialize look section
_nextHorizonatal:
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
        mov es:[di],ax
        mov es:[di+2*39],ax
        add di, 80*2 ;di + 160 -> one screen row
        loop _nextVertical ; dec cx if cx not ZERO loop to _nextVertical:
;end of drawing 

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

b dw 5
buf db 0


end