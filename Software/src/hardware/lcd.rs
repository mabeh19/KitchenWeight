#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::{
    TwiReference,
    
};
use arduino_hal;

const LCD_CLEAR_DISPLAY: u8         = 0x01;
const LCD_RETURN_HOME: u8           = 0x02;
const LCD_ENTRY_MODE_SET: u8        = 0x04;
const LCD_INCREMENT: u8             = 0x02;
const LCD_DECREMENT: u8             = 0x00;     // Sub Mode of ENTRY_MODE_SET : Decrement DDRAM (I/D), Entry Right
const LCD_SHIFT_ON: u8              = 0x01;     // Sub Mode of ENTRY_MODE_SET : Shift On  (S), Shift Display when byte written. Display Shift
const LCD_SHIFT_OFF: u8             = 0x00;     // Sub Mode of ENTRY_MODE_SET : Shift Off (S), Don't shift display when byte written. Cursor Move

// Display Function
const LCD_DISPLAY_ON_OFF: u8        = 0x08;     // Mode : Display On/Off, Sets on/off of all display, Cursor on/off, Cursor Blink on/off
const LCD_DISPLAY_ON: u8            = 0x04;     // Sub Mode of DISPLAY_ON_OFF : Puts display on  (D)
const LCD_DISPLAY_OFF: u8           = 0x00;     // Sub Mode of DISPLAY_ON_OFF : Puts display off (D)
const LCD_CURSOR_ON: u8             = 0x02;     // Sub Mode of DISPLAY_ON_OFF : Puts cursor on   (C)
const LCD_CURSOR_OFF: u8            = 0x00;     // Sub Mode of DISPLAY_ON_OFF : Puts cursor off  (C)
const LCD_BLINKING_ON: u8           = 0x01;     // Sub Mode of DISPLAY_ON_OFF : Blinking cursor  (B)
const LCD_BLINKING_OFF: u8          = 0x00;     // Sub Mode of DISPLAY_ON_OFF : Solid cursor     (B)

// Display Control
const LCD_MV_CUR_SHIFT_DISPLAY: u8  = 0x10;     // Mode : Move the cursor and shifts the display
const LCD_DISPLAY_SHIFT: u8         = 0x08;     // Sub Mode of CURSOR_SHFT_DIS : Display shifts after char print   (SC)
const LCD_CURSOR_SHIFT: u8          = 0x00;     // Sub Mode of CURSOR_SHFT_DIS : Cursor shifts after char print    (SC)
const LCD_SHIFT_RIGHT: u8           = 0x04;     // Sub Mode of CURSOR_SHFT_DIS : Cursor or Display shifts to right (RL)
const LCD_SHIFT_LEFT: u8            = 0x00;     // Sub Mode of CURSOR_SHFT_DIS : Cursor or Display shifts to left  (RL)

// Function Set
const LCD_FUNCTION_SET: u8          = 0x20;     // Mode : Set the type of interface that the display will use
const LCD_INTF8BITS: u8             = 0x10;     // Sub Mode of FUNCTION_SET : Select 8 bit interface         (DL)
const LCD_INTF4BITS: u8             = 0x00;     // Sub Mode of FUNCTION_SET : Select 4 bit interface         (DL)
const LCD_TWO_LINES: u8             = 0x08;     // Sub Mode of FUNCTION_SET : Selects two char line display  (N)
const LCD_ONE_LINE: u8              = 0x00;     // Sub Mode of FUNCTION_SET : Selects one char line display  (N)
const LCD_FONT_5_10: u8             = 0x04;     // Sub Mode of FUNCTION_SET : Selects 5 x 10 Dot Matrix Font (F)
const LCD_FONT_5_7: u8              = 0x00;     // Sub Mode of FUNCTION_SET : Selects 5 x 7 Dot Matrix Font  (F)

const LCD_CG_RAM_ADDRESS: u8        = 0x40;        // Mode : Enables the setting of the Char Gen (CG) Ram Address, to be or'ed with require address
const LCD_CG_RAM_ADDRESS_MASK: u8   = 0b00111111;  // Used to mask off the lower 6 bits of valid CG Ram Addresses

const LCD_DD_RAM_ADDRESS: u8        = 0x80;        // Mode : Enables the setting of the Display Data (DD) Ram Address, to be or'ed with require address
const LCD_DD_RAM_ADDRESS_MASK: u8   = 0b01111111;    // Used to mask off the lower 6 bits of valid DD Ram Addresses

