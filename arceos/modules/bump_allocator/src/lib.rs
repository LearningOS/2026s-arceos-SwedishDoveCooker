#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    last_paddle: usize, // toy
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            last_paddle: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        // todo!()
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = start + size;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> allocator::AllocResult {
        todo!()
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        // todo!()
        let align = layout.align();
        let size = layout.size();
        let aligned_b_pos =
            align_up(self.b_pos, align).ok_or(allocator::AllocError::InvalidParam)?;
        let next_b_pos = aligned_b_pos
            .checked_add(size)
            .ok_or(allocator::AllocError::NoMemory)?;
        if next_b_pos > self.p_pos {
            return Err(allocator::AllocError::NoMemory);
        }
        if let Some(ptr) = core::ptr::NonNull::new(aligned_b_pos as *mut u8) {
            self.last_paddle = self.b_pos;
            self.b_pos = next_b_pos;
            Ok(ptr)
        } else {
            Err(allocator::AllocError::InvalidParam)
        }
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        // todo!()
        // ¿
        let size = layout.size();
        let ptr = pos.as_ptr() as usize;
        if ptr < self.start || ptr >= self.b_pos {
            return;
        }
        if ptr + size == self.b_pos {
            self.b_pos = ptr - self.last_paddle;
            self.last_paddle = 0;
        }
    }

    fn total_bytes(&self) -> usize {
        todo!()
    }

    fn used_bytes(&self) -> usize {
        // todo!()
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        // todo!()
        self.p_pos - self.b_pos
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        // todo!()
        let align = align_pow2;
        let size = num_pages
            .checked_mul(Self::PAGE_SIZE)
            .ok_or(allocator::AllocError::InvalidParam)?;
        let next_p_pos = align_down(self.p_pos - size, align);
        if next_p_pos < self.b_pos {
            return Err(allocator::AllocError::NoMemory);
        }
        self.p_pos = next_p_pos;
        Ok(self.p_pos)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        // todo!()
        (self.end - self.p_pos) / Self::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        // todo!()
        (self.p_pos - self.b_pos) / Self::PAGE_SIZE
    }
}

#[inline]
fn align_up(addr: usize, align: usize) -> Option<usize> {
    debug_assert!(align.is_power_of_two());
    let mask = align - 1;
    addr.checked_add(mask).map(|x| x & !mask)
}

fn align_down(addr: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    addr & !(align - 1)
}
