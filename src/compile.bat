c:
cd C:\ASM\BASEZ47\SRC\

;c:\vc\vc.com

tasm /zi /z /t main.asm
tasm /zi /z /t drawline.asm
tasm /zi /z /t test.asm
tasm /zi /z /t pixmap.asm
tasm /zi /z /t vgapal.asm
tasm /zi /z /t drawrect.asm
tasm /zi /z /t fio.asm

tlink -Tde /l /v main.obj test.obj drawline.obj pixmap.obj vgapal.obj drawrect.obj fio.obj

;main.com

c:\vc\vc.com

