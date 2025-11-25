use alloy_primitives::{I256, U256};

use crate::evm::{EVM, EvmError};

// gets one byte from a 32-byte Word
pub fn byte(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    let i = vm.stack.pop()?;
    let x = vm.stack.pop()?;

    let result = if i < U256::from(32) {
        // i < 32
        let index = i.to::<usize>();
        let bytes: [u8; 32] = x.to_be_bytes();
        U256::from(bytes[index])
    } else {
        // i >= 32 , the result is zero
        U256::ZERO
    };

    vm.stack.push(result)?;
    vm.pc += 1;
    Ok(())
}

// bit shift left
pub fn shl(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;
    let shift = vm.stack.pop()?;
    let value = vm.stack.pop()?;
    vm.stack.push(value << shift)?;

    vm.pc += 1;
    Ok(())
}

// bit shift right
pub fn shr(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;
    let shift = vm.stack.pop()?;
    let value = vm.stack.pop()?;
    vm.stack.push(value >> shift)?;

    vm.pc += 1;
    Ok(())
}

// what's the difference btw an Arithmetic shift and a normal shift, how's it done in binary terms
// signed shift right
pub fn sar(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;
    let shift = vm.stack.pop()?;
    let value = vm.stack.pop()?;

    // some code
    let result;

    if shift >= U256::from(256) {
        // If the number was negative (Sign Bit is 1), the result is -1 (All 1s).
        // If the number was positive (Sign Bit is 0), the result is 0.
        if value.bit(255) {
            result = U256::MAX;
        } else {
            result = U256::ZERO;
        }
    } else {
        let shift_amt = shift.to::<usize>();
        // cast value to I256
        let signed_value = I256::from_raw(value);
        // rust knows I256 is a signed value,
        // so it performs an Arithmetic shift i.e A shift which preserves the sign bit
        let shifted = signed_value >> shift_amt;
        result = shifted.into_raw();
    }

    vm.stack.push(result)?;
    vm.pc += 1;
    Ok(())
}