//#define USE_BUSY_FLAG               // Define this if you wish to use busy flag polling on slow LCD activities

// Change here for your I2C to 16 pin parallel interface // TODO Adapt
const Bl: u8 = 0b00001000;  // Backlight enable bit (On = 1, Off =0)
const En: u8 = 0b00000100;  // Enable bit (Enable on low edge)
const Rw: u8 = 0b00000010;  // Read/Write bit (Read = 1, Write = 0)
const Rs: u8 = 0b00000001;  // Register select bit (Data = 1, Control = 0)

// Change here for your I2C to 16 pin parallel interface // TODO Adapt
//#define LCD_INIT      ((0b00000000 | En) & ~Rs) & (~Rw) // Used to set all the O/Ps on the PCF8574 to initialise the LCD
const LCD_8BIT_INIT: u8 = 0b00110000; // Used to initialise the interface at the LCD
const LCD_4BIT_INIT: u8 = 0b00100000; // Used to initialise the interface at the LCD

const LCD_PCF8574_ADDR: u8          = 0x27;  // Modify this if the default address is altered
const LCD_PCF8574_WEAK_PU: u8       = 0b11110000; // Used to turn on PCF8574 Bits 7-4 on. To allow for read of LCD.

const LCD_BUSY_FLAG_MASK: u8        = 0b10000000; // Used to mask off the status of the busy flag
const LCD_ADDRESS_COUNTER_MASK: u8  = 0b01111111; // Used to mask off the value of the Address Counter
const LCD_MAX_COLS: u8              = 20;
const LCD_MAX_ROWS: u8              = 4;

const LCD_LINE1: u8                 = 0x00;
const LCD_LINE2: u8                 = 0x40;
const LCD_LINE3: u8                 = 0x14;
const LCD_LINE4: u8                 = 0x54;

#[repr(C)]
pub struct LCD<'a> {
    address: u8,
    i2c: &'a TwiReference,
    function_set: u8,
    entrymode_set: u8,
    display_function: u8,
    display_control: u8,
    num_lines: u8,
    backlight_val: u8
}

impl<'a> LCD<'a> {
    pub fn init(i2c: &'a TwiReference) -> Self {
        let mut s = Self {
            address: LCD_PCF8574_ADDR,
            i2c,
            function_set: 0x00,
            entrymode_set: 0x00,
            display_function: 0x00,
            display_control: 0x00,
            num_lines: LCD_MAX_ROWS,
            backlight_val: Bl
        };

        arduino_hal::delay_ms(50);

        let mut twi = s.i2c.borrow_mut();
        let lcd_init: u8 = ((0b00000000 | En) & !Rs) & (!Rw);
        twi.write_reg(s.address, &[lcd_init]);
        
        arduino_hal::delay_us(100);

        s.write_4_bits(LCD_8BIT_INIT);
        arduino_hal::delay_us(4500);

        s.write_4_bits(LCD_8BIT_INIT);
        arduino_hal::delay_us(150);

        s.write_4_bits(LCD_8BIT_INIT);
        arduino_hal::delay_us(150);

        s.write_4_bits(LCD_4BIT_INIT);
        arduino_hal::delay_us(150);

        s.function_set = LCD_INTF4BITS | LCD_TWO_LINES | LCD_FONT_5_7;
        s.command_write(LCD_FUNCTION_SET | s.function_set);

        s.display_function = LCD_DISPLAY_OFF | LCD_CURSOR_OFF | LCD_BLINKING_OFF;
        s.display_off();

        s.display_on();

        s.entrymode_set = LCD_INCREMENT | LCD_SHIFT_OFF;
        s.command_write(LCD_ENTRY_MODE_SET | s.entrymode_set);

        s.command_write(LCD_DISPLAY_ON_OFF | s.display_function);

        s.display_control = LCD_DISPLAY_SHIFT | LCD_SHIFT_LEFT;
        s.command_write(LCD_MV_CUR_SHIFT_DISPLAY | s.display_control);

        s.clear();

        return s;
    }

    /* High level commands */
    pub fn write_char(&mut self, message: u8) {
        self.data_write(message);
    }

    pub fn write_str(&mut self, message: &str) {
        message.as_bytes().into_iter().for_each(|b| self.data_write(*b));
    }

    pub fn clear(&mut self) {
        self.command_write(LCD_CLEAR_DISPLAY);
        arduino_hal::delay_ms(30);
    }

