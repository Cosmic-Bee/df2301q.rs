pub use self::i2c::*;
mod i2c;
pub use self::types::*;
pub mod types;
pub use self::uart::*;
mod uart;

macro_rules! declare_enum {
    ($name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum $name {
            $($variant,)*
        }

        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant => $value,)*
                }
            }
        }

        impl From<u8> for $name {
            fn from(byte: u8) -> Self {
                match byte {
                    $($value => Self::$variant,)*
                    _ => { panic!("Invalid value for {}: {}", stringify!($name), byte) }
                }
            }
        }
    };
}
pub(crate) use declare_enum;

pub trait CommunicationInterface {
    type Error;

    /// Gets the command ID of the message from the device.
    ///
    /// # Returns
    ///
    /// * `Ok(u8)` if the command ID is successfully received.
    /// * `Err(Self::Error)` if an error occurs during the receive action.
    fn get_command_word(&mut self) -> Result<CommandWord, Self::Error>;

    /// Plays a specific track or function on the device by its command ID.
    ///
    /// # Arguments
    ///
    /// * `cmd_id` - The command ID of the track or function to play.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the track or function is successfully played.
    /// * `Err(Self::Error)` if an error occurs during the play action.
    fn play_by_command_word(&mut self, command_word: CommandWord) -> Result<(), Self::Error>;

    /// Sets the volume level of the device.
    ///
    /// # Arguments
    ///
    /// * `volume` - The volume level to set, usually between 0 and 100.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the volume is successfully set.
    /// * `Err(Self::Error)` if an error occurs.
    fn set_volume(&mut self, volume: u32) -> Result<(), Self::Error>;

    /// Sets the mute mode of the device.
    ///
    /// # Arguments
    ///
    /// * `mute` - The mute mode to set.
    ///     
    /// # Returns
    ///
    /// * `Ok(())` if the mute mode is successfully set.
    /// * `Err(Self::Error)` if an error occurs.
    fn set_mute(&mut self, mute: bool) -> Result<(), Self::Error>;

    /// Sets the wake time for the device.
    ///
    /// # Arguments
    ///
    /// * `wake_time` - The time to set for waking up the device.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the wake time is successfully set.
    /// * `Err(Self::Error)` if an error occurs.
    fn set_wake_time(&mut self, wake_time: u32) -> Result<(), Self::Error>;
}
