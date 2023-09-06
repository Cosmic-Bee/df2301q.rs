use df2301q::CommandWord;
use df2301q::CommunicationInterface;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::uart::config::Config;
use esp_idf_hal::uart::UartDriver;
use esp_idf_hal::units::Hertz;
use crate::gpio::PinDriver;
use esp_idf_sys as _;

fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio43;
    let rx = peripherals.pins.gpio44;

    // For LED example
    let mut led = PinDriver::output(peripherals.pins.gpio8).unwrap();
    led.set_low().unwrap();

    let config = Config::new().baudrate(Hertz(9600));
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )
    .unwrap();
    let (tx, rx) = uart.split();
    let mut df2301q = df2301q::Df2301qUART::new(rx, tx);
    df2301q.set_volume(10).unwrap();
    // df2301q.set_mute(true).unwrap();
    // df2301q.set_wake_time(30).unwrap();
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
            CommandWord::Reset => df2301q.reset().unwrap(),
            CommandWord::Silence => (), // Do nothing
            _ => println!("Found: {command_word:?}")
        }
    }
}
