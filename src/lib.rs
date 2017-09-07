extern crate i2cdev;

use std::thread;
use std::time::Duration;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};


// commands
#[derive(Copy, Clone)]
pub enum Command {
    ClearDisplay = 0x01,
    ReturnHome = 0x02,
    EntryModeSet = 0x04,
    DisplayControl = 0x08,
    CursorShift = 0x10,
    FunctionSet = 0x20,
    SetCGRamAddr = 0x40,
    SetDDRamAddr = 0x80,
}

//# flags for display entry mode
#[derive(Copy, Clone)]
pub enum EntryMode {
    ENTRYRIGHT = 0x00,
    ENTRYLEFT = 0x02,
}

#[derive(Copy, Clone)]
pub enum EntryShift {
    ENTRYSHIFTINCREMENT = 0x01,
    ENTRYSHIFTDECREMENT = 0x00,
}

//# flags for display on/off control
//pub enum
const LCD_DISPLAY_ON: u8 = 0x04;
//LCD_DISPLAYOFF = 0x00
//LCD_CURSORON = 0x02
//LCD_CURSOROFF = 0x00
//LCD_BLINKON = 0x01
//LCD_BLINKOFF = 0x00
//
//# flags for display/cursor shift
//LCD_DISPLAYMOVE = 0x08
//LCD_CURSORMOVE = 0x00
//LCD_MOVERIGHT = 0x04
//LCD_MOVELEFT = 0x00
//
//# flags for backlight control
const LCD_BACKLIGHT_ON: u8 = 0x08;
const LCD_BACKLIGHT_OFF: u8 = 0x00;
//LCD_NOBACKLIGHT = 0x00
//
//En = 0b00000100 # Enable bit
//Rw = 0b00000010 # Read/Write bit
//Rs = 0b00000001 # Register select bit


// Configuration

#[derive(Copy, Clone)]
pub enum BitMode {
    B4 = 0x00,
    B8 = 0x10,
}

#[derive(Copy, Clone)]
pub enum LineCount {
    L1 = 0x00,
    L2 = 0x08,
}

#[derive(Copy, Clone)]
pub enum MatrixSize {
    M5x8 = 0x00,
    M5x10 = 0x04,
}

pub struct ScreenConfig {
    bit_mode: BitMode,
    line_count: LineCount,
    matrix_size: MatrixSize,
}

impl ScreenConfig {
    pub fn new(bit_mode: BitMode, line_count: LineCount, matrix_size: MatrixSize) -> ScreenConfig {
        ScreenConfig {
            bit_mode,
            line_count,
            matrix_size,
        }
    }

    pub fn default() -> ScreenConfig {
        ScreenConfig::new(BitMode::B4, LineCount::L2, MatrixSize::M5x8)
    }
}

// Screen

pub struct Screen {
    dev: LinuxI2CDevice,
    config: ScreenConfig,
}

type ScreenResult = Result<(), LinuxI2CError>;

impl Screen {
    pub fn new(config: ScreenConfig, bus: &str, i2c_addr: u16) -> Result<Screen, LinuxI2CError> {
        let dev = try!(LinuxI2CDevice::new(bus, i2c_addr));
        Ok(Screen { dev, config })
    }

    pub fn init(&mut self) -> ScreenResult {
        try!(self.write(0x03));
        try!(self.write(0x03));
        try!(self.write(0x03));
        try!(self.write(0x02));

        try!(self.install_function_set());

        try!(self.command(Command::DisplayControl, LCD_DISPLAY_ON));
        try!(self.clear());

        Ok(())
    }

    pub fn install_function_set(&mut self) -> ScreenResult {
        let mut flags = 0;

        flags = flags | (self.config.bit_mode as u8);
        flags = flags | (self.config.line_count as u8);
        flags = flags | (self.config.matrix_size as u8);

        self.command(Command::FunctionSet, flags)
    }

    pub fn clear(&mut self) -> ScreenResult {
        self.command(Command::ClearDisplay, 0)
    }

    pub fn command(&mut self, command: Command, data: u8) -> ScreenResult {
        self.write((command as u8) | data)
    }

    pub fn write(&mut self, command: u8) -> ScreenResult {
        match self.config.bit_mode {
            BitMode::B4 => {
                try!(self.write_screen(command & 0xF0));
                try!(self.write_screen((command << 4) & 0xF0));
                Ok(())
            }
            BitMode::B8 => {
                try!(self.write_screen(command));
                Ok(())
            }
        }
    }

    pub fn write_screen(&mut self, command: u8) -> ScreenResult {
        try!(self.write_cmd(command | LCD_BACKLIGHT_ON));
        Ok(())
    }

    pub fn write_cmd(&mut self, command: u8) -> ScreenResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let config = ScreenConfig::default();
        let mut screen = Screen::new(config, "/dev/null", 0xf3).unwrap();

        screen.init().unwrap();
    }
}
