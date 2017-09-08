// Copyright 2017 Romain Porte
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

// Display entry mode
#[derive(Copy, Clone)]
pub enum EntryMode {
    Right = 0x00,
    Left = 0x02,
}

#[derive(Copy, Clone)]
pub enum EntryShift {
    Increment = 0x01,
    Decrement = 0x00,
}

// Flags for display on/off control

#[derive(Copy, Clone)]
pub enum DisplayState {
    Off = 0x00,
    On = 0x04,
}

#[derive(Copy, Clone)]
pub enum CursorState {
    Off = 0x00,
    On = 0x02,
}

#[derive(Copy, Clone)]
pub enum BlinkState {
    Off = 0x00,
    On = 0x01,
}

// Flags for display/cursor shift

#[derive(Copy, Clone)]
pub enum MoveType {
    Cursor = 0x00,
    Display = 0x08,
}

#[derive(Copy, Clone)]
pub enum MoveDirection {
    Left = 0x00,
    Right = 0x04,
}

#[derive(Copy, Clone)]
pub enum Backlight {
    Off = 0x00,
    On = 0x08,
}

// Specific flags
#[derive(Copy, Clone)]
pub enum WriteMode {
    Enable = 0x04,
    ReadWrite = 0x02,
    RegisterSelect = 0x01,
    Normal = 0x00,
}

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
        try!(self.write(0x03, WriteMode::Normal));
        try!(self.write(0x03, WriteMode::Normal));
        try!(self.write(0x03, WriteMode::Normal));
        try!(self.write(0x02, WriteMode::Normal));

        try!(self.install_function_set());

        try!(self.set_display(DisplayState::On, CursorState::Off, BlinkState::Off));
        try!(self.clear());
        try!(self.set_entry_mode(EntryMode::Left)); // Allow users to change this?

        // Wait a little for the screen to set up
        thread::sleep(Duration::from_millis(200));

        Ok(())
    }

    // High-order commands mapped to methods

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

    pub fn set_entry_mode(&mut self, entry_mode: EntryMode) -> ScreenResult {
        self.command(Command::EntryModeSet, entry_mode as u8)
    }

    pub fn set_display(&mut self,
                       display: DisplayState,
                       cursor: CursorState,
                       blink: BlinkState)
                       -> ScreenResult {
        let mut flags = 0;

        flags = flags | (display as u8);
        flags = flags | (cursor as u8);
        flags = flags | (blink as u8);

        self.command(Command::DisplayControl, flags)
    }

    // Other methods that are not commands

    pub fn set_backlight(&mut self, backlight: bool) -> ScreenResult {
        if backlight {
            self.write_cmd(Backlight::On as u8)
        } else {
            self.write_cmd(Backlight::Off as u8)
        }
    }

    pub fn display(&mut self, s: &str, line: u8, col: u8) -> ScreenResult {
        let pos = match line {
            1 => col,
            2 => 0x40 + col,
            3 => 0x14 + col,
            4 => 0x54 + col,
            _ => col,
        };

        try!(self.write(0x80 + pos, WriteMode::Normal));

        for c in s.chars() {
            try!(self.write_char(c as u8));
        }

        Ok(())
    }

    // Lower-level methods that write commands to device, ordered from higher
    // to lower level of abstraction

    pub fn command(&mut self, command: Command, data: u8) -> ScreenResult {
        self.write((command as u8) | data, WriteMode::Normal)
    }

    pub fn write_char(&mut self, ch: u8) -> ScreenResult {
        self.write(ch, WriteMode::RegisterSelect)
    }

    pub fn write_four_bytes(&mut self, data: u8) -> ScreenResult {
        try!(self.write_screen(data));
        try!(self.strobe(data));
        Ok(())
    }

    pub fn write(&mut self, command: u8, mode: WriteMode) -> ScreenResult {
        match self.config.bit_mode {
            BitMode::B4 => {
                try!(self.write_four_bytes((mode as u8) | (command & 0xF0)));
                try!(self.write_four_bytes((mode as u8) | ((command << 4) & 0xF0)));
                Ok(())
            }
            BitMode::B8 => {
                try!(self.write_screen((mode as u8) | command)); // Not sure here for mode
                Ok(())
            }
        }
    }

    pub fn strobe(&mut self, data: u8) -> ScreenResult {
        try!(self.write_screen(data | (WriteMode::Enable as u8)));
        thread::sleep(Duration::new(0, 50_000));
        try!(self.write_screen(data & !(WriteMode::Enable as u8)));
        thread::sleep(Duration::new(0, 10_000));
        Ok(())
    }

    pub fn write_screen(&mut self, command: u8) -> ScreenResult {
        self.write_cmd(command | (Backlight::On as u8))
    }

    pub fn write_cmd(&mut self, command: u8) -> ScreenResult {
        println!("write byte {}", command);
        try!(self.dev.smbus_write_byte(command));

        // Wait 10 microseconds
        thread::sleep(Duration::new(0, 10_000));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let config = ScreenConfig::default();
    }
}
