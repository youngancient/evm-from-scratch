pub const STOP: u8 = 0x00;
// arithmetic
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const SDIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const SMOD: u8 = 0x07;
pub const ADDMOD: u8 = 0x08;
pub const MULMOD: u8 = 0x09;
pub const EXP: u8 = 0x0A;
pub const SIGNEXTEND: u8 = 0xB;

// comparison
pub const LT: u8 = 0x10;
pub const GT: u8 = 0x11;
pub const SLT: u8 = 0x12;
pub const SGT: u8 = 0x13;
pub const EQ: u8 = 0x14;
pub const ISZERO: u8 = 0x15;

// logical
pub const AND: u8 = 0x16;
pub const OR: u8 = 0x17;
pub const XOR: u8 = 0x18;
pub const NOT: u8 = 0x19;
pub const BYTE: u8 = 0x1A;  // specific byte extractio
pub const SHL: u8 = 0x1B;   // shift left 
pub const SHR: u8 = 0x1C;   // shift right
pub const SAR: u8 = 0x1D;   // arithmetic shift right

pub const SHA3: u8 = 0x20;

// environment
pub const ADDRESS:        u8 = 0x30; // address(this)
pub const BALANCE:        u8 = 0x31; // address(this).balance
pub const ORIGIN:         u8 = 0x32; // tx.origin (Signer)
pub const CALLER:         u8 = 0x33; // msg.sender (Immediate caller)
pub const CALLVALUE:      u8 = 0x34; // msg.value
pub const CALLDATALOAD:   u8 = 0x35; // Load 32 bytes of input
pub const CALLDATASIZE:   u8 = 0x36; // Size of input
pub const CALLDATACOPY:   u8 = 0x37; // Copy input to memory
pub const CODESIZE:       u8 = 0x38; // Size of running code
pub const CODECOPY:       u8 = 0x39; // Copy running code to memory
pub const GASPRICE:       u8 = 0x3A; // tx.gasprice
pub const EXTCODESIZE:    u8 = 0x3B; // Size of external contract code
pub const EXTCODECOPY:    u8 = 0x3C; // Copy external contract code
pub const RETURNDATASIZE: u8 = 0x3D; // Size of last call's return data
pub const RETURNDATACOPY: u8 = 0x3E; // Copy last call's return data
pub const EXTCODEHASH:    u8 = 0x3F; // Hash of external contract code

// block information
// pub const BLOCKHASH:   u8 = 0x40; // Hash of recent blocks
// pub const COINBASE:    u8 = 0x41; // block.coinbase (Validator address)
// pub const TIMESTAMP:   u8 = 0x42; // block.timestamp
// pub const NUMBER:      u8 = 0x43; // block.number
// pub const DIFFICULTY:  u8 = 0x44; // block.difficulty (Now PREVRANDAO)
// pub const GASLIMIT:    u8 = 0x45; // block.gaslimit
// pub const CHAINID:     u8 = 0x46; // chainid (e.g., 1 for Mainnet)
// pub const SELFBALANCE: u8 = 0x47; // Cheaper version of BALANCE(address(this))
// pub const BASEFEE:     u8 = 0x48; // EIP-1559 Base Fee


pub const POP: u8 = 0x50;

// memory
pub const MLOAD: u8 = 0x51;
pub const MSTORE:   u8 = 0x52; // Store to memory
pub const MSTORE8:  u8 = 0x53; // Store 1 byte to memory

// storage
pub const SLOAD:    u8 = 0x54; // Load from storage
pub const SSTORE:   u8 = 0x55; // Store to storage

// jump
pub const JUMP: u8 = 0x56;
pub const JUMPI: u8 = 0x57;
pub const PC:       u8 = 0x58; // Get Program Counter
pub const GAS:      u8 = 0x5A; // Get remaining Gas
pub const JUMPDEST: u8 = 0x5B;

// transient storage
pub const TLOAD: u8 = 0x5C;
pub const TSTORE: u8 = 0x5D;

pub const PUSH1: u8 = 0x60;
pub const PUSH32: u8 = 0x7F;

pub const DUP1: u8 = 0x80;
pub const DUP16: u8 = 0x8F;

// 0x90 range - SWAP
pub const SWAP1:  u8 = 0x90;
pub const SWAP16: u8 = 0x9F;

// 0xA0 range - LOG
pub const LOG0:   u8 = 0xA0;
pub const LOG4:   u8 = 0xA4;

pub const REVERT:       u8 = 0xFD; 