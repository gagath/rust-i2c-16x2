extern crate i2c_16x2;

use i2c_16x2::*;
use std::thread;
use std::time::Duration;


fn main() {
    let config = ScreenConfig::default();
    let mut screen = Screen::new(config, "/dev/i2c-1", 0x3f).expect("Could not init device");

    println!("init");
    screen.init().unwrap();
    println!("off");
    screen.set_backlight(false).unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("on");
    screen.set_backlight(true).unwrap();
    thread::sleep(Duration::from_secs(1));
    //screen.display("hello rust", 1, 0).unwrap();
    screen.write_char('k' as u8);
}
