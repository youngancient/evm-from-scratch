use crate::evm::{EVM, EvmError};

pub fn stop(vm: &mut EVM) -> Result<(), EvmError> {
    vm.stop_flag = true;
    Ok(())
}
