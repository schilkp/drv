start: 
   j jmp1

   jmp3:
       li x1, 0x10000000
       li x2, 2
       add x3, x1, x2
       andi x3, x3, 0xFF
       j _done

   jmp2: j jmp3
   jmp1: j jmp2

   _done: j _done
