// These opcodes behave almost identically to storage but changes are discarded after every transaction.

use crate::evm::{EVM, EvmError};

pub fn tload(vm : &mut EVM) -> Result<(), EvmError>{
    vm.gas_dec(100)?;
    let key = vm.stack.pop()?;
    let (_is_warm, value) = vm.storage.load(key);
    vm.stack.push(value)?;
    vm.pc += 1;
    Ok(())
}

pub fn tstore(vm : &mut EVM) -> Result<(), EvmError>{
    vm.gas_dec(100)?;
    let key = vm.stack.pop()?;
    let value = vm.stack.pop()?;
    vm.storage.store(key, value);
    vm.pc += 1;
    Ok(())
}