use crate::declare_enum;

pub const DF2301Q_UART_MESSAGE_DATA_MAX_SIZE: usize = 8;
pub const DF2301Q_UART_MESSAGE_HEAD_LOW: u8 = 0xF4;
pub const DF2301Q_UART_MESSAGE_HEAD_HIGH: u8 = 0xF5;
pub const DF2301Q_UART_MESSAGE_TAIL: u8 = 0xFB;

#[derive(Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Message {
    pub command: Command,
    pub message_type: MessageType,
    pub sequence: u8,
    pub data: Vec<u8>,
}

#[allow(clippy::derivable_impls)]
impl Default for MessageType {
    fn default() -> Self {
        MessageType::CommandDown
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Command {
    fn default() -> Self {
        Command::PlayVoice
    }
}

declare_enum! {
    PlayVoiceData {
        PlayStart = 0x80,
        PlayPause = 0x81,
        PlayResume = 0x82,
        PlayStop = 0x83,
        PlayByVoiceId = 0x90,
        PlayBySemanticId = 0x91,
        PlayByCommandId = 0x92,
    }
}

declare_enum! {
    GetVersionData {
        Protocol = 0x80,
        SDK = 0x81,
        Asr = 0x82,
        Preprocess = 0x83,
        Player = 0x84,
        Application = 0x8A,
    }
}

declare_enum! {
    NotifyStatus {
        PowerOn = 0xB0,
        WakeupEnter = 0xB1,
        WakeupExit = 0xB2,
        PlayStart = 0xB3,
        PlayEnd = 0xB4,
    }
}

declare_enum! {
    ConfigKey {
        SetVolume = 0x80,
        SetEnterWakeup = 0x81,
        SetPrtMidRst = 0x82,
        SetMute = 0x83,
        SetWakeTime = 0x84,
        SetNeedAcknowledge = 0x90,
        SetNeedString = 0x91,
    }
}

declare_enum! {
    Command {
        GetCommandId = 0x00,
        AsrResult = 0x91,
        PlayVoice = 0x92,
        GetFlashUid = 0x93,
        GetVersion = 0x94,
        ResetModule = 0x95,
        SetConfig = 0x96,
        EnterOTAMode = 0x97,
        NotifyStatus = 0x9A,
        AcknowledgeCommon = 0xAA,
    }
}

declare_enum! {
    AcknowledgeResponse {
        ErrorNone = 0x0,
        ErrorChecksum = 0xff,
        ErrorNoSupport = 0xfe,
    }
}

declare_enum! {
    MessageType {
        CommandUp = 0xA0,
        CommandDown = 0xA1,
        Acknowledge = 0xA2,
        Notify = 0xA3,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Payload<T> {
    pub key: T,
    pub value: Option<u32>,
}

impl From<Payload<PlayVoiceData>> for Vec<u8> {
    fn from(payload: Payload<PlayVoiceData>) -> Self {
        let mut data = vec![];
        match payload.key {
            PlayVoiceData::PlayByCommandId => {
                data.push(PlayVoiceData::PlayStart.into());
                data.push(PlayVoiceData::PlayByCommandId.into());
            }
            _ => {
                data.push(payload.key.into());
            }
        }
        if let Some(value) = payload.value {
            data.append(&mut value.to_le_bytes().to_vec());
        }
        data
    }
}

impl From<Payload<GetVersionData>> for Vec<u8> {
    fn from(payload: Payload<GetVersionData>) -> Self {
        let mut data = vec![];
        data.push(payload.key.into());
        if let Some(value) = payload.value {
            data.append(&mut value.to_le_bytes().to_vec());
        }
        data
    }
}

impl From<Payload<NotifyStatus>> for Vec<u8> {
    fn from(payload: Payload<NotifyStatus>) -> Self {
        let mut data = vec![];
        data.push(payload.key.into());
        if let Some(value) = payload.value {
            data.append(&mut value.to_le_bytes().to_vec());
        }
        data
    }
}

impl From<Payload<ConfigKey>> for Vec<u8> {
    fn from(payload: Payload<ConfigKey>) -> Self {
        let mut data = vec![];
        data.push(payload.key.into());
        if let Some(value) = payload.value {
            data.append(&mut value.to_le_bytes().to_vec());
        }
        data
    }
}

struct TextMessage(String);
impl From<TextMessage> for Vec<u8> {
    fn from(msg: TextMessage) -> Self {
        msg.0.as_bytes().to_vec()
    }
}
pub struct ResetPayload;
impl From<ResetPayload> for Vec<u8> {
    fn from(_: ResetPayload) -> Self {
        vec!["r", "e", "s", "e", "t"]
            .into_iter()
            .map(|s| s.as_bytes()[0])
            .collect()
    }
}
