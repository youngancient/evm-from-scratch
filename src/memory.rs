use crate::evm::EvmError;

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }
    pub fn ensure_capacity(&mut self, offset: usize, size: usize) {
        let required_len = offset.saturating_add(size);
        if required_len > self.memory.len() {
            self.memory.resize(required_len, 0);
        }
    }
    pub fn access(&self, offset: usize, size: usize) -> Result<&[u8],EvmError> {
        let end = offset.saturating_add(size);
        if end > self.memory.len() {
            return Err(EvmError::MemoryOutOfBounds { offset, size, max: self.memory.len() })
        }
        Ok(&self.memory[offset..end])
    }
    pub fn load(&mut self, offset: usize) -> [u8; 32] {
        const WORD_SIZE: usize = 32;
        let required_len = offset.saturating_add(WORD_SIZE);
        self.ensure_capacity(offset, WORD_SIZE);
        let mut word = [0u8; WORD_SIZE];
        let available_data = &self.memory[offset..required_len];
        word.copy_from_slice(available_data);
        word
    }
    pub fn store(&mut self, offset: usize, value: &[u8]) {
        let size = value.len();
        if size == 0 {
            return;
        }
        self.ensure_capacity(offset, size); // expands memory to be able to store value
        let dest = &mut self.memory[offset..offset + size];
        dest.copy_from_slice(value);
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    fn init_memory() -> Memory {
        Memory::new()
    }

    #[test]
    fn test_memory_init() {
        let mem = init_memory();
        assert_eq!(mem.memory.len(), 0)
    }

    #[test]
    fn test_store() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(mem.memory.len(), 4);
    }

    #[test]
    fn test_access() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_access_success() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
        let result = mem.access(1, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),&[0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_access_failure() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
        // should return EvmError
        let result = mem.access(1, 5);
        assert!(result.is_err());
        let expected_error = EvmError::MemoryOutOfBounds { offset: 1, size: 5, max: 4 };
        assert_eq!(result.unwrap_err(), expected_error)
    }

    #[test]
    fn test_load() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(
            mem.load(0),
            [
                1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_load_from_offset_1() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(
            mem.load(1),
            [
                2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        );
    }
    #[test]
    fn test_load_from_offset_2() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(
            mem.load(3),
            [
                4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,0,0
            ]
        );
    }
}
