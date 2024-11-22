use std::collections::{HashMap, HashSet};

use bytes::BytesMut;

pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

impl RespDecode for BytesMut {
    fn decode(_buf: Self) -> Result<RespFrame, String> {
        todo!()
    }
}

pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(Option<Vec<u8>>),
    NullBulkString(RespNullBulkString),
    Null(RespNull),
    NullArray(RespNullArray),
    Array(Vec<RespFrame>),
    Boolean(bool),
    Double(f64),
    Map(HashMap<String, RespFrame>),
    Set(HashSet<RespFrame>),
}

pub struct SimpleString(String);

impl SimpleString {
    pub fn value(&self) -> &str {
        &self.0
    }
}
pub struct SimpleError(String);

impl SimpleError {
    pub fn value(&self) -> &str {
        &self.0
    }
}
pub struct RespNull;
pub struct RespNullArray;
pub struct RespNullBulkString;
