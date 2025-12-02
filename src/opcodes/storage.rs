// use alloy_primitives::U256;

use alloy_primitives::U256;

use crate::evm::{EVM, EvmError};

// loads one word (32 bytes) from storage by a `key`` onto the stack
pub fn sload(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2100)?;

    let key = vm.stack.pop()?;
    let (_is_warm, word) = vm.storage.load(key);
    vm.stack.push(word)?;

    vm.pc += 1;
    Ok(())
}

// get the key and word from the stack, and store in storage, where storage[key] = value
pub fn s_store(vm: &mut EVM) -> Result<(), EvmError> {
    let key = vm.stack.pop()?;
    let new_value = vm.stack.pop()?;
    let (is_warm, old_value) = vm.storage.store(key, new_value);
    let mut base_dynamic_gas = 0u64;
    if new_value != old_value{
        if old_value == U256::ZERO{
            // very expensive, cos we storing in a new storage slot 
            // sort of like the total amount a tenant pays when moving into a new apartment
            base_dynamic_gas = 20000;
        } else if new_value == U256::ZERO{
            // old_value -> new_value : non_zero -> zero
            base_dynamic_gas = 2900;    // pay standard modification fee
            // refund
            vm.refund += 4800;
        }else{
            // old_value -> new_value : non_zero -> non_zero
            base_dynamic_gas = 2900;
        }
    }
    let access_cost = if is_warm {100u64} else {2100};
    vm.gas_dec(base_dynamic_gas + access_cost)?;
    vm.pc += 1;
    Ok(())
}