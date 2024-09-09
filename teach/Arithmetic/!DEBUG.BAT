@ECHO ON
@ECHO 
tasm /zi /z /t arith.asm
tlink  /l /v  /Tde arith.obj
td arith.exe


