use alloy_primitives::U256;

use crate::evm::{EVM, EvmError, Log};

pub fn log(vm: &mut EVM, n: usize) -> Result<(), EvmError> {
    let offset_raw = vm.stack.pop()?;
    let size_raw = vm.stack.pop()?;

    let offset = offset_raw.saturating_to::<usize>();
    let size = size_raw.saturating_to::<usize>();
    // pop n-topics from stack and store in topics Vec
    let mut topics: Vec<U256> = Vec::new();
    for _ in 0..n {
        topics.push(vm.stack.pop()?);
    }
    let static_gas = 375;
    let topic_gas = static_gas * n as u64;
    let data_gas = 8 * (size as u64);

    let expansion_cost = vm.memory.ensure_capacity(offset, size);
    vm.gas_dec(static_gas + topic_gas + data_gas + expansion_cost)?;

    let data = vm.memory.access(offset, size)?.to_vec();
    // create log with data and topics
    let log_entry = Log::new(data, topics);
    vm.logs.push(log_entry);
    vm.pc += 1;
    Ok(())
}
