// These opcodes behave almost identically to storage but changes are discarded after every transaction.

use alloy_primitives::U256;

use crate::evm::{EVM, EvmError};

// loads data from storage to stack temporarily
pub fn tload(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(100)?;
    let key = vm.stack.pop()?;
    let value = *vm.transient_storage.get(&key).unwrap_or(&U256::ZERO);
    vm.stack.push(value)?;
    vm.pc += 1;
    Ok(())
}

// stores data in storage temporarily i.e if I run TSTORE(key=1,val=99) and run SLOAD(key=1), I should not see 99
pub fn tstore(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(100)?;
    let key = vm.stack.pop()?;
    let value = vm.stack.pop()?;
    if value == U256::ZERO {
        vm.transient_storage.remove(&key);
    } else {
        vm.transient_storage.insert(key, value);
    }
    vm.pc += 1;
    Ok(())
}
