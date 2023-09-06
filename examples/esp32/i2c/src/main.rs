use std::thread;
use std::time::Duration;
use df2301q::CommandWord;
use df2301q::CommunicationInterface;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::i2c::config::Config;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::units::Hertz;
use esp_idf_sys as _;

fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    // For LED example
    let mut led = PinDriver::output(peripherals.pins.gpio8).unwrap();
    led.set_low().unwrap();

    let config = Config::new().baudrate(Hertz(115200));
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &config,
    )
    .unwrap();
    let mut df2301q = df2301q::Df2301qI2C::new(i2c);
    df2301q.begin().unwrap();
    df2301q.set_volume(10).unwrap();
    df2301q.set_mute(false).unwrap();
    df2301q.set_wake_time(15).unwrap();
    let wake_time = df2301q.wake_time().unwrap();
    println!("Returned wake time: {wake_time}");
    df2301q.play_by_command_word(CommandWord::OpenCurtain).unwrap();

    loop {
        let command_word = df2301q.get_command_word().unwrap();
        match command_word {
            CommandWord::TurnOnTheLight => {
                println!("TurnOnTheLight: Turning On Light");
                led.set_high().unwrap();
            },
            CommandWord::TurnOffTheLight => {
                println!("TurnOffTheLight: Turning Off Light");
                led.set_low().unwrap();
            },
            CommandWord::Silence => (), // Do nothing
            _ => println!("Found: {command_word:?}")
        }
        thread::sleep(Duration::from_millis(50));
    }
}
