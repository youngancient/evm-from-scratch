// these opcodes give access to the ethereum environment

use std::cmp::min;

use alloy_primitives::U256;

use crate::evm::{EVM, EvmError};

pub fn address(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2)?;
    vm.stack.push(vm.sender.into_word().into())?;
    vm.pc += 1;
    Ok(())
}

pub fn balance(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2600)?;
    let _address = vm.stack.pop()?;
    vm.stack.push(U256::from(9999999))?;
    vm.pc += 1;
    Ok(())
}

pub fn origin(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2)?;
    vm.stack.push(vm.sender.into_word().into())?;
    vm.pc += 1;
    Ok(())
}

// pub fn caller(vm: &mut EVM) -> Result<(), EvmError> {
//     vm.gas_dec(2)?;
//     vm.stack.push("0x414b60745072088d013721b4a28a0559b1A9d213")?;
//     vm.pc += 1;
//     Ok(())
// }

pub fn call_value(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2)?;
    vm.stack.push(vm.value)?;
    vm.pc += 1;
    Ok(())
}

// reads input data of stuffs like functions into the Stack
// also handles infinite zero shi
pub fn call_data_load(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(3)?;

    let i = vm.stack.pop()?;
    let start_index = i.saturating_to::<usize>();
    let mut word_bytes = [0u8; 32]; // handles infinite zeros automatically
    // infinite zero case: if the size to be copied is larger than what is available,
    // e.g we want to copy 32 bytes, but what is available is 2 bytes,
    // then the 2 bytes is copied and 0x00 is appended to complete the rest 30 bytes
    // checks if we have data to copy
    if start_index < vm.calldata.len() {
        // the end index is either the start_index + 32 or the calldata.len
        let end_index = min(start_index + 32, vm.calldata.len());
        let data = &vm.calldata[start_index..end_index];
        word_bytes[..data.len()].copy_from_slice(data);
    }
    vm.stack.push(U256::from_be_bytes(word_bytes))?;
    vm.pc += 1;
    Ok(())
}

// pushes the size of data onto the stack
pub fn call_data_size(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2)?;
    vm.stack.push(U256::from(vm.calldata.len()))?;
    vm.pc += 1;

    Ok(())
}

// stores a specific part of the calldata in memory
pub fn call_data_copy(vm: &mut EVM) -> Result<(), EvmError> {
    let dest_offset_raw = vm.stack.pop()?;
    let src_offset_raw = vm.stack.pop()?;
    let size_raw = vm.stack.pop()?;

    let src_offset = src_offset_raw.saturating_to::<usize>();
    let size = size_raw.saturating_to::<usize>();
    let dest_offset = dest_offset_raw.saturating_to::<usize>();

    let mut data = vec![0u8; size];
    // copy if source offset is within the bound of calldata.len()
    if src_offset < vm.calldata.len() {
        let available_bytes = vm.calldata.len() - src_offset;
        let copy_len = min(size, available_bytes);
        let copy_slice = &vm.calldata[src_offset..src_offset + copy_len];
        data[..copy_len].copy_from_slice(copy_slice);
    }
    // store in memory
    let expansion_cost = vm.memory.store(dest_offset, &data);

    // calculate gas
    let min_word_size = (size as u64 + 31) / 32;
    let dynamic_gas = 3 * min_word_size + expansion_cost;
    let static_gas = 3u64;
    vm.gas_dec(dynamic_gas + static_gas)?;

    vm.pc += 1;
    Ok(())
}

// puts the size of the running program on the stack
pub fn code_size(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2)?;
    vm.stack.push(U256::from(vm.program.len()))?;
    vm.pc += 1;

    Ok(())
}

// stores a specified part of the program in memory
pub fn code_copy(vm: &mut EVM) -> Result<(), EvmError> {
    let dest_offset_raw = vm.stack.pop()?;
    let src_offset_raw = vm.stack.pop()?;
    let size_raw = vm.stack.pop()?;

    let src_offset = src_offset_raw.saturating_to::<usize>();
    let size = size_raw.saturating_to::<usize>();
    let dest_offset = dest_offset_raw.saturating_to::<usize>();

    let mut data = vec![0u8; size];
    // copy if source offset is within the bound of program.len()
    if src_offset < vm.program.len() {
        let available_bytes = vm.program.len() - src_offset;
        let copy_len = min(size, available_bytes);
        let copy_slice = &vm.program[src_offset..src_offset + copy_len];
        data[..copy_len].copy_from_slice(copy_slice);
    }
    // store copied program in memory
    let expansion_cost = vm.memory.store(dest_offset, &data);
    // calculate gas
    let min_word_size = (size as u64 + 31) / 32;
    let dynamic_gas = 3 * min_word_size + expansion_cost;
    let static_gas = 3u64;
    vm.gas_dec(dynamic_gas + static_gas)?;

    vm.pc += 1;
    Ok(())
}

// The current gas price. Because we are running everything locally, the gas price is simply 0.
pub fn gas_price(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2)?;
    vm.stack.push(U256::ZERO)?;
    vm.pc += 1;
    Ok(())
}

// checks the size of a code at an address
// push zero because we'll be running this locally
pub fn ext_code_size(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2600)?;
    let _address = vm.stack.pop()?; // pops address off the stack
    vm.stack.push(U256::ZERO)?;
    vm.pc += 1;
    Ok(())
}

pub fn ext_code_copy(vm: &mut EVM) -> Result<(), EvmError> {
    todo!()
}

pub fn return_data_size(vm: &mut EVM) -> Result<(), EvmError> {
    vm.gas_dec(2)?;
    vm.stack.push(U256::ZERO)?;
    vm.pc += 1;
    Ok(())
}

// Stores a specified part of the previous return data in memory
pub fn return_data_copy(vm: &mut EVM) -> Result<(), EvmError> {
    let dest_offset_raw = vm.stack.pop()?;
    let src_offset_raw = vm.stack.pop()?;
    let size_raw = vm.stack.pop()?;

    let dest_offset = dest_offset_raw.saturating_to::<usize>();
    let src_offset = src_offset_raw.saturating_to::<usize>();
    let size = size_raw.saturating_to::<usize>();

    let data = vec![0u8;size];
    if  src_offset < vm.program.len(){
        
    }
    todo!()
}

// The hash of another program given by its address. 
// There are no other programs in our simplified world so we simply return 0.
pub fn ext_code_hash(vm: &mut EVM) -> Result<(), EvmError> {
    let _address = vm.stack.pop()?;
    vm.gas_dec(2600)?;
    vm.stack.push(U256::ZERO)?;
    vm.pc += 1;
    Ok(())
}

// blockhash, coinbase, timestamp, prevrandao