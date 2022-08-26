use arduino_hal::{
    pac::USART0,
    port::{mode, Pin},
};
use core::cell::RefCell;
use core::mem::uninitialized;
use ufmt::uWrite;

pub type UsartReference = RefCell<UsartController>;

pub enum UsartError {}

pub struct UsartController {
    usart: USART0,
    rx: Pin<mode::Input<mode::Floating>>,
    tx: Pin<mode::Output>,
}

impl UsartController {
    pub fn new(usart: USART0, rx: Pin<mode::Input<mode::Floating>>, tx: Pin<mode::Output>) -> Self {
        UsartController {
            usart: usart,
            rx: rx,
            tx: tx,
        }
    }

    pub fn init(&mut self, baudrate: u32) {
        let ubrr: u16 = (16_000_000 / (16 * baudrate) - 1) as u16;

        self.usart.ubrr0.write(|w| unsafe { w.bits(ubrr & 0x0FFF) });

        /* Enable RX/TX pins */
        self.usart.ucsr0b.write(|w| unsafe { w.bits(0b11 << 3) });

        /* 8 bit transfer, 1 stop bit, even parity */
        self.usart
            .ucsr0c
            .write(|w| unsafe { w.bits(0b10 << 4 | 0b11 << 1) });
    }

    fn transmit(&mut self, byte: &u8) {
        /* Wait for data register to be empty */
        while (self.usart.ucsr0a.read().bits() & (1 << 5)) == 0 {}

        /* Write byte into data register */
        self.usart.udr0.write(|w| unsafe { w.bits(*byte) });
    }

    fn receive(&mut self) -> u8 {
        /* Wait for data to be received */
        while self.usart.ucsr0a.read().rxc0().bit_is_clear() {}

        /* Read data */
        self.usart.udr0.read().bits()
    }
}

impl uWrite for UsartController {
    /// The error associated to this writer
    type Error = UsartError;

    /// Writes a string slice into this writer, returning whether the write succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully written, and this
    /// method will not return until all data has been written or an error occurs.
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for byte in s.as_bytes() {
            self.transmit(byte);
        }
        Ok(())
    }

    /// Writes a [`char`] into this writer, returning whether the write succeeded.
    ///
    /// A single [`char`] may be encoded as more than one byte. This method can only succeed if the
    /// entire byte sequence was successfully written, and this method will not return until all
    /// data has been written or an error occurs.
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        let mut buf: [u8; 4] = unsafe { uninitialized() };
        self.write_str(c.encode_utf8(&mut buf))
    }
}

impl Read for UsartController {
    type Error = UsartError;

    fn read_char(&mut self) -> char {
        self.receive() as char
    }

    fn read_string(&mut self, s: &mut [char]) {
        let mut i: usize = 0;
        let mut c: char = '\0';
        while {
            c = self.read_char();
            c != '\0' && c != '\n' && i < s.len()
        } {
            s[i] = c;
            i += 1;
        }
    }
}

pub trait Read {
    type Error;

    fn read_char(&mut self) -> char;

    fn read_string(&mut self, s: &mut [char]);
}
