.MODEL TINY 
      .CODE
      .DATA
      .STARTUP
      ORG 100H


main proc
   call a1
   nop
   nop 
   mov ax,100h
   xor bx,ax
   ret
main endp

a1 proc
   nop
   call b1
   nop 
   mov ax,1123h
   add bx,ax
   ret
a1 endp

b1 proc
   call c1
   inc dx
   inc dx
   inc dx
   ret

b1 endp

c1 proc

   dec cx
   dec cx
   dec cx
   ret
c1 endp

end