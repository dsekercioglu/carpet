const HEAP_SIZE: usize = 65536;

#[derive(Debug)]
pub struct CVMHeap {
    heap: Vec<u32>,
    sizes: Vec<(usize, usize)>,
}

impl CVMHeap {
    pub fn new() -> Self {
        Self {
            heap: vec![0u32; HEAP_SIZE],
            sizes: vec![(HEAP_SIZE - 1, 0)],
        }
    }

    pub fn value_at(&mut self, ptr: usize) -> &mut u32 {
        &mut self.heap[ptr]
    }

    pub fn alloc(&mut self, size: usize) -> usize {
        let mut alloc_index = 0;
        for &(ptr, size) in &self.sizes {
            let dist_to_ptr = ptr - alloc_index;
            if size < dist_to_ptr {
                break;
            }
            alloc_index = ptr + size;
        }
        if alloc_index >= HEAP_SIZE - 1 {
            panic!("CVM out of Heap");
        }
        let insert_pos = self.sizes.binary_search_by_key(
            &alloc_index,
            |&(ptr, _)| ptr,
        ).unwrap_or_else(|index| index);
        self.sizes.insert(insert_pos, (alloc_index, size));
        alloc_index
    }

    pub fn free(&mut self, pointer: usize) {
        let index = self.sizes.iter().position(
            |(ptr, _)| *ptr == pointer
        );
        let index = index.expect("already free");
        self.sizes.remove(index);
    }
}