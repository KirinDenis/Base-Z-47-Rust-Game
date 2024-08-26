.MODEL SMALL
.STACK


INCLUDE pixmap.asm

_DATAS SEGMENT PARA PUBLIC 'DATA' USE16
block_counter db 0
;index 1 byte 
;size  2 byte 
;segment 2 byte 
block db 5*100h dup(0) ;100h sections 

heap db 'heap'
_DATAS ENDS

       
_CODE SEGMENT PARA PUBLIC 'CODE' USE16
 ASSUME CS:_CODE, DS:_DATAS

_START:
;initial DS
	mov ax, _DATAS
        mov ds,ax

	call initMem
         
;        mov ax, 100h
;        call getMem

;        mov ax, 0FFFh
;        call getMem

;        mov ax, 0F00h
;        call getMem
		

	mov ah, 00h
	mov al, 13h
	int 10h

;    call MemAlloc
;   call LineTests
   call LoadPixmap
   call LoadPixmap
   call LoadPixmap
   call LoadPixmap
   call LoadPixmap
   call LoadPixmap
   call LoadPixmap
   call LoadPixmap

;   call VGAPalette

    mov ah, 0
    int 16h

    mov ax, 03h
    int 10h

    mov ax, 4C00h
    int 21h

;Init heap manager
InitMem proc
      lea di, ds:block		; first block allocate table record 
      mov ax, ds
      mov bx, ss           ; last program byte
      add bx, 0FFFh   
      shr bx, 4                 ; recalculate to pages 
      add ax,bx                 ; first free byte of memory

      inc block_counter         ; init first block 
      mov cl,block_counter      
      mov ds:[di],cl            ; store block number to allocate table 

      mov ds:[di+3], ax
      ret      
InitMem endp 

;-> ax needed memory in paragraphs 
;<- cl memory block index
;<- dx memory block segment
PUBLIC GetMem
GetMem proc near
       push ax       
       mov dl, block_counter
       dec dl 	        ;  normalize index to zero based 
       mov ax, 5 
       imul dl		 ; calculate offset of last block     
       lea di, block     ; offset block allocate table 
       add di, ax	 ; offset of last block 
       mov bx, ds:[di+1] ; size of last block 
       mov cx, ds:[di+3] ; segment of last block 
       shr bx,4          ; calculate pages  
       add bx,cx         ; free memory 
       pop ax
       
       add di,5          ; first free block at block allocate table 
       inc block_counter ; counter to next (free) block 
       mov cl,block_counter
       mov ds:[di],cl    ; store next block 
       mov ds:[di+1],ax  ; store memory size 
       mov ds:[di+3],bx  ; store next free segment 
       mov dx,bx	           	
          
       ret
GetMem ENDP

_CODE ENDS

END _START



