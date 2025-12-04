use std::cmp::min;

use alloy_primitives::U256;

use crate::evm::{EVM, EvmError};

// PUSH N 
// pushes items onto the stack
pub fn push(vm: &mut EVM, n:usize) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    let start_index = vm.pc + 1;
    let mut data_bytes = vec![0u8; n];
    if start_index < vm.program.len(){
        let available_len = vm.program.len() - start_index;
        let copy_len = min(available_len, n);
        // copy code slice into byte_array
        let code_slice = &vm.program[start_index..start_index+copy_len];
        data_bytes[..copy_len].copy_from_slice(code_slice);
    }
    let value = U256::from_be_slice(&data_bytes);
    vm.stack.push(value)?;
    vm.pc += 1 + n; // added 1 because We skip the Opcode
    Ok(())
}
