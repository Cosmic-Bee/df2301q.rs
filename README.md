# DFRobot DF2301Q Gravity Voice Recognition Module Rust Driver

This is a rust driver for the [DF2301Q Gravity Voice Recognition Module](https://wiki.dfrobot.com/SKU_SEN0539-EN_Gravity_Voice_Recognition_Module_I2C_UART) from DFRobot. 

For Python and C++ libraries see the [associated DFRobot repository](https://github.com/DFRobot/DFRobot_DF2301Q).

This module allows a user over i2c or uart to connect and monitor for a set of preset command words or even train their own for triggering behaviors.

## Building

Make sure you [install the dependencies](https://esp-rs.github.io/book/installation/rust.html) for embedded rust and install all of the tooling where needed.

To build, flash, and monitor with the espflash cargo command as such:

`cargo espflash flash --monitor`

## Example
### UART

```rs
    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio43;
    let rx = peripherals.pins.gpio44;

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

    df2301q.play_by_command_word(CommandWord::OpenCurtain).unwrap();

    loop {
        let command_word = df2301q.get_command_word().unwrap();
        match command_word {
            CommandWord::TurnOnTheLight => {
                println!("TurnOnTheLight: Turning On Light");
            },
            CommandWord::TurnOffTheLight => {
                println!("TurnOffTheLight: Turning Off Light");
            },
            CommandWord::Reset => df2301q.reset().unwrap(),
            CommandWord::Silence => (), // Do nothing
            _ => println!("Found: {command_word:?}")
        }
    }
```

### I2C

```rs
    let peripherals = Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let config = Config::new().baudrate(Hertz(115200));
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &config,
    )
    .unwrap();
    let mut df2301q = df2301q::Df2301qI2C::new(i2c);

    df2301q.play_by_command_word(CommandWord::OpenCurtain).unwrap();

    loop {
        let command_word = df2301q.get_command_word().unwrap();
        match command_word {
            CommandWord::TurnOnTheLight => {
                println!("TurnOnTheLight: Turning On Light");
            },
            CommandWord::TurnOffTheLight => {
                println!("TurnOffTheLight: Turning Off Light");
            },
            CommandWord::Reset => df2301q.reset().unwrap(),
            CommandWord::Silence => (), // Do nothing
            _ => println!("Found: {command_word:?}")
        }
    }
```