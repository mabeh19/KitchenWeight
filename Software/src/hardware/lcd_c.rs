use core::ffi as raw;



extern "C" {
    pub fn LCD_init();
	pub fn LCD_write_char(message: raw::c_uchar);
    pub fn LCD_write_str(message: *const raw::c_uchar);

	pub fn LCD_clear();
	pub fn LCD_home();

	pub fn LCD_display_off();
	pub fn LCD_display_on();
	pub fn LCD_blink_off();
	pub fn LCD_blink_on();
	pub fn LCD_cursor_off();
	pub fn LCD_cursor_on();
	pub fn LCD_scroll_display_left();
	pub fn LCD_scroll_display_right();
	pub fn LCD_left_to_right();
	pub fn LCD_right_to_Left();
	pub fn LCD_no_backlight();
	pub fn LCD_backlight();
	pub fn LCD_autoscroll();
	pub fn LCD_no_autoscroll();
	pub fn LCD_create_char(location: raw::c_uchar, charmap: *mut raw::c_uchar);
	pub fn LCD_set_cursor(col: raw::c_uchar, row: raw::c_uchar);

    pub fn LCD_command_write(value: raw::c_uchar);
	pub fn LCD_command_read() -> raw::c_uchar;
	pub fn LCD_data_write(value: raw::c_uchar);
	pub fn LCD_data_read() -> raw::c_uchar;
	pub fn LCD_busy() -> raw::c_uchar;
	pub fn LCD_address_counter() -> raw::c_uchar;
	pub fn LCD_read_DDRam(address: raw::c_uchar) -> raw::c_uchar;
	pub fn LCD_read_CGRam(address: raw::c_uchar) -> raw::c_uchar;
       
}
