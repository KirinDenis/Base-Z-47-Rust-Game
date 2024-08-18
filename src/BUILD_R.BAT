@ECHO OFF
CLS
c:
cd C:\ASM\BASEZ47\SRC\

REM if don't neet to compile when DOSBOX running - open next line
REM c:\vc\vc.com

REM compile all *.asm files in the current directory
@ECHO ON
@ECHO ----------
tasm main.asm

REM link with debug info
REM tlink -Tde main.obj test.obj drawline.obj pixmap.obj vgapal.obj drawrect.obj fio.obj
tlink -Tde main.obj


REM open if need run compiled file 
REM main.com

REM Open DOS Shell Volcov Comander
c:\vc\vc.com