    pub fn home(&mut self) {
        self.command_write(LCD_RETURN_HOME);
        arduino_hal::delay_ms(30);
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) {
        let row_offsets: [u8; 4] = [LCD_LINE1, LCD_LINE2, LCD_LINE3, LCD_LINE4];
        if row >= self.num_lines {
            let row = self.num_lines - 1;
        }

        self.command_write(LCD_DD_RAM_ADDRESS | (col + row_offsets[row as usize]));
    }

    pub fn display_off(&mut self) {
        self.display_function &= !LCD_DISPLAY_ON;
        self.command_write(LCD_DISPLAY_ON_OFF | self.display_function);
    }

    pub fn display_on(&mut self) {
        self.display_function |= LCD_DISPLAY_ON;
        self.command_write(LCD_DISPLAY_ON_OFF | self.display_function);
    }

    pub fn cursor_off(&mut self) {
        self.display_function &= !LCD_CURSOR_ON;
        self.command_write(LCD_DISPLAY_ON_OFF | self.display_function);
    }

    pub fn cursor_on(&mut self) {
        self.display_function |= LCD_CURSOR_ON;
        self.command_write(LCD_DISPLAY_ON_OFF | self.display_function);
    }

    // Turn on and off the blinking cursor
 	pub fn blink_off(&mut self) {
	    self.display_function &= !LCD_BLINKING_ON;
	    self.command_write(LCD_DISPLAY_ON_OFF | self.display_function);
    }

 	pub fn blink_on(&mut self) {
        self.display_function |= LCD_BLINKING_ON;
        self.command_write(LCD_DISPLAY_ON_OFF | self.display_function);
    }

    // These commands scroll the display without changing the RAM
 	pub fn scroll_display_left(&mut self) {
        self.display_control &=  !LCD_SHIFT_RIGHT;
        self.display_control |=   LCD_DISPLAY_SHIFT;
        self.command_write(LCD_MV_CUR_SHIFT_DISPLAY | self.display_control);
    }

 	pub fn scroll_display_right(&mut self) {
        self.display_control |=  LCD_SHIFT_RIGHT;
        self.display_control |=  LCD_DISPLAY_SHIFT;
        self.command_write(LCD_MV_CUR_SHIFT_DISPLAY | self.display_control);
    }


    // This is for text that flows Left to Right
 	pub fn left_to_right(&mut self) {
        self.entrymode_set |= LCD_INCREMENT;
        self.command_write(LCD_ENTRY_MODE_SET | self.entrymode_set);
    }

    // This is for text that flows Right to Left
 	pub fn right_to_left(&mut self) {
        self.entrymode_set &= !LCD_INCREMENT;
        //self.entrymode_set &= ~LCD_SHIFT_ON;
        self.command_write(LCD_ENTRY_MODE_SET | self.entrymode_set);
    }

    // This will 'right justify' text from the cursor. Display shift
 	pub fn autoscroll(&mut self){
        self.entrymode_set |= LCD_SHIFT_ON;
        //self.entrymode_set |= LCD_INCREMENT;
        self.command_write(LCD_ENTRY_MODE_SET | self.entrymode_set);
    }

    // This will 'left justify' text from the cursor. Cursor Move
    pub fn no_autoscroll(&mut self){
        self.entrymode_set &= !LCD_SHIFT_ON;
        //self.entrymode_set &= ~LCD_INCREMENT;
        self.command_write(LCD_ENTRY_MODE_SET | self.entrymode_set);
    }

    // Allows us to fill the first 8 CGRAM locations
    // with custom characters
    pub fn createChar(&mut self, location: u8, charmap: &[u8]) {
        let location = location & 0x7; // we only have 8 locations 0-7
        self.command_write(LCD_CG_RAM_ADDRESS | (location << 3));
        charmap.into_iter().for_each(|b| self.data_write(*b));
    }

    // Turn the (optional) backlight off/on
 	pub fn no_backlight(&mut self){
        self.backlight_val &= !Bl;
        let dummy_data = self.read_pcf8574();
	    self.write_pcf8574(dummy_data);  // Dummy write to LCD, only led control bit is of interest
    }

 	pub fn backlight(&mut self){
        self.backlight_val |= Bl;
        let dummy_data = self.read_pcf8574();
        self.write_pcf8574(dummy_data);  // Dummy write to LCD, only led control bit is of interest
    }



/*********** mid level commands, for sending data/cmds */

