use crate::evm::{EVM, EvmError};


pub fn add(vm : &mut EVM) -> Result<(), EvmError>{
    vm.gas_dec(3)?;

    // pop the 2 values
    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;
    // add them
    vm.stack.push(a.wrapping_add(b))?;
    // increase pc
    vm.pc += 1;
    Ok(())
}

