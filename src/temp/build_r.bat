@ECHO OFF
CLS
c:
cd C:\ASM\BASEZ47\SRC\

REM if don't neet to compile when DOSBOX running - open next line
REM c:\vc\vc.com

REM Compile with DEBUG simbols 
REM Debug mode
REM tasm /zi /z /t main.asm; /zi /z /t drawline.asm; /zi /z /t test.asm; /zi /z /t pixmap.asm; /zi /z /t vgapal.asm;
REM tasm /zi /z /t drawrect.asm; /zi /z /t fio.asm;

REM compile all *.asm files in the current directory
@ECHO ON
@ECHO ----------
tasm /zi /z /t *

REM link with debug info
tlink -Tde /l /v main.obj test.obj drawline.obj pixmap.obj vgapal.obj drawrect.obj fio.obj

REM open if need run compiled file 
REM main.com

REM Open DOS Shell Volcov Comander
c:\vc\vc.com

