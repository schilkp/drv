.section .text

_start:
    lui x15, 0x12345
    auipc x10, 0x1beef
    jal x1, .+0x4
    jalr x11, 0x4(x12)
    beq x0, x1, .+0x4
    bne x2, x3, .+0x8
    blt x4, x5, .+0x12
    bge x6, x7, .+0x16
    bltu x8, x9, .+0x20
    bgeu x10, x11, .+0x24
    lb x12, 0xFF(x13)
    lh x12, 0xFF(x13)
    lw x12, 0xFF(x13)
    lbu x12, 0xFF(x13)
    lhu x12, 0xFF(x13)
    sb x1, 0xAB(x0)
    sh x2, 0xAB(x1)
    sw x3, 0xAB(x2)
    addi x5, x6, 0x01
    slti x5, x6, 0x01
    sltiu x5, x6, 0x01
    xori x5, x6, 0x01
    ori x5, x6, 0x01
    andi x5, x6, 0x01
    slli x6, x5, 31
    srli x7, x8, 20
    srai x1, x2, 10
    add x1, x2, x3
    sub x4, x5, x6
    sll x7, x8, x9
    slt x10, x11, x12
    sltu x10, x11, x12
    xor x1, x2, x3
    srl x3, x4, x5
    sra x3, x4, x5
    or x1, x2, x3
    and x1, x2, x3
    fence
    ecall
    ebreak
    dret
    mret
