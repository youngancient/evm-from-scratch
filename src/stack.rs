use alloy_primitives::U256;

use crate::evm::EvmError;

pub const MAXIMUM_STACK_SIZE:usize = 1024;
#[derive(Debug,Clone)]
pub struct Stack{
    items : Vec<U256>
}

impl Stack {
    pub fn new() -> Self{
        Self { items: Vec::with_capacity(MAXIMUM_STACK_SIZE) }
    }
    pub fn push(&mut self, item: U256)-> Result<(), EvmError>{
        if self.items.len() + 1 > MAXIMUM_STACK_SIZE{
            return Err(EvmError::StackOverflow)
        }
        self.items.push(item);
        Ok(())
    }

    pub fn pop(&mut self)-> Result<U256, EvmError>{
        self.items.pop().ok_or(EvmError::StackUnderflow)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn create_stack()-> Stack{
        Stack::new()
    }

    #[test]
    fn test_create_stack(){
        assert_eq!(create_stack().items.len(),0);
    }

    #[test]
    fn test_push_item_to_stack(){
        let mut new_stack = create_stack();
        new_stack.push(U256::from(0x9222));
        new_stack.push(U256::from(0x87222));
        assert_eq!(new_stack.items.len(),2);
    }
    // fix
    // #[test]
    // fn test_should_panic_if_stack_overflow(){
    //     let mut new_stack = create_stack();
    //     for _ in 0..=1024{
    //         new_stack.push(U256::from(0x9222));
    //     }
    // }

    #[test]
    fn test_pop_item_from_stack(){
        let mut new_stack = create_stack();
        new_stack.push(U256::from(0x9222));
        new_stack.push(U256::from(0x87222));
        new_stack.pop();
        assert_eq!(new_stack.items[0],U256::from(0x9222));
        assert_eq!(new_stack.items.len(),1);
    }
    // fix
    // #[test]
    // fn test_should_panic_if_stack_underflow(){
    //     let mut new_stack = create_stack();
    //     new_stack.push(U256::from(0x9222));
    //     new_stack.push(U256::from(0x87222));
    //     for _ in 0..5{
    //         new_stack.pop();
    //     }
    // }
}