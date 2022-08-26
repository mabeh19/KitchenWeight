use arduino_hal::port::{Pin, mode};
use crate::TwiReference;

pub struct HX711<'a> {
    i2c: &'a TwiReference,
    pd_sck: Pin<mode::Output>,
    dout: Pin<mode::Input<mode::PullUp>>,
    gain: u8,
    offset: u16,
    scale: f32
}

impl<'a> HX711<'a> {
    pub fn new(i2c: &'a TwiReference, dout: Pin<mode::Input<mode::PullUp>>, pd_sck: Pin<mode::Output>, gain: u8) -> Self {
        Self {
            i2c,
            pd_sck,
            dout,
            gain,
            offset: 0,
            scale: 1.,
        }
   }

    pub fn is_ready(&self) -> bool {
        return self.dout.is_low();
    }

    pub fn set_gain(&mut self, gain: u8) {
        match gain {
            128 => self.gain = 1,
            64 => self.gain = 3,
            32 => self.gain = 2,
            _ => {}
        }
    }

    pub fn read(&mut self) -> u32 {
        self.wait_ready(1);

        // Define structures for reading data into.
        let mut value: u64 = 0;
        let mut data: [u8; 3] = [0; 3];
        let mut filler: u8 = 0x00;
 
        // Pulse the clock pin 24 times to read the data.
        data[2] = self.shift_in(BitOrder::MSB);
        data[1] = self.shift_in(BitOrder::MSB);
        data[0] = self.shift_in(BitOrder::MSB);

        
        // Set the channel and the gain factor for the next reading using the clock pin.
        for i in 0..self.gain {
            self.pd_sck.set_high();
            self.pd_sck.set_low();
        }
 
        // Replicate the most significant bit to pad out a 32-bit signed integer
        if (data[2] & 0x80) {
            filler = 0xFF;
        } else {
            filler = 0x00;
        }

        // Construct a 32-bit signed integer
        value = ( (filler  as u64) << 24
                | (data[2] as u64) << 16
                | (data[1] as u64) << 8
                | (data[0] as u64) );

        return value;
    }

    pub fn wait_ready(&self, delay_ms: u16) {
        // Wait for the chip to become ready.
        // This is a blocking implementation and will
        // halt the sketch until a load cell is connected.
        while (!self.is_ready()) {
            // Probably will do no harm on AVR but will feed the Watchdog Timer (WDT) on ESP.
            // https://github.com/bogde/HX711/issues/73
            arduino_hal::delay_ms(delay_ms);
        }
    }

    pub fn wait_ready_retry(&self, retries: u16, delay_ms: u32) -> bool {
        // Wait for the chip to become ready by
        // retrying for a specified amount of attempts.
        // https://github.com/bogde/HX711/issues/76
        let mut count = 0;
        while count < retries {
            if (self.is_ready()) {
                return true;
            }
            arduino_hal::delay_ms(delay_ms);
            count += 1;
        }
        return false;
    }

    /*
    pub fn wait_ready_timeout(timeout: u16, delay_ms: u32) -> bool {
        // Wait for the chip to become ready until timeout.
        // https://github.com/bogde/HX711/pull/96
        let millisStarted: u16 = arduino_hal::;
        while (millis() - millisStarted < timeout) {
            if (is_ready()) {
                return true;
            }
            delay(delay_ms);
        }
        return false;
    }
    */

    pub fn read_average(&mut self, times: u8) -> u16 {
        let mut sum: u16 = 0;
        for i in 0..times {
            sum += self.read();
            // Probably will do no harm on AVR but will feed the Watchdog Timer (WDT) on ESP.
            // https://github.com/bogde/HX711/issues/73
            arduino_hal::delay_ms(0);
        }
        return sum / times as u16;
    }

    pub fn get_value(&mut self, times: u8) -> f32 {
        return (self.read_average(times) - self.offset).into();
    }

    pub fn get_units(&mut self, times: u8) -> f32 {
        return self.get_value(times) / self.scale;
    }

    pub fn tare(&mut self, times: u8) {
        let sum: u16 = self.read_average(times);
        self.offset = sum;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn get_scale(&self) -> f32 {
        return self.scale;
    }

    pub fn set_offset(&mut self, offset: u16) {
        self.offset = offset;
    }

    pub fn get_offset(&self) -> u16 {
        return self.offset;
    }

    pub fn power_down(&mut self) {
        self.pd_sck.set_low();
        self.pd_sck.set_high();
    }

    pub fn power_up(&mut self) {
        self.pd_sck.set_low();
    }

    fn shift_in(&mut self, bit_order: BitOrder) -> u8 {
        let mut value: u8 = 0;

        for i in 0..8 {
            self.pd_sck.set_high();
            match bit_order {
                LSB => value |= self.dout.is_high().into() << i,
                _ => value |=self.dout.is_high().into() << (7 - i)
            }
            self.pd_sck.set_low();
        }
        return value;
    }

}


enum BitOrder {
    LSB,
    MSB
}
