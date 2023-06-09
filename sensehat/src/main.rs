extern crate libc;

use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use sensehat::matrix::{SenseHatMatrix, LedColor};

fn main() {
    // I2C-apparaat initialiseren voor de Sense HAT
    let i2c_device = File::open("/dev/i2c-1").expect("Failed to open I2C device");

    // LED-matrix initialiseren
    let mut matrix = SenseHatMatrix::new();

    loop {
        // Lees de temperatuurwaarde van de sensor
        let temperature = read_temperature(&i2c_device);

        // Converteer de temperatuur naar een string met één decimaal
        let temperature_str = format!("{:.1}", temperature as f32 / 1000.0);

        // Weergave op de LED-matrix
        matrix.set_rotation(sensehat::Rotation::Rot0);
        matrix.set_text(&temperature_str, LedColor::Green, LedColor::Off);

        // Wacht 2 seconden voordat we de temperatuur opnieuw lezen
        thread::sleep(Duration::from_secs(2));
    }
}

fn read_temperature(i2c_device: &File) -> i32 {
    let slave_address = 0x46; // Adres van de Sense HAT-sensor
    let register = 0xE3; // Register voor de temperatuur

    let mut buffer: [u8; 2] = [0, 0];

    // Schrijf het registeradres naar het apparaat
    i2c_device.write_all(&[register]).expect("Failed to write to I2C device");

    // Lees 2 bytes van het apparaat
    i2c_device.read_exact(&mut buffer).expect("Failed to read from I2C device");

    let raw_temp = ((buffer[0] as u16) << 8) | (buffer[1] as u16);
    let temp = -46.85 + (175.72 * (raw_temp as f32) / 65536.0);
    temp as i32
}
