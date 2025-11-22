pub struct Memory {
    memory: Vec<u8>,
}
// FIX: change panics to Err
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
    pub fn access(&self, offset: usize, size: usize) -> &[u8] {
        let end = offset.saturating_add(size);
        if end > self.memory.len() {
            panic!(
                "Memory access out of bounds: tried to access {} bytes at offset {} but memory is only {} bytes long",
                size,
                offset,
                self.memory.len()
            );
        }
        self.memory
            .get(offset..end)
            .expect("Memory bounds check failed unexpectedly after explicit check")
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
    #[should_panic]
    fn test_access_should_fail() {
        let mut mem = init_memory();
        mem.store(0, &[0x01, 0x02, 0x03, 0x04]);
        // should panic because offset n size do not fit the available memory
        let m = mem.access(1, 5);
        println!("{:?}",m);
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
