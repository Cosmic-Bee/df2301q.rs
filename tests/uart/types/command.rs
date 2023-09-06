use df2301q::*;
use embedded_hal_mock::serial::{Mock, Transaction as MockTransaction};

#[test]
fn asr_result_send() {
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    // 0 length
    transactions.push(MockTransaction::write(0));
    transactions.push(MockTransaction::write(0));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::AsrResult.into()));
    transactions.push(MockTransaction::write(0)); // Sequence

    let checksum: u32 =
        u8::from(MessageType::CommandDown) as u32 + u8::from(Command::AsrResult) as u32;
    transactions.push(MockTransaction::write((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::write(((checksum >> 8) & 0xFF) as u8));

    transactions.push(MockTransaction::write(DF2301Q_UART_MESSAGE_TAIL));
    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let message = Message {
        command: Command::AsrResult,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data: Vec::new(),
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn get_flash_uid_send() {
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    // 0 length
    transactions.push(MockTransaction::write(0));
    transactions.push(MockTransaction::write(0));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::GetFlashUid.into()));
    transactions.push(MockTransaction::write(0)); // Sequence

    let checksum: u32 =
        u8::from(MessageType::CommandDown) as u32 + u8::from(Command::GetFlashUid) as u32;
    transactions.push(MockTransaction::write((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::write(((checksum >> 8) & 0xFF) as u8));

    transactions.push(MockTransaction::write(DF2301Q_UART_MESSAGE_TAIL));
    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let message = Message {
        command: Command::GetFlashUid,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data: Vec::new(),
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn get_version_send() {
    let mut data = vec![GetVersionData::SDK.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::GetVersion) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::GetVersion.into()));
    transactions.push(MockTransaction::write(0)); // Sequence

    for byte in &data {
        transactions.push(MockTransaction::write(*byte as u8));
        checksum += *byte as u16;
    }

    transactions.push(MockTransaction::write((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::write(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::write(DF2301Q_UART_MESSAGE_TAIL));
    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let message = Message {
        command: Command::GetVersion,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn reset_module_send() {
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    // TODO: Fix this to have proper 5 length and reset data
    // 0 length
    transactions.push(MockTransaction::write(0));
    transactions.push(MockTransaction::write(0));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::ResetModule.into()));
    transactions.push(MockTransaction::write(0)); // Sequence

    let checksum =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::ResetModule) as u16;
    transactions.push(MockTransaction::write((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::write(((checksum >> 8) & 0xFF) as u8));

    transactions.push(MockTransaction::write(DF2301Q_UART_MESSAGE_TAIL));
    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let message = Message {
        command: Command::ResetModule,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data: Vec::new(),
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn enter_ota_mode_send() {
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    // 0 length
    transactions.push(MockTransaction::write(0));
    transactions.push(MockTransaction::write(0));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::EnterOTAMode.into()));
    transactions.push(MockTransaction::write(0)); // Sequence

    let checksum =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::EnterOTAMode) as u16;
    transactions.push(MockTransaction::write((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::write(((checksum >> 8) & 0xFF) as u8));

    transactions.push(MockTransaction::write(DF2301Q_UART_MESSAGE_TAIL));
    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let message = Message {
        command: Command::EnterOTAMode,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data: Vec::new(),
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn notify_status_send() {
    let mut data = vec![NotifyStatus::WakeupEnter.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::Notify) as u16 + u8::from(Command::NotifyStatus) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::Notify.into()));
    transactions.push(MockTransaction::write(Command::NotifyStatus.into()));
    transactions.push(MockTransaction::write(0)); // Sequence

    for byte in &data {
        transactions.push(MockTransaction::write(*byte as u8));
        checksum += *byte as u16;
    }

    transactions.push(MockTransaction::write((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::write(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::write(DF2301Q_UART_MESSAGE_TAIL));
    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let message = Message {
        command: Command::NotifyStatus,
        message_type: MessageType::Notify,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn acknowledge_common_send() {
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    transactions.push(MockTransaction::write(0));
    transactions.push(MockTransaction::write(0));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::AcknowledgeCommon.into()));
    transactions.push(MockTransaction::write(0)); // Sequence

    let checksum =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::AcknowledgeCommon) as u16;
    transactions.push(MockTransaction::write((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::write(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::write(DF2301Q_UART_MESSAGE_TAIL));
    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let message = Message {
        command: Command::AcknowledgeCommon,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data: Vec::new(),
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn acknowledge_receive() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 1;
    let length_bytes = (1 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::Acknowledge.into()));
    transactions.push(MockTransaction::read(Command::AcknowledgeCommon.into()));
    transactions.push(MockTransaction::read(sequence)); // Sequence

    let mut checksum: u16 = sequence as u16
        + u8::from(MessageType::Acknowledge) as u16
        + u8::from(Command::AcknowledgeCommon) as u16;
    let data = u8::from(AcknowledgeResponse::ErrorNone);
    checksum += data as u16;
    transactions.push(MockTransaction::read(data));
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));

    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);
    let received_message = uart.receive_message().unwrap();
    assert_eq!(Command::AcknowledgeCommon, received_message.command);
    assert_eq!(MessageType::Acknowledge, received_message.message_type);
    assert_eq!(sequence, received_message.sequence);
    assert_eq!(1, received_message.data.len());
    assert_eq!(
        AcknowledgeResponse::ErrorNone,
        (*received_message.data.first().unwrap()).into()
    );
}
