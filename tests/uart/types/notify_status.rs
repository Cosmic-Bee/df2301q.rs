use df2301q::*;
use embedded_hal_mock::serial::{Mock, Transaction as MockTransaction};

#[test]
fn notify_power_on_receive() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 0;
    let length_bytes = (5 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::Notify.into()));
    transactions.push(MockTransaction::read(Command::NotifyStatus.into()));
    transactions.push(MockTransaction::read(sequence));

    let mut data = vec![NotifyStatus::PowerOn.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    for byte in &data {
        transactions.push(MockTransaction::read(*byte as u8));
    }

    let checksum = data.iter().map(|d| u16::from(*d)).sum::<u16>()
        + u8::from(Command::NotifyStatus) as u16
        + sequence as u16
        + u8::from(MessageType::Notify) as u16;
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);

    let expected_message = Message {
        command: Command::NotifyStatus,
        message_type: MessageType::Notify,
        sequence,
        data,
    };

    let received_message = uart.receive_message().unwrap();
    assert_eq!(received_message, expected_message);
}

#[test]
fn notify_wakeup_enter_receive() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 0;
    let length_bytes = (5 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::Notify.into()));
    transactions.push(MockTransaction::read(Command::NotifyStatus.into()));
    transactions.push(MockTransaction::read(sequence));

    let mut data = vec![NotifyStatus::WakeupEnter.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    for byte in &data {
        transactions.push(MockTransaction::read(*byte as u8));
    }

    let checksum = data.iter().map(|d| u16::from(*d)).sum::<u16>()
        + u8::from(Command::NotifyStatus) as u16
        + sequence as u16
        + u8::from(MessageType::Notify) as u16;
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);

    let expected_message = Message {
        command: Command::NotifyStatus,
        message_type: MessageType::Notify,
        sequence,
        data,
    };

    let received_message = uart.receive_message().unwrap();
    assert_eq!(received_message, expected_message);
}

#[test]
fn notify_wakeup_exit_receive() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 0;
    let length_bytes = (5 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::Notify.into()));
    transactions.push(MockTransaction::read(Command::NotifyStatus.into()));
    transactions.push(MockTransaction::read(sequence));

    let mut data = vec![NotifyStatus::WakeupExit.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    for byte in &data {
        transactions.push(MockTransaction::read(*byte as u8));
    }

    let checksum = data.iter().map(|d| u16::from(*d)).sum::<u16>()
        + u8::from(Command::NotifyStatus) as u16
        + sequence as u16
        + u8::from(MessageType::Notify) as u16;
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);

    let expected_message = Message {
        command: Command::NotifyStatus,
        message_type: MessageType::Notify,
        sequence,
        data,
    };

    let received_message = uart.receive_message().unwrap();
    assert_eq!(received_message, expected_message);
}

#[test]
fn notify_play_start_receive() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 0;
    let length_bytes = (5 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::Notify.into()));
    transactions.push(MockTransaction::read(Command::NotifyStatus.into()));
    transactions.push(MockTransaction::read(sequence));

    let mut data = vec![NotifyStatus::PlayStart.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    for byte in &data {
        transactions.push(MockTransaction::read(*byte as u8));
    }

    let checksum = data.iter().map(|d| u16::from(*d)).sum::<u16>()
        + u8::from(Command::NotifyStatus) as u16
        + sequence as u16
        + u8::from(MessageType::Notify) as u16;
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);

    let expected_message = Message {
        command: Command::NotifyStatus,
        message_type: MessageType::Notify,
        sequence,
        data,
    };

    let received_message = uart.receive_message().unwrap();
    assert_eq!(received_message, expected_message);
}

#[test]
fn notify_play_end_receive() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 0;
    let length_bytes = (5 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::Notify.into()));
    transactions.push(MockTransaction::read(Command::NotifyStatus.into()));
    transactions.push(MockTransaction::read(sequence));

    let mut data = vec![NotifyStatus::PlayEnd.into()];
    data.append((0 as u32).to_le_bytes().to_vec().as_mut());
    assert_eq!(data.len(), 5);
    for byte in &data {
        transactions.push(MockTransaction::read(*byte as u8));
    }

    let checksum = data.iter().map(|d| u16::from(*d)).sum::<u16>()
        + u8::from(Command::NotifyStatus) as u16
        + sequence as u16
        + u8::from(MessageType::Notify) as u16;
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);

    let expected_message = Message {
        command: Command::NotifyStatus,
        message_type: MessageType::Notify,
        sequence,
        data,
    };

    let received_message = uart.receive_message().unwrap();
    assert_eq!(received_message, expected_message);
}
