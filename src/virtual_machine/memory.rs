use std::ops::{Index, DerefMut, Deref, IndexMut};

use super::FontSet;



#[derive(Debug)]
pub struct Memory {
    pub(crate) _inner: [u8; 4096]
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            _inner: [0; 4096]
        }
    }
}

impl Deref for Memory {
    type Target = [u8; 4096];
    fn deref(&self) -> &Self::Target {
        &self._inner
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._inner
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._inner.index_mut(index)  
    }
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load_font_data(&mut self, fontset: &FontSet, offset: usize) {
        let fontset_size = fontset.0.len();

        for idx in 0..fontset_size {
            self._inner[offset + idx] = fontset.0[idx];
        }
    }
}

impl Index<usize> for Memory {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self._inner[index]
    }
}