use crate::declare_enum;

pub const DF2301Q_I2C_ADDRESS: u8 = 0x64;

declare_enum! {
    I2CRegisters {
        Begin = 0x00,
        CommandId = 0x02,
        PlayCommandId = 0x03,
        SetMute = 0x04,
        SetVolume = 0x05,
        WakeTime = 0x06,
        MessageTail = 0x5A,
    }
}
