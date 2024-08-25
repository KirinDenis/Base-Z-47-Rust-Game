.MODEL SMALL

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

InitMem proc
      lea di, block_counter        
      mov ax, ds
      lea bx, heap
      shr bx, 4
      add ax,bx
      mov ds:[di+3], ax
      ret      
InitMem endp 

;-> ax needed memory in paragraphs 
;<- cl memory block index
;<- dx memory block segment
GetMem proc
       push ax
       lea dx, block_counter
       mov ax, 5
       imul dx
       lea di, block
       add di, ax	
       mov bx, ds:[di+1]
       mov cx, ds:[di+3]
       add bx,cx
       pop ax
       
       add di,5
       inc block_counter
       mov cl,block_counter
       mov ds:[di],cl  
       mov ds:[di+1],ax
       mov ds:[di+3],bx
       mov dx,bx	           	
          
       ret
GetMem ENDP
_CODE ENDS




