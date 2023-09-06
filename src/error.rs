#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Df2301qUARTError<RX, TX> {
    SerialReadError(RX),
    SerialWriteError(TX),
    IncompleteMessage,
}

impl<RX, TX> From<TX> for Df2301qUARTError<RX, TX> {
    fn from(err: TX) -> Self {
        Df2301qUARTError::SerialWriteError(err)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Df2301qI2CError<I2C> {
    I2CError(I2C),
    IncompleteMessage,
}

impl<I2C> From<I2C> for Df2301qI2CError<I2C> {
    fn from(err: I2C) -> Self {
        Df2301qI2CError::I2CError(err)
    }
}
