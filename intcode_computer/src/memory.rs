use std::collections::HashMap;
use std::iter::{repeat};
use std::ops::{Index, IndexMut};
use crate::error::{MemoryError};
use std::fmt::Display;
use std::fmt::Debug;

const CACHE_LINE_SIZE: usize = 64;

pub type MemoryValueType = i64;

/// Simple memory paging. Page size is equal to the cache line size.
pub struct Memory {
    // todo remove unecessary hash map access via cache:
    // annoying because of ownership and borrowing
    //current_page: Option<(usize, &'a mut Box<[MemoryValueType]>)>,
    page_size: usize,
    page_mask: usize,
    page_table: HashMap<usize, Box<[MemoryValueType]>>
}


impl Memory {
    pub fn new() -> Self {
        let page_size = CACHE_LINE_SIZE / std::mem::size_of::<MemoryValueType>();
        let page_table = HashMap::new();
        Memory {
            //current_page: None,
            page_size,
            // calculates bit mask for base address with some bit magic
            page_mask: -(page_size as isize) as usize,
            page_table
        }
    }

    // could be inlined. But LLVM is probably smart enough
    fn table_index(&self, address: usize) -> usize {
        address & self.page_mask
    }

    fn page_index(&self, address: usize) -> usize {
        address & !self.page_mask
    }
 
    /// Copies the given data and stores it into the memory contiguously.
    /// IMPORTANT: Starting address has to be aligned to page size though the
    /// ending address does not have to be aligned.
    pub fn insert_contiguous(&mut self, starting_address: usize, values: &[MemoryValueType]) -> Result<(), MemoryError> {
        if starting_address % self.page_size != 0 {
            return Err(MemoryError::NotAligned {address: starting_address, page_size: self.page_size});
        }
        let mut address = starting_address;
        let chunks = values.chunks_exact(self.page_size);
        
        // get last chunk that does not fill a page
        let rest = chunks.remainder();

        for chunk in chunks {
            self.page_table.insert(address, chunk.to_vec().into_boxed_slice());
            address += self.page_size;
        }

        if rest.len() > 0 {
            let page = self.get_page_mut(address);
            // TODO: is there a more ergonomic way?
            for i in 0..rest.len() {
                page[i] = rest[i];
            }
        }

        Ok(())
    }

    // fn get_page(&self, address: usize) -> & Box<[MemoryValueType]> {
    //     &self.page_table[&self.table_index(address)]
    // }

    fn get_page_mut(&mut self, address: usize) -> &mut Box<[MemoryValueType]> {
        self.page_table.entry(self.table_index(address)).or_insert(
            repeat(0).take(self.page_size).collect::<Vec<MemoryValueType>>().into_boxed_slice()
        )
    } 
}

impl Index<usize> for Memory {
    type Output = MemoryValueType;

    fn index(&self, address: usize) -> &Self::Output {
        self.page_table.get(&self.table_index(address)).map(|s| &s[self.page_index(address)]).unwrap_or(&0)
    }
} 

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        let page_index = self.page_index(address);
        &mut self.get_page_mut(address)[page_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserts() {
        let mut mem: Memory = Memory::new();

        let values = vec![(26451, 10), (135, 126), (0, 36), (26452, 11)];

        for (i, v) in &values {
            mem[*i] = *v;
        }
        for (i, v) in &values {
            assert_eq!(mem[*i], *v, "Insert wrong");
        }

    }

    #[test]
    fn test_consecutive_test() {
        let mut mem: Memory = Memory::new();

        let s  = vec![10, 16246, 7371, 317, 234, 626, 1212, 253];
        let s1 = vec![10, 16246, 7371, 317, 234, 626, 1212, 253, 138, 12, 147, ];
        let s2 = vec![10, 16246, 7371, 317 ,234, 626, 1212, 253, 138, 12, 147, 23423, 23423, 243, 242, 153, 2];

        mem.insert_contiguous(0, &s).expect("Should be aligned");
        mem.insert_contiguous(7, &s).expect_err("Should not be aligned");

        mem.insert_contiguous(64, &s1).expect("Should be aligned");
        mem.insert_contiguous(128, &s2).expect("Should be aligned");


        for i in 0..s.len() {
            assert_eq!(s[i], mem[i], "Contiguous memory wrong");
        }
    }
}