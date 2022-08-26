use crate::utils::crc::{Crc8Generator, Crc8GeneratorReference};

use crate::hardware::spi_controller::*;
use crate::utils::logging_tool::*;

pub struct SDCard<'a> {
    spi_controller: &'a SpiReference,
    crc_generator: &'a Crc8GeneratorReference<'a>,
    logger: &'a LoggingToolReference,
}

impl<'a> SDCard<'a> {
    pub fn init(&mut self) -> u8 {
        /* Get reference to SPI and CRC controllers */
        let mut spi = self.spi_controller.borrow_mut();
        let crc = self.crc_generator.borrow_mut();

        /* Response buffer */
        let mut response: [u8; 5] = [0; 5];

        spi.master_init();

        /* Power up SD card */
        logln!(self.logger, "Powering up SD card...");
        self.power_up_sequence(&mut spi);

        logln!(self.logger, "Setting SD card in idle mode");
        let res = self.go_idle(&mut spi, &crc);
        if res != 0x01 {
            return res;
        }

        logln!(self.logger, "Sending IF condition...");
        self.send_if_cond(&mut spi, &crc, &mut response);

        drop(spi);
        drop(crc);
        return response[0];
    }

    pub fn new(
        spi_controller: &'a SpiReference,
        crc_generator: &'a Crc8GeneratorReference<'a>,
        logger: &'a LoggingToolReference,
    ) -> Self {
        SDCard {
            spi_controller: spi_controller,
            crc_generator: crc_generator,
            logger: logger,
        }
    }

    fn send_command(
        &mut self,
        cmd: u8,
        args: &u32,
        spi: &mut SpiController,
        crc: &Crc8Generator<'a>,
    ) -> u8 {
        /* Copy command and arguments to buffer */
        let package: [u8; 5] = [
            cmd | 0x40,
            ((*args >> 24) & 0xFF) as u8,
            ((*args >> 16) & 0xFF) as u8,
            ((*args >> 08) & 0xFF) as u8,
            ((*args >> 00) & 0xFF) as u8,
        ];
        /* Calculate CRC7 */
        let crc7: u8 = crc.calculate_crc(&package) << 1;
        let mut i = 10;

        //logln!( self.logger, "Data: [{},{},{},{},{}]", package[0], package[1], package[2], package[3],package[4]);
        //logln!( self.logger, "CRC8: {}", crc7);

        logln!(self.logger, "Deselecting chip");
        /* Deselect chip */
        spi.deselect_chip();
        spi.transmit(&[0xFF]);

        logln!(self.logger, "Selecting chip");
        /* Assert chip select */
        spi.select_chip();
        spi.transmit(&[0xFF]);

        logln!(self.logger, "Waiting for SD card to be ready");
        self.wait_ready(spi, 500);

        logln!(self.logger, "Sending command and arguments");
        /* Transfer command and arguments */
        spi.transmit(&package);

        logln!(self.logger, "Sending crc ");
        /* Transfer CRC */
        spi.transmit(&[crc7 | 0x01]);

        /* Return with response value */
        self.read_res1(spi)
    }

    fn power_up_sequence(&mut self, spi: &mut SpiController) {
        /* Make sure card is deselected */
        spi.deselect_chip();

        /* Give SD card time to power up */
        arduino_hal::delay_ms(1);

        let buff: [u8; 1] = [0xFF];
        /* Send 80 clock cycles to synchronize */
        for _ in 0..10 {
            spi.transmit(&buff);
        }

        spi.deselect_chip();
        spi.transmit(&buff);
    }

    fn go_idle(&mut self, spi: &mut SpiController, crc: &Crc8Generator<'a>) -> u8 {
        let args: u32 = 0x00;
        let mut response: u8;

        logln!(self.logger, "Sending CMD0");
        response = self.send_command(SD_Command::CMD0 as u8, &args, spi, crc);

        spi.transmit(&[0xFF]);
        spi.deselect_chip();
        spi.transmit(&[0xFF]);

        return response;
    }

    fn send_if_cond(
        &mut self,
        spi: &mut SpiController,
        crc: &Crc8Generator<'a>,
        res: &mut [u8; 5],
    ) {
        let args: u32 = 0x000001AA;
        /* Assert chip select */
        spi.transmit(&[0xFF]);
        spi.select_chip();
        spi.transmit(&[0xFF]);

        /* Send CMD8 */
        self.send_command(SD_Command::CMD8 as u8, &args, spi, crc);

        /* Read response */
        self.read_res7(spi, res);

        /* Deassert chip select */
        spi.transmit(&[0xFF]);
        spi.deselect_chip();
        spi.transmit(&[0xFF]);
    }

    fn read_res1(&mut self, spi: &mut SpiController) -> u8 {
        let mut res1;
        let mut i: u8 = 0;
        while {
            res1 = spi.transmit(&[0xFF]);
            (res1 & 0x80) == 0x80
        } {
            i += 1;
            if i == 10 {
                break;
            }
        }

        return res1;
    }

    fn read_res7(&mut self, spi: &mut SpiController, res: &mut [u8; 5]) {
        /* Read response 1 */
        res[0] = self.read_res1(spi);

        /* If an error occurred, return early */
        if res[0] > 1 {
            return;
        }

        for i in 0..5 {
            res[i] = spi.transmit(&[0xFF]);
        }
    }

    fn wait_ready(
        /* 1:Ready, 0:Timeout */
        &mut self,
        spi: &mut SpiController,
        mut wt: u16, /* Timeout [ms] */
    ) -> u8 {
        let mut d: u8 = 0x00;

        wt /= 10;

        while {
            d = spi.transmit(&[0xFF]);
            d != 0xFF
        } {}

        return if d == 0xFF { 1 } else { 0 };
    }
}

#[allow(non_camel_case_types)]
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum SD_Command {
    CMD0 = 0x00,
    CMD8 = 0x08,
}
