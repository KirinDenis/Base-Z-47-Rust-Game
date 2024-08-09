c:
cd C:\ASM\BASEZ47\SRC\

rem c:\vc\vc.com

tasm main.asm
tasm drawline.asm
tasm test.asm
tasm pixmap.asm
tasm vgapal.asm
tasm drawrect.asm

tlink -t main.obj test.obj drawline.obj pixmap.obj vgapal.obj drawrect.obj

main.com

c:\vc\vc.com

