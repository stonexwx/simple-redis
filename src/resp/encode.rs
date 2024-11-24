use super::{
    BulkString, FloatWrapper, RespArray, RespEncode, RespMap, RespNull, RespNullArray,
    RespNullBulkString, RespSet, SimpleError, SimpleString,
};

const BUF_CAP: usize = 4096;

// + simple string: "+<value>\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

// - error: "-<value>\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

// - bulk string: "$<length>\r\n<value>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(format!("${}\r\n", self.len()).as_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// - null bulk string: "$-1\r\n"
impl RespEncode for () {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

// - array: "*<number-of-elements>\r\n<elements-1>...<elements-N>"
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(format!("*{}\r\n", self.0.len()).as_bytes());
        for element in self.0 {
            buf.extend(element.encode());
        }
        buf
    }
}

// - null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

// - null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

// - null bulk string: "$-1\r\n"
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}

// - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespEncode for FloatWrapper {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.0.abs() > 1e+8 {
            format!(",{:+e}\r\n", self.0)
        } else {
            let sign = if self.0 < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self.0)
        };
        buf.extend_from_slice(ret.as_bytes());
        buf
    }
}

// - map: "%<number-of-elements>\r\n<key-1><value-1>...<key-N><value-N>"
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        println!("{:?}", self.0);
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(format!("%{}\r\n", self.0.len()).as_bytes());
        for (key, value) in self.0 {
            buf.extend(SimpleString::new(key).encode());
            buf.extend(value.encode());
        }
        buf
    }
}

// - set: "~<number-of-elements>\r\n<element-1>...<element-N>"
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(format!("~{}\r\n", self.0.len()).as_bytes());
        for frame in self.0 {
            buf.extend(frame.encode());
        }
        buf
    }
}

#[cfg(test)]
use super::*;

#[test]
fn test_simple_string_encode() {
    let frame: RespFrame = SimpleString::new("hello world").into();
    assert_eq!(frame.encode(), b"+hello world\r\n");
}

#[test]
fn test_simple_error_encode() {
    let frame: RespFrame = SimpleError::new("hello world").into();
    assert_eq!(frame.encode(), b"-hello world\r\n");
}

#[test]
fn test_integer_encode() {
    let frame: RespFrame = 123.into();
    assert_eq!(frame.encode(), b":+123\r\n");

    let frame: RespFrame = (-123).into();
    assert_eq!(frame.encode(), b":-123\r\n");
}

#[test]
fn test_bulk_string_encode() {
    let frame: RespFrame = BulkString::new("hello world").into();
    assert_eq!(frame.encode(), b"$11\r\nhello world\r\n");
}

#[test]
fn test_null_bulk_string_encode() {
    let frame: RespFrame = RespNullBulkString.into();
    assert_eq!(frame.encode(), b"$-1\r\n");
}

#[test]
fn test_array_encode() {
    let frame: RespFrame = RespArray::new(vec![
        SimpleString::new("hello world").into(),
        123.into(),
        BulkString::new("hello world").into(),
    ])
    .into();
    assert_eq!(
        frame.encode(),
        b"*3\r\n+hello world\r\n:+123\r\n$11\r\nhello world\r\n"
    );
}

#[test]
fn test_null_array_encode() {
    let frame: RespFrame = RespNullArray.into();
    assert_eq!(frame.encode(), b"*-1\r\n");
}

#[test]
fn test_null_encode() {
    let frame: RespFrame = RespNull.into();
    assert_eq!(frame.encode(), b"_\r\n");
}

#[test]
fn test_boolean_encode() {
    let frame: RespFrame = true.into();
    assert_eq!(frame.encode(), b"#t\r\n");

    let frame: RespFrame = false.into();
    assert_eq!(frame.encode(), b"#f\r\n");
}

#[test]
fn test_double_encode() {
    let frame: RespFrame = FloatWrapper(123.456).into();
    assert_eq!(frame.encode(), b",+123.456\r\n");

    let frame: RespFrame = FloatWrapper(-123.456).into();
    assert_eq!(frame.encode(), b",-123.456\r\n");

    let frame: RespFrame = FloatWrapper(123456789.0).into();
    assert_eq!(frame.encode(), b",+1.23456789e8\r\n");
}

#[test]
fn test_map_encode() {
    let mut frame = RespMap::new_map();
    frame.insert("hello".to_string(), SimpleString::new("world").into());
    frame.insert("number".to_string(), 123.into());
    frame.insert("bulk".to_string(), BulkString::new("hello world").into());
    let frame: RespFrame = RespMap(frame).into();
    assert_eq!(
        frame.encode(),
        b"%3\r\n+bulk\r\n$11\r\nhello world\r\n+hello\r\n+world\r\n+number\r\n:+123\r\n"
    );
}

#[test]
fn test_set_encode() {
    let frame: RespFrame = RespSet::new(vec![SimpleString::new("hello").into(), 123.into()]).into();
    assert_eq!(frame.encode(), b"~2\r\n+hello\r\n:+123\r\n");
}

#[test]
fn test_resp_encode() {
    let frame: RespFrame = RespArray::new(vec![
        SimpleString::new("hello world").into(),
        123.into(),
        BulkString::new("hello world").into(),
    ])
    .into();
    assert_eq!(
        frame.encode(),
        b"*3\r\n+hello world\r\n:+123\r\n$11\r\nhello world\r\n"
    );
}

#[test]
fn test_resp_encode_null() {
    let frame: RespFrame = RespNull.into();
    assert_eq!(frame.encode(), b"_\r\n");
}

#[test]
fn test_resp_encode_null_array() {
    let frame: RespFrame = RespNullArray.into();
    assert_eq!(frame.encode(), b"*-1\r\n");
}

#[test]
fn test_resp_encode_null_bulk_string() {
    let frame: RespFrame = RespNullBulkString.into();
    assert_eq!(frame.encode(), b"$-1\r\n");
}

#[test]
fn test_resp_encode_simple_string() {
    let frame: RespFrame = SimpleString::new("hello world").into();
    assert_eq!(frame.encode(), b"+hello world\r\n");
}

#[test]

fn test_resp_encode_simple_error() {
    let frame: RespFrame = SimpleError::new("hello world").into();
    assert_eq!(frame.encode(), b"-hello world\r\n");
}
