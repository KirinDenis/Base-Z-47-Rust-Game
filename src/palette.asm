;Palette 
mov cx,0FFFFh

_n2:
push cx
mov bl,cl
mov cx,255

_np:
add bl,cl

mov dx, 3C8h
mov al, cl       
out dx, al      

mov dx, 3C9h   
mov al, bl
out dx, al      
mov al, cl      
out dx, al      
mov al, bl       
out dx, al      
loop _np

pop cx
loop _n2
