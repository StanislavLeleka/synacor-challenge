pub enum Opcodes {
    OpHalt = 0, /* stop execution and terminate the program */
    OpSet, /* set register <a> to the value of <b> */
    OpPush, /* push <a> onto the stack */
    OpPop, /* remove the top element from the stack and write it into <a>; empty stack = error */
    OpEq, /* set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise */
    OpGt, /* set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise */
    OpJmp, /* jump to <a> */
    OpJt, /* if <a> is nonzero, jump to <b> */
    OpJf, /* if <a> is zero, jump to <b> */
    OpAdd, /* assign into <a> the sum of <b> and <c> (modulo 32768) */
    OpMult, /* store into <a> the product of <b> and <c> (modulo 32768) */
    OpMod, /* store into <a> the remainder of <b> divided by <c> */
    OpAnd, /* stores into <a> the bitwise and of <b> and <c> */
    OpOr, /* stores into <a> the bitwise or of <b> and <c> */
    OpNot, /* stores 15-bit bitwise inverse of <b> in <a> */
    OpRmem, /* read memory at address <b> and write it to <a> */
    OpWmem, /* write the value from <b> into memory at address <a> */
    OpCall, /* write the address of the next instruction to the stack and jump to <a> */
    OpRet, /* remove the top element from the stack and jump to it; empty stack = halt */
    OpOut, /* write the character represented by ascii code <a> to the terminal */
    OpIn, /* read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard and trust that they will be fully read */
    OpNoop /* no operation */
}

impl Opcodes {
    pub fn from_u16(value: u16) -> Opcodes {
        match value {
            0 => Opcodes::OpHalt,
            1 => Opcodes::OpSet,
            2 => Opcodes::OpPush,
            3 => Opcodes::OpPop,
            4 => Opcodes::OpEq,
            5 => Opcodes::OpGt,
            6 => Opcodes::OpJmp,
            7 => Opcodes::OpJt,
            8 => Opcodes::OpJf,
            9 => Opcodes::OpAdd,
            10 => Opcodes::OpMult,
            11 => Opcodes::OpMod,
            12 => Opcodes::OpAnd,
            13 => Opcodes::OpOr,
            14 => Opcodes::OpNot,
            15 => Opcodes::OpRmem,
            16 => Opcodes::OpWmem,
            17 => Opcodes::OpCall,
            18 => Opcodes::OpRet,
            19 => Opcodes::OpOut,
            20 => Opcodes::OpIn,
            21 => Opcodes::OpNoop,
            _ => panic!()
        } 
    }
}