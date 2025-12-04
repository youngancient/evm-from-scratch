use alloy_primitives::{I256, U256};

use crate::evm::{EVM, EvmError};

pub fn add(vm: &mut EVM) -> Result<(), EvmError> {
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

pub fn mul(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(5)?;

    // pop the 2 values
    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;
    // multiply and push to the stack
    vm.stack.push(a.wrapping_mul(b))?;
    // increase pc
    vm.pc += 1;
    Ok(())
}

pub fn sub(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    // pop the 2 values
    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;
    // add them
    vm.stack.push(a.wrapping_sub(b))?;
    // increase pc
    vm.pc += 1;
    Ok(())
}

pub fn div(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(5)?;

    // pop the 2 values
    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;
    // divide and push to the stack
    let result = if b == U256::ZERO { U256::ZERO } else { a / b };
    vm.stack.push(result)?;
    // increase pc
    vm.pc += 1;
    Ok(())
}

pub fn sdiv(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(5)?;

    // pop the 2 values
    let a_raw = vm.stack.pop()?;
    let b_raw = vm.stack.pop()?;

    let a = I256::from_raw(a_raw);
    let b = I256::from_raw(b_raw);

    // multiply and push to the stack
    let result = if b == I256::ZERO {
        I256::ZERO
    } else if a == I256::MIN && b == I256::MINUS_ONE {
        I256::MIN
    } else {
        a / b
    };

    vm.stack.push(result.into_raw())?;
    // // increase pc
    vm.pc += 1;
    Ok(())
}

pub fn vm_mod(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(5)?;
    // pop the 2 values
    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;
    let result = if b == U256::ZERO { U256::ZERO } else { a % b };
    // push result to Evm stack
    vm.stack.push(result)?;
    // increase pc
    vm.pc += 1;
    Ok(())
}

pub fn smod(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(5)?;
    // pop the 2 values
    let a_raw = vm.stack.pop()?;
    let b_raw = vm.stack.pop()?;
    //  to signed
    let a = I256::from_raw(a_raw);
    let b = I256::from_raw(b_raw);

    if b == I256::ZERO {
        vm.stack.push(U256::ZERO)?;
        return Ok(());
    }

    let result = a % b;
    vm.stack.push(result.into_raw())?;
    vm.pc += 1;
    Ok(())
}

pub fn add_mod(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(8)?;
    // pop the 2 values
    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;
    let n = vm.stack.pop()?;
    let result = if n == U256::ZERO {
        U256::ZERO
    } else {
        a.add_mod(b, n)
    };

    vm.stack.push(result)?;
    vm.pc += 1;
    Ok(())
}

pub fn mul_mod(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(8)?;
    // pop the 2 values
    let a = vm.stack.pop()?;
    let b = vm.stack.pop()?;
    let n = vm.stack.pop()?;
    let result = if n == U256::ZERO {
        U256::ZERO
    } else {
        a.mul_mod(b, n)
    };

    vm.stack.push(result)?;
    vm.pc += 1;
    Ok(())
}

pub fn size_in_bytes(num: &U256) -> u64 {
    let bits = num.bit_len() as u64;
    (bits + 7) / 8
}

pub fn exp(vm: &mut EVM) -> Result<(), EvmError> {
    // pop the 2 values
    let base = vm.stack.pop()?;
    let exponent = vm.stack.pop()?;

    // calculate gas
    let exponent_byte_len = size_in_bytes(&exponent);
    let gas_cost = 10 + (50 * exponent_byte_len);
    vm.gas_dec(gas_cost)?;

    let result = base.pow(exponent);

    vm.stack.push(result)?;

    vm.pc += 1;

    Ok(())
}

pub fn signextend(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(5)?;
    // pop the 2 values
    let size_marker = vm.stack.pop()?;
    let value: alloy_primitives::Uint<256, 4> = vm.stack.pop()?;
    let result;
    if size_marker < U256::from(31) {
        let byte_index = size_marker.to::<usize>();
        // Calculate which bit is the "Sign Bit"
        // (byte_index * 8) gets us to the start of the byte
        // + 7 gets us to the most significant bit of that byte
        let bit_index = (byte_index * 8) + 7;
        // Logic: (1 << (7 + 1)) - 1  =  256 - 1  =  255 (0xFF)
        let mask = (U256::ONE << (bit_index + 1)) - U256::ONE;
        if value.bit(bit_index) {
            // the number is negative
            result = value | !mask;
        } else {
            // the number is positive
            result = value & mask;
        }
    } else {
        // if the size is greater or equals 31, the number is of full width already
        // no extension is needed
        result = value;
    }
    vm.stack.push(result)?;
    vm.pc += 1;
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_size_in_bytes() {
        assert_eq!(size_in_bytes(&U256::from(12345)), 2);
        assert_eq!(size_in_bytes(&U256::ZERO), 0);
    }
}
