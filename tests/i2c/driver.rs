use df2301q::*;
use embedded_hal_mock::i2c::{Mock, Transaction as MockTransaction};

#[test]
fn get_command_word() {
    let transactions = vec![MockTransaction::write_read(
        DF2301Q_I2C_ADDRESS,
        vec![I2CRegisters::CommandId.into()],
        vec![u8::from(CommandWord::BluetoothMode)],
    )];

    let mock = Mock::new(&transactions);
    let mut i2c = Df2301qI2C::new(mock);

    let command_word: CommandWord = i2c.get_command_word().unwrap();
    assert_eq!(command_word, CommandWord::BluetoothMode);
}

#[test]
fn play_by_command_word() {
    let transactions = vec![MockTransaction::write(
        DF2301Q_I2C_ADDRESS,
        vec![
            I2CRegisters::PlayCommandId.into(),
            u8::from(CommandWord::BluetoothMode),
        ],
    )];

    let mock = Mock::new(&transactions);
    let mut i2c = Df2301qI2C::new(mock);

    i2c.play_by_command_word(CommandWord::BluetoothMode)
        .unwrap();
}

#[test]
fn set_volume() {
    let transactions = vec![MockTransaction::write(
        DF2301Q_I2C_ADDRESS,
        vec![I2CRegisters::SetVolume.into(), 20],
    )];

    let mock = Mock::new(&transactions);
    let mut i2c = Df2301qI2C::new(mock);

    i2c.set_volume(20).unwrap();
}

#[test]
fn set_mute() {
    let transactions = vec![MockTransaction::write(
        DF2301Q_I2C_ADDRESS,
        vec![I2CRegisters::SetMute.into(), 1],
    )];

    let mock = Mock::new(&transactions);
    let mut i2c = Df2301qI2C::new(mock);

    i2c.set_mute(true).unwrap();
}

#[test]
fn set_wake_time() {
    let transactions = vec![MockTransaction::write(
        DF2301Q_I2C_ADDRESS,
        vec![I2CRegisters::WakeTime.into(), 1],
    )];

    let mock = Mock::new(&transactions);
    let mut i2c = Df2301qI2C::new(mock);

    i2c.set_wake_time(1).unwrap();
}

#[test]
fn wake_time() {
    let transactions = vec![MockTransaction::write_read(
        DF2301Q_I2C_ADDRESS,
        vec![I2CRegisters::WakeTime.into()],
        vec![1],
    )];

    let mock = Mock::new(&transactions);

    let mut i2c = Df2301qI2C::new(mock);
    let wake_time = i2c.wake_time().unwrap();
    assert_eq!(wake_time, 1);
}
