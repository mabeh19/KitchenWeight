#![no_std]
#![no_main]
#![feature(generic_const_exprs)]
#![feature(core_ffi_c)]

/* Import crates */
pub mod hardware;
pub mod utils;

use core::cell::RefCell;

use panic_halt as _;

use hardware::{
    lcd::LCD, 
    pca9685::*, 
    twi_conroller::*, 
    usart_controller::*,
    hx711::*,
};
use utils::logging_tool::*;

type Callback = fn(&mut [u8]);

#[arduino_hal::entry]
fn main() -> ! {
    const BAUD_RATE: u32 = 57600;
    const LCD_SLAVE_ADDR: u8 = 0x27;

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = UsartController::new(
        dp.USART0,
        pins.d0.downgrade(),
        pins.d1.into_output().downgrade(),
    );
    serial.init(BAUD_RATE);

    let uart_ref = RefCell::new(serial);
    let mut logger = LoggingTool::new(LoggerType::Uart(uart_ref));
    let logger_ref = RefCell::new(logger);

    /* TWI Controller */
    let mut twi_controller = TwiController::new(dp.TWI);
    let devices_connected = twi_controller.ping_for_devices();
    let twi_reference: TwiReference = RefCell::new(twi_controller);

    let mut lcd = LCD::init(&twi_reference);
    lcd.clear();
    lcd.home();
     
    let mut weight_sensor = HX711::new(
        &twi_reference,
        pins.d2.downgrade(),
        pins.d3.into_output().downgrade(),
        1
    );

    loop {
           
    }
}
