start: 
    li x1, 0x100
    jal x2, f1
    jal x2, f1
    jal x2, f1
    jal x2, f1
    _end: j _end

f1:
    addi x1, x1, 0x10
    jalr x0, 0(x2)

