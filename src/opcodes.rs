use once_cell::sync::OnceCell;

pub struct ExpOpCode<'a> {
    pub hex: u8,
    pub str: &'a str,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OpCode(pub u8);

impl OpCode {
    pub fn new(uint: u8) -> Self {
        Self(uint)
    }

    pub fn try_from_u8<'a>(opcode: u8) -> Option<OpCode> {
        OPCODE_JUMPMAP.get().unwrap()[opcode as usize].map(|_| OpCode(opcode))
    }

    pub fn as_str<'a>(&self) -> &'static str {
        if let Some(str) = OPCODE_JUMPMAP.get().unwrap()[self.0 as usize] {
            str
        } else {
            "unknown"
        }
    }

    #[inline(always)]
    pub const fn u8(&self) -> u8 {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        OPCODE_JUMPMAP.get().unwrap()[self.0 as usize].is_some()
    }
}

pub static OPCODE_JUMPMAP: OnceCell<[Option<&'static str>; 256]> = OnceCell::with_value([
    /* 0x00 */ Some("STOP"),
    /* 0x01 */ Some("ADD"),
    /* 0x02 */ Some("MUL"),
    /* 0x03 */ Some("SUB"),
    /* 0x04 */ Some("DIV"),
    /* 0x05 */ Some("SDIV"),
    /* 0x06 */ Some("MOD"),
    /* 0x07 */ Some("SMOD"),
    /* 0x08 */ Some("ADDMOD"),
    /* 0x09 */ Some("MULMOD"),
    /* 0x0a */ Some("EXP"),
    /* 0x0b */ Some("SIGNEXTEND"),
    /* 0x0c */ None,
    /* 0x0d */ None,
    /* 0x0e */ None,
    /* 0x0f */ None,
    /* 0x10 */ Some("LT"),
    /* 0x11 */ Some("GT"),
    /* 0x12 */ Some("SLT"),
    /* 0x13 */ Some("SGT"),
    /* 0x14 */ Some("EQ"),
    /* 0x15 */ Some("ISZERO"),
    /* 0x16 */ Some("AND"),
    /* 0x17 */ Some("OR"),
    /* 0x18 */ Some("XOR"),
    /* 0x19 */ Some("NOT"),
    /* 0x1a */ Some("BYTE"),
    /* 0x1b */ Some("SHL"),
    /* 0x1c */ Some("SHR"),
    /* 0x1d */ Some("SAR"),
    /* 0x1e */ None,
    /* 0x1f */ None,
    /* 0x20 */ Some("SHA3"),
    /* 0x21 */ None,
    /* 0x22 */ None,
    /* 0x23 */ None,
    /* 0x24 */ None,
    /* 0x25 */ None,
    /* 0x26 */ None,
    /* 0x27 */ None,
    /* 0x28 */ None,
    /* 0x29 */ None,
    /* 0x2a */ None,
    /* 0x2b */ None,
    /* 0x2c */ None,
    /* 0x2d */ None,
    /* 0x2e */ None,
    /* 0x2f */ None,
    /* 0x30 */ Some("ADDRESS"),
    /* 0x31 */ Some("BALANCE"),
    /* 0x32 */ Some("ORIGIN"),
    /* 0x33 */ Some("CALLER"),
    /* 0x34 */ Some("CALLVALUE"),
    /* 0x35 */ Some("CALLDATALOAD"),
    /* 0x36 */ Some("CALLDATASIZE"),
    /* 0x37 */ Some("CALLDATACOPY"),
    /* 0x38 */ Some("CODESIZE"),
    /* 0x39 */ Some("CODECOPY"),
    /* 0x3a */ Some("GASPRICE"),
    /* 0x3b */ Some("EXTCODESIZE"),
    /* 0x3c */ Some("EXTCODECOPY"),
    /* 0x3d */ Some("RETURNDATASIZE"),
    /* 0x3e */ Some("RETURNDATACOPY"),
    /* 0x3f */ Some("EXTCODEHASH"),
    /* 0x40 */ Some("BLOCKHASH"),
    /* 0x41 */ Some("COINBASE"),
    /* 0x42 */ Some("TIMESTAMP"),
    /* 0x43 */ Some("NUMBER"),
    /* 0x44 */ Some("DIFFICULTY"),
    /* 0x45 */ Some("GASLIMIT"),
    /* 0x46 */ Some("CHAINID"),
    /* 0x47 */ Some("SELFBALANCE"),
    /* 0x48 */ Some("BASEFEE"),
    /* 0x49 */ None,
    /* 0x4a */ None,
    /* 0x4b */ None,
    /* 0x4c */ None,
    /* 0x4d */ None,
    /* 0x4e */ None,
    /* 0x4f */ None,
    /* 0x50 */ Some("POP"),
    /* 0x51 */ Some("MLOAD"),
    /* 0x52 */ Some("MSTORE"),
    /* 0x53 */ Some("MSTORE8"),
    /* 0x54 */ Some("SLOAD"),
    /* 0x55 */ Some("SSTORE"),
    /* 0x56 */ Some("JUMP"),
    /* 0x57 */ Some("JUMPI"),
    /* 0x58 */ Some("PC"),
    /* 0x59 */ Some("MSIZE"),
    /* 0x5a */ Some("GAS"),
    /* 0x5b */ Some("JUMPDEST"),
    /* 0x5c */ None,
    /* 0x5d */ None,
    /* 0x5e */ None,
    /* 0x5f */ None,
    /* 0x60 */ Some("PUSH1"),
    /* 0x61 */ Some("PUSH2"),
    /* 0x62 */ Some("PUSH3"),
    /* 0x63 */ Some("PUSH4"),
    /* 0x64 */ Some("PUSH5"),
    /* 0x65 */ Some("PUSH6"),
    /* 0x66 */ Some("PUSH7"),
    /* 0x67 */ Some("PUSH8"),
    /* 0x68 */ Some("PUSH9"),
    /* 0x69 */ Some("PUSH10"),
    /* 0x6a */ Some("PUSH11"),
    /* 0x6b */ Some("PUSH12"),
    /* 0x6c */ Some("PUSH13"),
    /* 0x6d */ Some("PUSH14"),
    /* 0x6e */ Some("PUSH15"),
    /* 0x6f */ Some("PUSH16"),
    /* 0x70 */ Some("PUSH17"),
    /* 0x71 */ Some("PUSH18"),
    /* 0x72 */ Some("PUSH19"),
    /* 0x73 */ Some("PUSH20"),
    /* 0x74 */ Some("PUSH21"),
    /* 0x75 */ Some("PUSH22"),
    /* 0x76 */ Some("PUSH23"),
    /* 0x77 */ Some("PUSH24"),
    /* 0x78 */ Some("PUSH25"),
    /* 0x79 */ Some("PUSH26"),
    /* 0x7a */ Some("PUSH27"),
    /* 0x7b */ Some("PUSH28"),
    /* 0x7c */ Some("PUSH29"),
    /* 0x7d */ Some("PUSH30"),
    /* 0x7e */ Some("PUSH31"),
    /* 0x7f */ Some("PUSH32"),
    /* 0x80 */ Some("DUP1"),
    /* 0x81 */ Some("DUP2"),
    /* 0x82 */ Some("DUP3"),
    /* 0x83 */ Some("DUP4"),
    /* 0x84 */ Some("DUP5"),
    /* 0x85 */ Some("DUP6"),
    /* 0x86 */ Some("DUP7"),
    /* 0x87 */ Some("DUP8"),
    /* 0x88 */ Some("DUP9"),
    /* 0x89 */ Some("DUP10"),
    /* 0x8a */ Some("DUP11"),
    /* 0x8b */ Some("DUP12"),
    /* 0x8c */ Some("DUP13"),
    /* 0x8d */ Some("DUP14"),
    /* 0x8e */ Some("DUP15"),
    /* 0x8f */ Some("DUP16"),
    /* 0x90 */ Some("SWAP1"),
    /* 0x91 */ Some("SWAP2"),
    /* 0x92 */ Some("SWAP3"),
    /* 0x93 */ Some("SWAP4"),
    /* 0x94 */ Some("SWAP5"),
    /* 0x95 */ Some("SWAP6"),
    /* 0x96 */ Some("SWAP7"),
    /* 0x97 */ Some("SWAP8"),
    /* 0x98 */ Some("SWAP9"),
    /* 0x99 */ Some("SWAP10"),
    /* 0x9a */ Some("SWAP11"),
    /* 0x9b */ Some("SWAP12"),
    /* 0x9c */ Some("SWAP13"),
    /* 0x9d */ Some("SWAP14"),
    /* 0x9e */ Some("SWAP15"),
    /* 0x9f */ Some("SWAP16"),
    /* 0xa0 */ Some("LOG0"),
    /* 0xa1 */ Some("LOG1"),
    /* 0xa2 */ Some("LOG2"),
    /* 0xa3 */ Some("LOG3"),
    /* 0xa4 */ Some("LOG4"),
    /* 0xa5 */ None,
    /* 0xa6 */ None,
    /* 0xa7 */ None,
    /* 0xa8 */ None,
    /* 0xa9 */ None,
    /* 0xaa */ None,
    /* 0xab */ None,
    /* 0xac */ None,
    /* 0xad */ None,
    /* 0xae */ None,
    /* 0xaf */ None,
    /* 0xb0 */ None,
    /* 0xb1 */ None,
    /* 0xb2 */ None,
    /* 0xb3 */ None,
    /* 0xb4 */ None,
    /* 0xb5 */ None,
    /* 0xb6 */ None,
    /* 0xb7 */ None,
    /* 0xb8 */ None,
    /* 0xb9 */ None,
    /* 0xba */ None,
    /* 0xbb */ None,
    /* 0xbc */ None,
    /* 0xbd */ None,
    /* 0xbe */ None,
    /* 0xbf */ None,
    /* 0xc0 */ None,
    /* 0xc1 */ None,
    /* 0xc2 */ None,
    /* 0xc3 */ None,
    /* 0xc4 */ None,
    /* 0xc5 */ None,
    /* 0xc6 */ None,
    /* 0xc7 */ None,
    /* 0xc8 */ None,
    /* 0xc9 */ None,
    /* 0xca */ None,
    /* 0xcb */ None,
    /* 0xcc */ None,
    /* 0xcd */ None,
    /* 0xce */ None,
    /* 0xcf */ None,
    /* 0xd0 */ None,
    /* 0xd1 */ None,
    /* 0xd2 */ None,
    /* 0xd3 */ None,
    /* 0xd4 */ None,
    /* 0xd5 */ None,
    /* 0xd6 */ None,
    /* 0xd7 */ None,
    /* 0xd8 */ None,
    /* 0xd9 */ None,
    /* 0xda */ None,
    /* 0xdb */ None,
    /* 0xdc */ None,
    /* 0xdd */ None,
    /* 0xde */ None,
    /* 0xdf */ None,
    /* 0xe0 */ None,
    /* 0xe1 */ None,
    /* 0xe2 */ None,
    /* 0xe3 */ None,
    /* 0xe4 */ None,
    /* 0xe5 */ None,
    /* 0xe6 */ None,
    /* 0xe7 */ None,
    /* 0xe8 */ None,
    /* 0xe9 */ None,
    /* 0xea */ None,
    /* 0xeb */ None,
    /* 0xec */ None,
    /* 0xed */ None,
    /* 0xee */ None,
    /* 0xef */ None,
    /* 0xf0 */ Some("CREATE"),
    /* 0xf1 */ Some("CALL"),
    /* 0xf2 */ Some("CALLCODE"),
    /* 0xf3 */ Some("RETURN"),
    /* 0xf4 */ Some("DELEGATECALL"),
    /* 0xf5 */ Some("CREATE2"),
    /* 0xf6 */ None,
    /* 0xf7 */ None,
    /* 0xf8 */ None,
    /* 0xf9 */ None,
    /* 0xfa */ Some("STATICCALL"),
    /* 0xfb */ None,
    /* 0xfc */ None,
    /* 0xfd */ Some("REVERT"),
    /* 0xfe */ Some("INVALID"),
    /* 0xff */ Some("SELFDESTRUCT"),
]);
