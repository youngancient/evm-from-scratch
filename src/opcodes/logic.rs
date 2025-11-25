use crate::evm::{EVM, EvmError};


pub fn and(vm : &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;

    vm.stack.push(a & b)?;
    vm.pc += 1;

    Ok(())
}

pub fn or(vm : &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;

    vm.stack.push(a | b)?;
    vm.pc += 1;

    Ok(())
}

pub fn xor(vm : &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;

    vm.stack.push(a ^ b)?;
    vm.pc += 1;

    Ok(())
}

pub fn not(vm : &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    let val = vm.stack.pop()?;

    vm.stack.push(!val)?;
    vm.pc += 1;

    Ok(())
}