use super::twi_conroller::*;

#[repr(C)]
pub struct PCA9685<'a> {
    address: u8,
    i2c: &'a TwiReference,
}

impl<'a> PCA9685<'a> {
    pub fn new(address: u8, i2c: &'a TwiReference) -> Self {
        PCA9685 {
            address: address,
            i2c: i2c,
        }
    }

    /* Initializes to default settings */
    pub fn init(&mut self) -> Result<(), TwiError> {
        let mut i2c = self.i2c.borrow_mut();
        let cfg_packet = [PCA9685_Register::MODE1 as u8, (1 << 5) | (1 << 4)];
        let freq_packet = [
            PCA9685_Register::PRE_SCALE as u8,
            0x03, /* Set to max PWM frequency */
        ];

        if !i2c.ping_device(self.address) {
            drop(i2c);
            return Err(TwiError::NoConnection);
        }

        i2c.write_reg(self.address, &cfg_packet);
        i2c.write_reg(self.address, &freq_packet);
        drop(i2c);
        Ok(())
    }

    pub fn set_motor_speed(&mut self, led_num: u8, high_time: u16, low_time: u16) {
        let mut i2c = self.i2c.borrow_mut();
        let packet: [u8; 5] = [
            PCA9685_Register::LED0_ON_L as u8 + 4 * led_num,
            (high_time & 0xFF) as u8,
            (high_time >> 8) as u8,
            (low_time & 0xFF) as u8,
            (low_time >> 8) as u8,
        ];
        i2c.write_reg(self.address, &packet);
        drop(i2c);
    }
}

#[allow(non_camel_case_types)]
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum PCA9685_Register {
    MODE1 = 0x00,
    MODE2 = 0x01,
    SUBADR1 = 0x02,
    SUBADR2 = 0x03,
    SUBADR3 = 0x04,
    ALLCALLADR = 0x05,
    LED0_ON_L = 0x06,
    LED0_ON_H = 0x07,
    LED0_OFF_L = 0x08,
    LED0_OFF_H = 0x09,
    LED1_ON_L = 0x0A,
    LED1_ON_H = 0x0B,
    LED1_OFF_L = 0x0C,
    LED1_OFF_H = 0x0D,
    LED2_ON_L = 0x0E,
    LED2_ON_H = 0x0F,
    LED2_OFF_L = 0x10,
    LED2_OFF_H = 0x11,
    LED3_ON_L = 0x12,
    LED3_ON_H = 0x13,
    LED3_OFF_L = 0x14,
    LED3_OFF_H = 0x15,
    LED4_ON_L = 0x16,
    LED4_ON_H = 0x17,
    LED4_OFF_L = 0x18,
    LED4_OFF_H = 0x19,
    LED5_ON_L = 0x1A,
    LED5_ON_H = 0x1B,
    LED5_OFF_L = 0x1C,
    LED5_OFF_H = 0x1D,
    LED6_ON_L = 0x1E,
    LED6_ON_H = 0x1F,
    LED6_OFF_L = 0x20,
    LED6_OFF_H = 0x21,
    LED7_ON_L = 0x22,
    LED7_ON_H = 0x23,
    LED7_OFF_L = 0x24,
    LED7_OFF_H = 0x25,
    LED8_ON_L = 0x26,
    LED8_ON_H = 0x27,
    LED8_OFF_L = 0x28,
    LED8_OFF_H = 0x29,
    LED9_ON_L = 0x2A,
    LED9_ON_H = 0x2B,
    LED9_OFF_L = 0x2C,
    LED9_OFF_H = 0x2D,
    LED10_ON_L = 0x2E,
    LED10_ON_H = 0x2F,
    LED10_OFF_L = 0x30,
    LED10_OFF_H = 0x31,
    LED11_ON_L = 0x32,
    LED11_ON_H = 0x33,
    LED11_OFF_L = 0x34,
    LED11_OFF_H = 0x35,
    LED12_ON_L = 0x36,
    LED12_ON_H = 0x37,
    LED12_OFF_L = 0x38,
    LED12_OFF_H = 0x39,
    LED13_ON_L = 0x3A,
    LED13_ON_H = 0x3B,
    LED13_OFF_L = 0x3C,
    LED13_OFF_H = 0x3D,
    LED14_ON_L = 0x3E,
    LED14_ON_H = 0x3F,
    LED14_OFF_L = 0x40,
    LED14_OFF_H = 0x41,
    LED15_ON_L = 0x42,
    LED15_ON_H = 0x43,
    LED15_OFF_L = 0x44,
    LED15_OFF_H = 0x45,
    /*
        Reserved
    */
    ALL_LED_ON_L = 0xFA,
    ALL_LED_ON_H = 0xFB,
    ALL_LED_OFF_L = 0xFC,
    ALL_LED_OFF_H = 0xFD,
    PRE_SCALE = 0xFE,
    TESTMODE = 0xFF,
}
