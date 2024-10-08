@ECHO OFF
CLS
c:
cd C:\ASM\BASEZ47\SRC\

REM if don't neet to compile when DOSBOX running - open next line
REM c:\vc\vc.com

@ECHO ON
@ECHO ----------
tasm main.asm

REM link with debug info
tlink -Tde main.obj


REM open if need run compiled file 
REM main.com

REM Open DOS Shell Volcov Comander
c:\vc\vc.com

