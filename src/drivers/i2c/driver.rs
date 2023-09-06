use super::types::*;
use crate::CommandWord;
use crate::CommunicationInterface;
use crate::Df2301qI2CError;
use embedded_hal::blocking::i2c;

pub struct Df2301qI2C<I2C, E>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    i2c: I2C,
}

impl<I2C, E> CommunicationInterface for Df2301qI2C<I2C, E>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    type Error = Df2301qI2CError<E>;

    fn get_command_word(&mut self) -> Result<CommandWord, Self::Error> {
        let mut buffer: [u8; 1] = [0];
        self.i2c
            .write_read(
                DF2301Q_I2C_ADDRESS,
                &[I2CRegisters::CommandId.into()],
                &mut buffer,
            )
            .map_err(Df2301qI2CError::I2CError)?;
        Ok(CommandWord::from(buffer[0]))
    }

    fn play_by_command_word(&mut self, command_word: CommandWord) -> Result<(), Self::Error> {
        self.i2c.write(
            DF2301Q_I2C_ADDRESS,
            &[I2CRegisters::PlayCommandId.into(), command_word.into()],
        )?;
        Ok(())
    }

    fn set_volume(&mut self, volume: u32) -> Result<(), Self::Error> {
        self.i2c.write(
            DF2301Q_I2C_ADDRESS,
            &[I2CRegisters::SetVolume.into(), volume as u8],
        )?;
        Ok(())
    }

    fn set_mute(&mut self, mute: bool) -> Result<(), Self::Error> {
        self.i2c.write(
            DF2301Q_I2C_ADDRESS,
            &[I2CRegisters::SetMute.into(), mute as u8],
        )?;
        Ok(())
    }

    fn set_wake_time(&mut self, wake_time: u32) -> Result<(), Self::Error> {
        self.i2c.write(
            DF2301Q_I2C_ADDRESS,
            &[I2CRegisters::WakeTime.into(), wake_time as u8],
        )?;
        Ok(())
    }
}

impl<I2C, E> Df2301qI2C<I2C, E>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn begin(&mut self) -> Result<(), Df2301qI2CError<E>> {
        self.i2c
            .write(DF2301Q_I2C_ADDRESS, &[I2CRegisters::Begin.into()])
            .map_err(Df2301qI2CError::I2CError)?;
        Ok(())
    }

    pub fn wake_time(&mut self) -> Result<u8, Df2301qI2CError<E>> {
        let mut buffer: [u8; 1] = [0];
        self.i2c
            .write_read(
                DF2301Q_I2C_ADDRESS,
                &[I2CRegisters::WakeTime.into()],
                &mut buffer,
            )
            .map_err(Df2301qI2CError::I2CError)?;
        Ok(buffer[0])
    }

    pub fn message_tail(&mut self) -> Result<u8, Df2301qI2CError<E>> {
        let mut buffer: [u8; 1] = [0];
        self.i2c
            .write_read(
                DF2301Q_I2C_ADDRESS,
                &[I2CRegisters::MessageTail.into()],
                &mut buffer,
            )
            .map_err(Df2301qI2CError::I2CError)?;
        Ok(buffer[0])
    }
}
