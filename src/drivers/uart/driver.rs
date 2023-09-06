use crate::Command;
use crate::CommandWord;
use crate::CommunicationInterface;
use crate::ConfigKey;
use crate::Df2301qUARTError;
use crate::Message;
use crate::MessageType;
use crate::Payload;
use crate::PlayVoiceData;
use crate::ResetPayload;
use crate::DF2301Q_UART_MESSAGE_DATA_MAX_SIZE;
use crate::DF2301Q_UART_MESSAGE_HEAD_HIGH;
use crate::DF2301Q_UART_MESSAGE_HEAD_LOW;
use crate::DF2301Q_UART_MESSAGE_TAIL;
use embedded_hal::serial::Read;
use embedded_hal::serial::Write;
use nb::block;
use std::thread;
use std::time::Duration;

pub struct Df2301qUART<RX, TX> {
    tx: TX,
    rx: RX,
    send_sequence: u8,
}

impl<RX, TX> CommunicationInterface for Df2301qUART<RX, TX>
where
    RX: Read<u8>,
    TX: Write<u8>,
{
    type Error = Df2301qUARTError<RX::Error, TX::Error>;

    fn get_command_word(&mut self) -> Result<CommandWord, Self::Error> {
        let command_id = *self
            .receive_message()?
            .data
            .first()
            .ok_or(Df2301qUARTError::IncompleteMessage)?;

        thread::sleep(Duration::from_millis(50));
        Ok(CommandWord::from(command_id))
    }

    fn play_by_command_word(&mut self, command_word: CommandWord) -> Result<(), Self::Error> {
        self.send_uart_message(
            Command::PlayVoice,
            Some(Payload {
                key: PlayVoiceData::PlayByCommandId,
                value: Some(u8::from(command_word) as u32),
            }),
        )
    }

    fn set_volume(&mut self, volume: u32) -> Result<(), Self::Error> {
        self.send_uart_message(
            Command::SetConfig,
            Some(Payload {
                key: ConfigKey::SetVolume,
                value: Some(volume),
            }),
        )
    }

    fn set_mute(&mut self, mute: bool) -> Result<(), Self::Error> {
        self.send_uart_message(
            Command::SetConfig,
            Some(Payload {
                key: ConfigKey::SetMute,
                value: Some(mute as u32),
            }),
        )
    }

    fn set_wake_time(&mut self, wake_time: u32) -> Result<(), Self::Error> {
        self.send_uart_message(
            Command::SetConfig,
            Some(Payload {
                key: ConfigKey::SetWakeTime,
                value: Some(wake_time),
            }),
        )
    }
}

