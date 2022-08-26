use core::mem::MaybeUninit;

#[derive(Debug)]
#[repr(u8)]
pub enum LinkedListError {
    NoSpace,
    BufferTooSmall,
}

#[repr(C)]
pub struct LinkedListStatic<T, const S: usize> {
    items: [MaybeUninit<T>; S],
    current_size: usize,
}

impl<T, const S: usize> LinkedListStatic<T, S> {
    const ELEM: MaybeUninit<T> = MaybeUninit::uninit();
    const INIT: [MaybeUninit<T>; S] = [Self::ELEM; S]; // important for optimization of `new`

    pub fn add(&mut self, item: T) -> Result<(), LinkedListError> {
        /* Check if there is space */
        if self.current_size == S {
            return Err(LinkedListError::NoSpace);
        }
        self.items[self.current_size] = MaybeUninit::new(item);
        self.current_size += 1;

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            items: Self::INIT,
            current_size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.current_size
    }
}

impl<T, const S: usize> core::ops::Index<usize> for LinkedListStatic<T, S> {
    type Output = T;
    fn index<'a>(&'a self, i: usize) -> &'a T {
        unsafe { self.items[i].assume_init_ref() }
    }
}
