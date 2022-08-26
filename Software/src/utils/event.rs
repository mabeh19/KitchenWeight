use core::mem::MaybeUninit;

pub enum EventError {
    NoSpace,
}

pub struct Subscriber<F>
where
    F: FnMut() -> (),
{
    callback: F,
    name: [char; 10],
}

impl<F> Subscriber<F>
where
    F: FnMut() -> (),
{
    pub fn new(callback: F, name: &str) -> Self {
        let as_chars = name.chars();
        let mut n: [char; 10] = ['\0'; 10];
        let mut i: usize = 0;
        for c in as_chars {
            n[i] = c;
            i += 1;
        }
        Subscriber {
            callback: callback,
            name: n,
        }
    }

    pub fn call(&mut self) {
        (self.callback)();
    }
}

pub struct Event<F, const S: usize>
where
    F: FnMut() -> (),
{
    callbacks: [MaybeUninit<Subscriber<F>>; S],
    actual_len: usize,
}

impl<F, const S: usize> Event<F, S>
where
    F: FnMut() -> (),
{
    const ELEM: MaybeUninit<Subscriber<F>> = MaybeUninit::uninit();
    const INIT: [MaybeUninit<Subscriber<F>>; S] = [Self::ELEM; S]; // important for optimization of `new`

    pub fn new() -> Self {
        Event {
            callbacks: Self::INIT,
            actual_len: 0,
        }
    }

    pub fn subscribe(&mut self, callback: F, name: &str) -> Result<(), EventError> {
        if self.actual_len == S {
            return Err(EventError::NoSpace);
        } else {
            self.callbacks[self.actual_len] = MaybeUninit::new(Subscriber::new(callback, name));
            self.actual_len += 1;
            Ok(())
        }
    }

    pub fn raise(&mut self) {
        for i in 0..self.actual_len {
            unsafe {
                //self.callbacks[i].assume_init_ref().call();
            }
        }
    }
}
