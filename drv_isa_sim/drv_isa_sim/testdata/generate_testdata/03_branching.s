start: 
    li x1, 0x80
    li x2, 0x01

    loop: 
        slli x2, x2, 0x01
        beq x1, x2, _end
        j loop

    _end: j _end
