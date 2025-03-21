// TODO: закомментируйте эту строчку, когда закончите отладку программы.
#![allow(unused_variables, dead_code)]

use std::convert::TryInto;

// Структура для представления поля Protobuf
#[derive(Debug, PartialEq)]
struct ProtobufField {
    field_number: u32,
    field_type: FieldType,
    value: FieldValue,
}

// Типы полей Protobuf
#[derive(Debug, PartialEq)]
enum FieldType {
    Varint,
    Fixed64,
    LengthDelimited,
    Fixed32,
    // Добавьте другие типы по мере необходимости
}

// Возможные значения полей Protobuf
#[derive(Debug, PartialEq)]
enum FieldValue {
    Varint(u64),
    Fixed64(u64),
    LengthDelimited(Vec<u8>),
    Fixed32(u32),
    // Добавьте другие типы по мере необходимости
}

// Функция для декодирования varint
fn decode_varint(bytes: &[u8]) -> Result<(u64, usize), &'static str> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;
    let mut i = 0;

    for &byte in bytes {
        let value = (byte & 0x7F) as u64; // Get the lower 7 bits
        result |= value << shift;
        shift += 7;

        if byte & 0x80 == 0 { // Check if the MSB is not set (end of varint)
            return Ok((result, i + 1));
        }

        if shift > 63 { // Prevent overflow for the 64-bit varint
            return Err("Varint is too long");
        }

        i += 1;
    }
    Err("Unexpected end of input while decoding varint")
}

// Функция для десериализации одного поля Protobuf
fn deserialize_field(bytes: &[u8]) -> Result<(ProtobufField, usize), &'static str> {
    if bytes.is_empty() {
        return Err("Input is empty");
    }

    let (key, key_len) = decode_varint(bytes)?;
    let tag = key >> 3; // First few bits for field number
    let wire_type = (key & 0x07) as u8; // Last 3 bits for wire type

    let (field_type, value_len) = match wire_type {
        0 => (FieldType::Varint, {
            let (value, len) = decode_varint(&bytes[key_len..])?;
            (FieldValue::Varint(value), len)
        }),
        1 => {
            if bytes.len() < key_len + 8 {
                return Err("Not enough bytes for Fixed64");
            }
            let mut buf: [u8; 8] = [0; 8];
            buf.copy_from_slice(&bytes[key_len..key_len + 8]);
            let value = u64::from_le_bytes(buf);
            (FieldType::Fixed64, (FieldValue::Fixed64(value), 8))
        }
        2 => {
            let (length, len_len) = decode_varint(&bytes[key_len..])?;
            if bytes.len() < key_len + len_len + length as usize {
                return Err("Not enough bytes for LengthDelimited");
            }
            let value = bytes[key_len + len_len..key_len + len_len + length as usize].to_vec();
            (FieldType::LengthDelimited, (FieldValue::LengthDelimited(value), len_len + length as usize))
        }
        5 => {
            if bytes.len() < key_len + 4 {
                return Err("Not enough bytes for Fixed32");
            }
            let mut buf: [u8; 4] = [0; 4];
            buf.copy_from_slice(&bytes[key_len..key_len + 4]);
            let value = u32::from_le_bytes(buf);
            (FieldType::Fixed32, (FieldValue::Fixed32(value), 4))
        }
        _ => return Err("Unsupported wire type"),
    };

    let field = ProtobufField {
        field_number: tag as u32,
        field_type: field_type,
        value: value_len.0,
    };

    Ok((field, key_len + value_len.1))
}

// Функция для десериализации всей структуры Protobuf
fn deserialize_protobuf(bytes: &[u8]) -> Result<Vec<ProtobufField>, &'static str> {
    let mut fields = Vec::new();
    let mut offset = 0;

    while offset < bytes.len() {
        match deserialize_field(&bytes[offset..]) {
            Ok((field, len)) => {
                fields.push(field);
                offset += len;
            }
            Err(e) => {
                // В реальной программе можно обрабатывать ошибки более детально
                eprintln!("Error deserializing field: {}", e);
                break; // Или можно пропустить некорректное поле и продолжить
            }
        }
    }

    Ok(fields)
}

fn main() {
    // Пример Protobuf сообщения (предположим, что это строка с номером поля 1,
    // типом LengthDelimited и значением "hello")
    let message = [
        0x0a, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, // 1: "hello"
    ];

    match deserialize_protobuf(&message) {
        Ok(fields) => {
            println!("Deserialized fields:");
            for field in fields {
                println!("{:?}", field);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
