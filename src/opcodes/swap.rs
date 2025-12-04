use crate::evm::{EVM, EvmError};

// swaps the top of the stack with another item on the stack given by n
// SWAP (n) => SWAP n - 1 => SWAP stack.len() - 1 - n
// SWAP 0 => SWAP stack.len() - 1 - 0 => SWAP stack.len() - 1 => top of the stack
pub fn swap(vm: &mut EVM, n: usize) -> Result<(), EvmError> {
    vm.gas_dec(3)?;
    let stack_len = vm.stack.len();
    // we want to swap n items, the stack must have n + 1 items
    if stack_len <= n {
        return Err(EvmError::StackUnderflow);
    }
    let top_index = stack_len - 1;
    let other_index = stack_len - 1 - n;
    vm.stack.items.swap(top_index, other_index);
    vm.pc += 1;
    Ok(())
}
