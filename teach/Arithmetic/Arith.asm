	.MODEL TINY 
	.CODE
	.DATA
	.STARTUP
	ORG 100H

;NOTE use Turbo Debugger menu View\CPU
;AAD Instruction (ASCII Adjust AX Before Division)
;https://github.com/KirinDenis/Mystery-of-Base-Z-47-DOS-Game/wiki/%F0%9F%A7%AE-AAD-Instruction


	mov ah, 05h  ; AH = 05h (5 in decimal)
	mov al, 08h  ; AL = 08h (8 in decimal)
	aad	     ; AX = (10 * 5) + 8 = 58 decimal (3Ah)

	mov ah, 00h  
	mov al, 00h  
	aad	     

	mov ah, 01h  
	mov al, 01h  
	aad	     

	mov ah, 01h  
	mov al, 09h  
	aad	     

	mov ah, 09h  
	mov al, 09h  
	aad	     

	mov ah, 10h  
	mov al, 01h  
	aad	    ; ERROR Sign Flag is UP  and Parity Flag is UP -> Parity Flag (PF) - this flag is set to 1 when there is even number of one bits in result, and to 0 when there is odd number of one bits. Even if result is a word only 8 low bits are analyzed)

;AAM Instruction (ASCII Adjust AX After Multiply)
;https://github.com/KirinDenis/Mystery-of-Base-Z-47-DOS-Game/wiki/%F0%9F%94%A2-AAM-Instruction

	mov al, 0Ah  ; AL = 0Ah (10 in decimal)
	mov ah, 05h  ; AH = 05h (5 in decimal)
	mul ah       ; AX = AL * AH = 32h (50 in decimal)
	aam          ; AX = 0500h -> AH 05h, AL 00h -> Adjust AX for BCD

	mov ax,0010h ; AX = 16 decimal
        aam          ; AX = 0106h -> AH 01h, AL 06h -> Adjust AX for BCD 

        mov ax,00FFh
        aam         ; ERROR Parity Flag is UP

;ADC (Add with Carry)
;https://github.com/KirinDenis/Mystery-of-Base-Z-47-DOS-Game/wiki/%F0%9F%A7%AE-ADC-Instruction

	mov al, 01h    ; Load 1 into AL
	mov bl, 01h    ; Load 1 into BL
	adc al, bl     ; AL = AL + BL + No Carry flag

	mov al, 0FFh   ; Load FF into AL
	mov bl, 001h   ; Load 1 into BL
	adc al, bl     ; AL = AL + BL + Carry flag = 00h -> In the 8086, byte register addition is cyclic�adding 1 to 0xFF results in 0x00 and sets the overflow flag.
	               ; Flags:
                       ; - Carry flag UP -> for this instruction is mean result is 100h = FFh + 1h
                       ; - Zero flag UP -> formaly the result is 0
                       ; - Parity Flag is UP -> 0 is even number
                       ; - Auxiliary Carry Flag is UP ->  a carry is a digit that is transferred from one column of digits to another column of more significant digits?

	mov al, 0FFh   ; Load FF into AL
	mov bl, 002h   ; Load 1 into BL
	adc al, bl     ; AL = AL + BL + Carry flag = 01h (101h) -> byte register cyclic�adding
                       ; Flags: Carry flag UP and Auxiliary Carry flag is UP

	mov al, 0FFh   ; Load FF into AL
	mov bl, 0FFh   ; Load 1 into BL      
	adc al, bl     ; AL = AL + BL + Carry flag = FEh -> FFh + FFh = 1FEh (-2 decimal -> SEE: Sign Flag flag)
                       ; Flags: 
                       ; - Carry flag UP
                       ; - Auxiliary Carry flag is UP
                       ; - Sign Flag is UP the value is negative



	ret
end