    #[inline(always)]
    pub fn command_write(&mut self, value: u8) {
	    self.send(value, Rs & !Rs);
    }
    
    #[inline(always)]
    pub fn command_read(&mut self) -> u8 {
        return self.receive(Rs & !Rs);
    }

    #[inline(always)]
    pub fn data_write(&mut self, value: u8) {
        self.send(value, Rs);
    }

    #[inline(always)]
    pub fn data_read(&mut self) -> u8 {
	    return self.receive(Rs);
    }

    pub fn busy(&mut self) -> u8 {
        return self.command_read() & LCD_BUSY_FLAG_MASK;
    }

    pub fn address_counter(&mut self) -> u8 {
	    return self.command_read() & LCD_ADDRESS_COUNTER_MASK;
    }


    pub fn read_DDRam(&mut self, address: u8) -> u8 {
        self.command_write(LCD_DD_RAM_ADDRESS | (address & LCD_DD_RAM_ADDRESS_MASK));
        return self.data_read();
    }

    pub fn read_CGRam(&mut self, address: u8) -> u8 {
        self.command_write(LCD_CG_RAM_ADDRESS | (address & LCD_CG_RAM_ADDRESS_MASK));
        return self.data_read();
    }

/************ low level data write commands **********/

// Change this routine for your I2C to 16 pin parallel interface, if your pin interconnects are different to that outlined above // TODO Adapt

// write either command or data
	pub fn send(&mut self, value: u8, RsMode: u8) {
	    let highnib: u8 = value & 0xF0;

        let mut lownib: u8  = value << 4;
        lownib &= 0xF0;

        self.write_4_bits((highnib) | En | RsMode);
        self.write_4_bits((lownib ) | En | RsMode);
    }

// Change this routine for your I2C to 16 pin parallel interface, if your pin interconnects are different to that outlined above // TODO Adapt

// read either command or data
    fn receive(&mut self, RsMode: u8) -> u8 {
        let highnib: u8;
        let lownib: u8;

        self.write_pcf8574(LCD_PCF8574_WEAK_PU | (En & !En) | RsMode); // Set P7..P4 = 1, En = 0, RnW = 0, Rs = XX
        highnib = self.read_4_bits(LCD_PCF8574_WEAK_PU | En | RsMode);
        lownib = self.read_4_bits(LCD_PCF8574_WEAK_PU | En | RsMode);
        self.write_pcf8574((LCD_PCF8574_WEAK_PU & !LCD_PCF8574_WEAK_PU) | En | RsMode); // Set P7..P4 = 1, En = 1, RnW = 0, Rs = XX
        return ((highnib & 0xF0) | ((lownib & 0xF0) >> 4));
    }

	fn write_4_bits(&mut self, nibEnRsMode: u8) {
        self.write_pcf8574(nibEnRsMode & !Rw);
        self.pulse_enable_neg(nibEnRsMode & !Rw);
    }


    fn read_4_bits(&mut self, rs_en_mode: u8) -> u8 {
        let b: u8;
        self.pulse_enable_pos(rs_en_mode | Rw);
        b = self.read_pcf8574(); // Read the data from the LCD just after the rising edge. NOT WELL DOCUMENTED!
        self.pulse_enable_neg(rs_en_mode | Rw);
        return b;
    }

	fn pulse_enable_neg(&mut self, data: u8) {
	    self.write_pcf8574(data | En);	// En high
        arduino_hal::delay_us(1);		// enable pulse must be >450ns

        self.write_pcf8574(data & !En);	// En low
        arduino_hal::delay_us(50);		// commands need > 37us to settle
    }

	fn pulse_enable_pos(&mut self, data: u8) {
        self.write_pcf8574(data & !En);	// En low
        arduino_hal::delay_us(1);		// enable pulse must be >450ns
    
        self.write_pcf8574(data | En);	// En high
        arduino_hal::delay_us(50);		// commands need > 37us to settle
    }


	fn write_pcf8574(&mut self, value: u8) {
        if let Ok(mut twi) = self.i2c.try_borrow_mut() {
            twi.write_reg(self.address, &[value | self.backlight_val]);
        }
    }

    fn read_pcf8574(&mut self) -> u8 {
        if let Ok(mut twi) = self.i2c.try_borrow_mut() {
            let mut result = [0x00];
            twi.read_reg(self.address, 0x00, &mut result);
            return result[0];
        } else {
            return 0xFF;
        }
    }
}
