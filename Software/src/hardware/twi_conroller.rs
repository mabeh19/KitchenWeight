use arduino_hal::pac::TWI;
use core::cell::RefCell;

pub type TwiReference = RefCell<TwiController>;

#[repr(u8)]
pub enum DataDirection {
    Write = 0x00,
    Read = 0x01,
}

#[allow(non_camel_case_types)]
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TwiError {
    InvalidAddress,
    NoConnection,
    InitError,
}

pub struct TwiController {
    i2c: TWI,
}

impl TwiController {
    pub fn new(i2c: TWI) -> Self {
        /* Set baudrate to 400 kHz */
        i2c.twsr.write(|w| unsafe { w.bits(0x00) }); /* Prescaler = 1 */
        i2c.twbr.write(|w| unsafe { w.bits(0x0C) }); /* Baudrate divisor = 12 */

        TwiController { i2c }
    }

    /* Only this should to be called to transfer data */
    pub fn read_reg(&mut self, slave_address: u8, start_register: u8, buffer: &mut [u8]) {
        self.write_data(slave_address, &[start_register]);

        self.start_transaction(slave_address, DataDirection::Read);

        let last_byte = buffer.len() - 1;
        for i in 0..last_byte + 1 {
            buffer[i] = if i < last_byte {
                self.read_ack()
            } else {
                self.read_nack()
            };
        }

        self.stop_transaction();
    }

    pub fn write_reg(&mut self, slave_address: u8, buffer: &[u8]) {
        self.write_data(slave_address, buffer);

        self.stop_transaction();
    }

    fn write_data(&mut self, slave_address: u8, buffer: &[u8]) {
        self.start_transaction(slave_address, DataDirection::Write);
        buffer.into_iter().for_each(|b| self.write_byte(*b));
    }

    /* Assumes a read transaction has been started */
    pub fn read_ack(&mut self) -> u8 {
        /* Send ACK */
        self.i2c.twcr.write(|w| unsafe {
            w.bits(
            0x00     |
            (1 << 7) |  /* Clear TWI interrupt flag */
            (1 << 6) |  /* Set ACK condition flag   */
            (1 << 2), /* Enable TWI operation     */
            )
        });

        self.wait();

        /* Retrieve data from register */
        self.i2c.twdr.read().bits()
    }

    pub fn read_nack(&mut self) -> u8 {
        /* Send ACK */
        self.i2c.twcr.write(|w| unsafe {
            w.bits(
                0x00     |
            (1 << 7) |  /* Clear TWI interrupt flag */
            (1 << 2), /* Enable TWI operation     */
            )
        });

        self.wait();

        let data = self.i2c.twdr.read().bits();

        self.stop_transaction();

        return data;
    }

    pub fn ping_device(&mut self, slave_address: u8) -> bool {
        let ret: bool;
        self.start_transaction(slave_address, DataDirection::Write);

        self.wait();

        match self.i2c.twsr.read().bits() & 0xF8 {
            0x18 => {
                ret = true;
            }
            _ => {
                ret = false;
            }
        }

        self.stop_transaction();
        return ret;
    }

    pub fn ping_for_devices(&mut self) -> [bool; 0x7F] {
        let mut device_list: [bool; 0x7F] = [false; 0x7F];
        for addr in 0..0x7F {
            if self.ping_device(addr) {
                device_list[addr as usize] = true;
            }
        }

        device_list
    }

    /* Internals */
    fn write_byte(&mut self, byte: u8) {
        /* Copy byte into data register */
        self.i2c.twdr.write(|w| unsafe { w.bits(byte) });

        /* Set control register to enable transfer */
        self.i2c.twcr.write(|w| unsafe {
            w.bits(
                0x00     |
            (1 << 7) |  /* Clear TWI interrupt flag */
            (1 << 2), /* Enable TWI operation */
            )
        });

        self.wait();
    }

    fn start_transaction(&mut self, slave_address: u8, direction: DataDirection) {
        let byte: u8 = (slave_address << 1) | direction as u8;

        self.send_start_condition();

        self.wait();

        self.write_byte(byte);

        self.wait();
    }

    pub fn stop_transaction(&mut self) {
        self.i2c.twcr.write(|w| unsafe {
            w.bits(
                0x00     |
            (1 << 7) |  /* Clear TWI interrupt flag */
            (1 << 4) |  /* Set STOP condition bit */
            (1 << 2), /* Enable TWI operation */
            )
        });
    }

    pub fn send_start_condition(&mut self) {
        self.i2c.twcr.write(|w| unsafe {
            w.bits(
                0x00     |
            (1 << 7) |  /* Clear TWI interrupt flag */
            (1 << 5) |  /* Set START condition bit */
            (1 << 2), /* Enable TWI operation */
            )
        });

        /* Wait for START condition to be transmitted */
        self.wait();
    }

    fn wait(&self) {
        while (self.i2c.twcr.read().bits() & (1 << 7)) == 0 {}
    }
}
