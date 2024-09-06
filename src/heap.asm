.MODEL SMALL

_DATAS SEGMENT PARA PUBLIC 'DATA' USE16

HEAP_ITEM STRUC 
	_handler DB ?
        _size    DW ?
        _offset  DW ?
        _segment DW ?
HEAP_ITEM ENDS

handlers db -1

heap HEAP_ITEM 100h dup(?) 

_DATAS ENDS

_CODE SEGMENT PARA PUBLIC 'CODE' USE16
 ASSUME CS:_CODE, DS:_DATAS

;Init heap manager
InitHeap PROC

      push ax	       



      mov handlers, 1h         ; init first block 
      mov al,handlers     
      mov byte ptr ds:[heap]._handler,al            ; store block number to allocate table 

      mov ax, ss           ; last program byte
      add ax, 0FFFh        ; one segement to pages 

      mov ds:[heap]._size, 0      
      mov ds:[heap]._offset, 0h
      mov ds:[heap]._segment, ax

      pop ax
      ret      
InitHeap ENDP 

;-> ax needed memory in paragraphs 
;<- cl memory block index
;<- dx memory block segment
PUBLIC GetHeap
GetHeap PROC near
       cmp handlers,-1
       jne heap_initialized
       call InitHeap
heap_initialized:
       push di
       push bx 
       push ax       
       mov al, handlers
       dec al
       mov dx, SIZE HEAP_ITEM
       imul dx
       mov di, ax	  


       mov bx, ds:[heap+di]._size ; size of last block 
       mov cx, ds:[heap+di]._segment ; segment of last block 
       shr bx,4          ; calculate pages  
       add bx,cx         ; free memory 
       pop ax
       
       add di,SIZE HEAP_ITEM
       inc handlers ; counter to next (free) block 
     	
       mov cl,handlers
       mov ds:[heap+di]._handler,cl    ; store next block 
       mov ds:[heap+di]._size,ax  ; store memory size 
       mov ds:[heap+di]._offset,0h  ; store next free segment 
       mov ds:[heap+di]._segment,bx  ; store next free segment 
       mov dx,bx	           	
     
       pop bx
       pop di             

       ret
GetHeap ENDP
_CODE ENDS
