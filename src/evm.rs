use alloy_primitives::{Address, U256};

use crate::{memory::Memory, stack::Stack, storage::Storage};

#[derive(Debug,PartialEq)]
pub enum EvmError {
    OutOfGas,
    StackUnderflow,
    StackOverflow,
    MemoryOutOfBounds { offset: usize, size: usize, max: usize },
    ReturnDataOutOfBounds { offset: usize, size: usize, max: usize },
}
pub struct Log {
    pub topics: Vec<U256>,
    pub data: Vec<u8>,
}

pub struct EVM {
    pub pc: usize,
    pub value: U256,
    pub calldata: Vec<u8>,
    pub gas: u64,
    pub refund: u64,    // refunds can not pay for transactions themselves, they like vouchers given on transaction execution
    pub sender: Address,
    // sub components
    pub program: Vec<u8>,
    pub stack: Stack,
    pub memory: Memory,
    pub storage: Storage,
    // flags
    pub stop_flag: bool,
    pub revert_flag: bool,
    // output
    pub return_data: Vec<u8>,
    pub logs: Vec<Log>,
}

impl EVM {
    pub fn new(
        sender: Address,
        program: Vec<u8>,
        gas: u64,
        value: U256,
        calldata: Vec<u8>,
    ) -> Self {
        Self {
            pc: 0,
            value,
            sender,
            calldata,
            program,
            gas,
            refund : 0,
            stop_flag: false,
            revert_flag: false,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            return_data: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn gas_dec(&mut self, amount: u64) -> Result<(), EvmError> {
        if amount > self.gas{
            return Err(EvmError::OutOfGas);
        }
        self.gas -= amount;
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_state() {}
}
