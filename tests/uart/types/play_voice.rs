use df2301q::*;
use embedded_hal_mock::serial::{Mock, Transaction as MockTransaction};

#[test]
fn play_by_voice_id_send() {
    let mut data = vec![PlayVoiceData::PlayByVoiceId.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        1 + u8::from(MessageType::CommandDown) as u16 + u8::from(Command::PlayVoice) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::PlayVoice.into()));
    transactions.push(MockTransaction::write(1)); // Sequence

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
        command: Command::PlayVoice,
        message_type: MessageType::CommandDown,
        sequence: 1,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn play_by_command_id_send() {
    let mut data = vec![
        PlayVoiceData::PlayStart.into(),
        PlayVoiceData::PlayByCommandId.into(),
    ];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 6);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::PlayVoice) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::PlayVoice.into()));
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
        command: Command::PlayVoice,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn play_by_semantic_id_send() {
    let mut data = vec![PlayVoiceData::PlayBySemanticId.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::PlayVoice) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::PlayVoice.into()));
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
        command: Command::PlayVoice,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn play_start_send() {
    let mut data = vec![PlayVoiceData::PlayStart.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::PlayVoice) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::PlayVoice.into()));
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
        command: Command::PlayVoice,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn play_pause_send() {
    let mut data = vec![PlayVoiceData::PlayPause.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::PlayVoice) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::PlayVoice.into()));
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
        command: Command::PlayVoice,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn play_resume_send() {
    let mut data = vec![PlayVoiceData::PlayResume.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::PlayVoice) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::PlayVoice.into()));
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
        command: Command::PlayVoice,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn play_stop_send() {
    let mut data = vec![PlayVoiceData::PlayStop.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::PlayVoice) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::PlayVoice.into()));
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
        command: Command::PlayVoice,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}
