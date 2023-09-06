use df2301q::*;
use embedded_hal_mock::serial::{Mock, Transaction as MockTransaction};

#[test]
#[should_panic(expected = "called serial::read with no expectation")]
fn test_incomplete_message_error() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 0;
    let length_bytes = (1 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::CommandUp.into()));
    transactions.push(MockTransaction::read(Command::GetCommandId.into()));
    transactions.push(MockTransaction::read(sequence)); // Sequence

    // Missing data
    // transactions.push(MockTransaction::read(1 as u8));

    let checksum = 1;
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);

    let received_message = uart.receive_message().unwrap_err();
    assert_eq!(received_message, Df2301qUARTError::IncompleteMessage);
}

#[test]
fn get_command_id() {
    let mut transactions = vec![
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_LOW),
        MockTransaction::read(DF2301Q_UART_MESSAGE_HEAD_HIGH),
    ];

    let sequence = 0;
    let length_bytes = (1 as u16).to_le_bytes();
    transactions.push(MockTransaction::read(length_bytes[0]));
    transactions.push(MockTransaction::read(length_bytes[1]));
    transactions.push(MockTransaction::read(MessageType::CommandUp.into()));
    transactions.push(MockTransaction::read(Command::GetCommandId.into()));
    transactions.push(MockTransaction::read(sequence)); // Sequence

    transactions.push(MockTransaction::read(1 as u8));

    let checksum: u16 = 1
        + sequence as u16
        + u8::from(MessageType::CommandUp) as u16
        + u8::from(Command::GetCommandId) as u16;
    transactions.push(MockTransaction::read((checksum & 0xFF) as u8));
    transactions.push(MockTransaction::read(((checksum >> 8) & 0xFF) as u8));
    transactions.push(MockTransaction::read(DF2301Q_UART_MESSAGE_TAIL));

    let mock = Mock::new(&transactions);
    let mut uart = Df2301qUART::new(mock.clone(), mock);

    let expected_message = Message {
        command: Command::GetCommandId,
        message_type: MessageType::CommandUp,
        data: vec![1],
        sequence,
    };

    let received_message = uart.receive_message().unwrap();
    assert_eq!(received_message, expected_message);
}
