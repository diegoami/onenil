//! [designed] Optional initialization provenance for native setup before its Cpu handoff.
//! Existing untracked kernels keep their byte-image behavior. Tracked setup starts
//! unknown;declared input stores and native writes establish byte presence.
use std::{
    cell::Cell,
    fmt,
    ops::{
        Deref, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
    slice::SliceIndex,
};

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Image {
    bytes: Box<[u8]>,
    initialized: Option<Vec<bool>>,
    first_read: Cell<Option<usize>>,
}
impl From<Box<[u8]>> for Image {
    fn from(bytes: Box<[u8]>) -> Self {
        Self {
            bytes,
            initialized: None,
            first_read: Cell::new(None),
        }
    }
}
impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("len", &self.bytes.len())
            .field("tracked", &self.initialized.is_some())
            .field("first_uninitialized_read", &self.first_read.get())
            .finish()
    }
}
impl Deref for Image {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}
impl<'a> IntoIterator for &'a Image {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter()
    }
}
pub(crate) trait Span: SliceIndex<[u8]> + Clone {
    fn span(&self, len: usize) -> Range<usize>;
}
impl Span for usize {
    fn span(&self, _: usize) -> Range<usize> {
        *self..*self + 1
    }
}
impl Span for Range<usize> {
    fn span(&self, _: usize) -> Range<usize> {
        self.clone()
    }
}
impl Span for RangeFrom<usize> {
    fn span(&self, len: usize) -> Range<usize> {
        self.start..len
    }
}
impl Span for RangeTo<usize> {
    fn span(&self, _: usize) -> Range<usize> {
        0..self.end
    }
}
impl Span for RangeFull {
    fn span(&self, len: usize) -> Range<usize> {
        0..len
    }
}
impl Span for RangeInclusive<usize> {
    fn span(&self, _: usize) -> Range<usize> {
        *self.start()..*self.end() + 1
    }
}
impl Span for RangeToInclusive<usize> {
    fn span(&self, _: usize) -> Range<usize> {
        0..self.end + 1
    }
}
impl<I: Span> Index<I> for Image {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        let value = &self.bytes[index.clone()];
        self.observe(index.span(self.bytes.len()));
        value
    }
}
impl<I: Span> IndexMut<I> for Image {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        // Native mutable slice uses are complete copy_from_slice/fill stores;
        // there is deliberately no DerefMut exposing the entire unknown image.
        let _ = &self.bytes[index.clone()];
        if let Some(present) = &mut self.initialized {
            present[index.span(self.bytes.len())].fill(true);
        }
        &mut self.bytes[index]
    }
}
impl Image {
    pub(crate) fn tracked(len: usize) -> Self {
        Self {
            bytes: vec![0; len].into_boxed_slice(),
            initialized: Some(vec![false; len]),
            first_read: Cell::new(None),
        }
    }
    fn observe(&self, range: Range<usize>) {
        if self.first_read.get().is_none() {
            if let Some(present) = &self.initialized {
                if let Some(offset) = present[range.clone()].iter().position(|value| !value) {
                    self.first_read.set(Some(range.start + offset));
                }
            }
        }
    }
    pub(crate) fn check_initialized(&self) -> Result<(), String> {
        if let Some(address) = self.first_read.get() {
            Err(format!("native setup read uninitialized byte{address:05X}"))
        } else {
            Ok(())
        }
    }
    pub(crate) fn resize(&mut self, len: usize) {
        let mut bytes = std::mem::take(&mut self.bytes).into_vec();
        bytes.resize(len, 0);
        self.bytes = bytes.into_boxed_slice();
        if let Some(present) = &mut self.initialized {
            present.resize(len, false);
        }
    }
    pub(crate) fn copy_within(&mut self, source: Range<usize>, destination: usize) {
        self.bytes.copy_within(source.clone(), destination);
        if let Some(present) = &mut self.initialized {
            present.copy_within(source, destination);
        }
    }
    pub(crate) fn swap(&mut self, first: usize, second: usize) {
        self.observe(first..first + 1);
        self.observe(second..second + 1);
        self.bytes.swap(first, second);
        if let Some(present) = &mut self.initialized {
            present.swap(first, second);
        }
    }
    pub(crate) fn take_initialized(&mut self) -> Result<Vec<bool>, String> {
        self.check_initialized()?;
        self.initialized
            .take()
            .ok_or("native image has no initialization provenance".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_allocation_is_unknown_and_explicit_stores_define_only_their_cells() {
        let mut image = Image::tracked(20);
        image[3] = 0;
        image[6..9].fill(0);
        assert_eq!((image[3], image[7]), (0, 0));
        image.check_initialized().unwrap();
        let mask = image.take_initialized().unwrap();
        assert_eq!(mask.iter().filter(|v| **v).count(), 4);
        assert!(!mask[4]);
        let unknown = Image::tracked(20);
        assert_eq!(unknown[3], 0);
        assert!(unknown.check_initialized().unwrap_err().contains("00003"));
    }
    #[test]
    fn copying_and_growing_memory_preserves_unknown_byte_provenance() {
        let mut image = Image::tracked(20);
        image[2] = 9;
        image.copy_within(1..4, 10);
        image.resize(30);
        assert_eq!(image[11], 9);
        image.check_initialized().unwrap();
        let mask = image.take_initialized().unwrap();
        assert!(mask[2] && mask[11]);
        assert!(!mask[10] && !mask[12] && !mask[25]);
    }
}
