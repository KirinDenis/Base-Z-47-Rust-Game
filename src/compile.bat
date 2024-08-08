c:
cd C:\ASM\VGA\VGALINE

rem c:\vc\vc.com

tasm main.asm
tasm drawline.asm
tasm test.asm
tasm bitmap.asm
tasm vgapal.asm
tasm drawrect.asm

tlink -t main.obj test.obj drawline.obj bitmap.obj vgapal.obj drawrect.obj

main.com

c:\vc\vc.com

