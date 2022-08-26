use core::cell::RefCell;

use crate::hardware::usart_controller::UsartReference;

pub type LoggingToolReference = RefCell<LoggingTool>;

pub enum LoggerType {
    Uart(UsartReference),
}

pub struct LoggingTool {
    logger: LoggerType,
}

impl LoggingTool {
    pub fn new(logger: LoggerType) -> Self {
        LoggingTool { logger: logger }
    }

    pub fn get_formatter(&mut self) -> &mut LoggerType /*Usart<arduino_hal::pac::USART0, Pin<Input, PD0>, Pin<Output, PD1>> */
    {
        &mut self.logger
    }

    pub fn write(&mut self, string: &str) {
        //ufmt::uwriteln!(&mut self.uart, string);
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! logln {
    ( $logging_tool_ref:expr, $( $arg:expr ),* ) => {


        let mut logging_tool = $logging_tool_ref.borrow_mut();
        match &mut *logging_tool.get_formatter() {
            crate::LoggerType::Uart(serial) => {
                let mut s = serial.borrow_mut();
                ufmt::uwriteln!(
                    &mut s,
                    $( $arg, )*
                );
                drop(s);
            }
        }
        drop(logging_tool);
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ( $logging_tool_ref:expr, $( $arg:expr ),* ) => {


        let mut logging_tool = $logging_tool_ref.borrow_mut();
        match &mut *logging_tool.get_formatter() {
            crate::LoggerType::Uart(serial) => {
                let mut s = serial.borrow_mut();
                ufmt::uwrite!(
                    &mut s,
                    $( $arg, )*
                );
                drop(s);
            }
        }
        drop(logging_tool);
    };
}

#[macro_export]
macro_rules! input_string {
    ( $logging_tool_ref:expr, $buffer:expr ) => {
        let mut logging_tool = $logging_tool_ref.borrow_mut();
        match &mut *logging_tool.get_formatter() {
            crate::LoggerType::Uart(serial) => {
                let mut s = serial.borrow_mut();
                s.read_string(&mut $buffer);
                drop(s);
            }
        }
        drop(logging_tool);
    };
}

#[macro_export]
macro_rules! input_char {
    ( $logging_tool_ref:expr, $buffer:expr ) => {
        let mut logging_tool = $logging_tool_ref.borrow_mut();
        match &mut *logging_tool.get_formatter() {
            crate::LoggerType::Uart(serial) => {
                let mut s = serial.borrow_mut();
                $buffer = s.read_char();
                drop(s);
            }
        }
        drop(logging_tool);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ( $( $arg:expr ),* ) => {};
}
pub(crate) use {input_char, input_string, log, logln};
