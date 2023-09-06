use df2301q::*;
use embedded_hal_mock::serial::{Mock, Transaction as MockTransaction};

#[test]
fn mute_send() {
    let mut data = vec![ConfigKey::SetMute.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::SetConfig) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::SetConfig.into()));
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
        command: Command::SetConfig,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn set_volume_send() {
    let mut data = vec![ConfigKey::SetVolume.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::SetConfig) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::SetConfig.into()));
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
        command: Command::SetConfig,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn set_enter_wakeup_send() {
    let mut data = vec![ConfigKey::SetEnterWakeup.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::SetConfig) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::SetConfig.into()));
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
        command: Command::SetConfig,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn set_prt_mid_rst_send() {
    let mut data = vec![ConfigKey::SetPrtMidRst.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::SetConfig) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::SetConfig.into()));
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
        command: Command::SetConfig,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn set_wake_time_send() {
    let mut data = vec![ConfigKey::SetWakeTime.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::SetConfig) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::SetConfig.into()));
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
        command: Command::SetConfig,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn set_need_acknowledge_send() {
    let mut data = vec![ConfigKey::SetNeedAcknowledge.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::SetConfig) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::SetConfig.into()));
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
        command: Command::SetConfig,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}

#[test]
fn set_need_string_send() {
    let mut data = vec![ConfigKey::SetNeedString.into()];
    data.append((1 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    let mut checksum: u16 =
        u8::from(MessageType::CommandDown) as u16 + u8::from(Command::SetConfig) as u16;
    let mut transactions = vec![
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::write(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let length_bytes = (data.len() as u16).to_le_bytes();
    transactions.push(MockTransaction::write(length_bytes[0]));
    transactions.push(MockTransaction::write(length_bytes[1]));

    transactions.push(MockTransaction::write(MessageType::CommandDown.into()));
    transactions.push(MockTransaction::write(Command::SetConfig.into()));
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
        command: Command::SetConfig,
        message_type: MessageType::CommandDown,
        sequence: 0,
        data,
    };
    assert!(uart.send_message(&message).is_ok());
}