impl<RX, TX> Df2301qUART<RX, TX>
where
    RX: Read<u8>,
    TX: Write<u8>,
{
    pub fn new(rx: RX, tx: TX) -> Self {
        Self {
            tx,
            rx,
            send_sequence: 0,
        }
    }

    pub fn reset(&mut self) -> Result<(), Df2301qUARTError<RX::Error, TX::Error>> {
        self.send_uart_message(Command::ResetModule, Some(ResetPayload))?;

        Ok(())
    }

    pub fn send_uart_message<T: Into<Vec<u8>>>(
        &mut self,
        command: Command,
        payload: Option<T>,
    ) -> Result<(), Df2301qUARTError<RX::Error, TX::Error>> {
        let data: Vec<u8> = payload.map(|p| p.into()).unwrap_or_default();
        let message = Message {
            message_type: MessageType::CommandDown,
            sequence: self.send_sequence,
            command,
            data,
        };

        self.send_message(&message)?;
        self.send_sequence += 1;

        Ok(())
    }

    pub fn send_message(
        &mut self,
        message: &Message,
    ) -> Result<(), Df2301qUARTError<RX::Error, TX::Error>> {
        let mut checksum: u16 = 0;
        let data_length = message.data.len() as u16;

        let header: u16 =
            (DF2301Q_UART_MESSAGE_HEAD_HIGH as u16) << 8 | (DF2301Q_UART_MESSAGE_HEAD_LOW as u16);
        let header_bytes = header.to_le_bytes();
        block!(self.tx.write(header_bytes[0]))?;
        block!(self.tx.write(header_bytes[1]))?;

        let length_bytes = data_length.to_le_bytes();
        block!(self.tx.write(length_bytes[0]))?;
        block!(self.tx.write(length_bytes[1]))?;

        let message_type: u8 = message.message_type.clone().into();
        block!(self.tx.write(message_type))?;
        let command: u8 = message.command.clone().into();
        block!(self.tx.write(command))?;
        block!(self.tx.write(message.sequence))?;
        checksum += message_type as u16;
        checksum += command as u16;
        checksum += message.sequence as u16;

        for byte in &message.data {
            checksum += *byte as u16;
            block!(self.tx.write(*byte))?;
        }

        block!(self.tx.write((checksum & 0xff) as u8))?;
        block!(self.tx.write(((checksum >> 8) & 0xff) as u8))?;

        block!(self.tx.write(DF2301Q_UART_MESSAGE_TAIL))?;

        thread::sleep(Duration::from_millis(100));
        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<Message, Df2301qUARTError<RX::Error, TX::Error>> {
        #[derive(Debug, PartialEq, Eq)]
        enum ReceiveState {
            Head0,
            Head1,
            Length0,
            Length1,
            Type,
            Command,
            Sequence,
            Data,
            Checksum0,
            Checksum1,
            Tail,
        }

        let mut receive_state = ReceiveState::Head0;
        let mut length0: u8 = 0;
        let mut length1: u16 = 0;
        let mut message_checksum: u8 = 0;
        let mut message = Message::default();
        loop {
            let receive_char = match self.rx.read() {
                Ok(char) => Ok(char),
                Err(nb::Error::WouldBlock) => {
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }
                _ => Err(Df2301qUARTError::IncompleteMessage),
            }?;

            match receive_state {
                ReceiveState::Head0 => {
                    if receive_char == DF2301Q_UART_MESSAGE_HEAD_LOW {
                        receive_state = ReceiveState::Head1;
                    }
                }
                ReceiveState::Head1 => {
                    if receive_char == DF2301Q_UART_MESSAGE_HEAD_HIGH {
                        receive_state = ReceiveState::Length0;
                    } else if receive_char != DF2301Q_UART_MESSAGE_HEAD_LOW {
                        receive_state = ReceiveState::Head0;
                    }
                }
                ReceiveState::Length0 => {
                    length0 = receive_char;
                    receive_state = ReceiveState::Length1;
                }
                ReceiveState::Length1 => {
                    length1 = u16::from(receive_char) << 8;
                    length1 += u16::from(length0);

                    if length1 <= DF2301Q_UART_MESSAGE_DATA_MAX_SIZE as u16 {
                        receive_state = ReceiveState::Type;
                    } else {
                        receive_state = ReceiveState::Head0;
                    }
                }
                ReceiveState::Type => {
                    message.message_type = MessageType::from(receive_char);
                    receive_state = ReceiveState::Command;
                }
                ReceiveState::Command => {
                    message.command = Command::from(receive_char);
                    receive_state = ReceiveState::Sequence;
                }
                ReceiveState::Sequence => {
                    message.sequence = receive_char;
                    if length1 > 0 {
                        receive_state = ReceiveState::Data;
                    } else {
                        receive_state = ReceiveState::Checksum0;
                    }
                }
                ReceiveState::Data => {
                    message.data.push(receive_char);

                    if message.data.len() == length1 as usize {
                        receive_state = ReceiveState::Checksum0;
                    }
                }
                ReceiveState::Checksum0 => {
                    message_checksum = receive_char;
                    receive_state = ReceiveState::Checksum1;
                }
                ReceiveState::Checksum1 => {
                    let mut checksum = u16::from(receive_char) << 8;
                    checksum += u16::from(message_checksum);
                    let mut packet_chk_sum: u16 = message.data.iter().map(|d| u16::from(*d)).sum();
                    packet_chk_sum += u8::from(message.command.clone()) as u16;
                    packet_chk_sum += u8::from(message.message_type.clone()) as u16;
                    packet_chk_sum += message.sequence as u16;
                    if checksum == packet_chk_sum {
                        receive_state = ReceiveState::Tail;
                    } else {
                        receive_state = ReceiveState::Head0;
                    }
                }
                ReceiveState::Tail => {
                    if receive_char == DF2301Q_UART_MESSAGE_TAIL {
                        break;
                    }
                    receive_state = ReceiveState::Head0;
                    message.data = Vec::new();
                }
            }
        }

        if message.data.len() == length1 as usize && receive_state == ReceiveState::Tail {
            Ok(message)
        } else {
            Err(Df2301qUARTError::IncompleteMessage)
        }
    }
}
