start: 
   li x1, 0x2000000 # RAM base adr.

   # Store '0xDEADBEEF' to RAM_START+0:
   li x2, 0xDEADBEEF
   sw x2, 0(x1)

   # Store '0xF1BEF1BE' to RAM_START+4:
   li x2, 0xF1BEF1BE
   sw x2, 4(x1)

   # Read RAM_START+0:
   lw x2, 0(x1)

   # Read RAM_START+4 as four seperate bytes:
   lbu x3, 4(x1)

   lbu x4, 5(x1)
   slli x4, x4, 8
   or x3, x3, x4

   lbu x4, 6(x1)
   slli x4, x4, 16
   or x3, x3, x4

   lbu x4, 7(x1)
   slli x4, x4, 24
   or x3, x3, x4

   _done: j _done

