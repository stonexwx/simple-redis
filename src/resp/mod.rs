mod encode;
use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, LowerExp},
    ops::Deref,
};

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

impl RespDecode for BytesMut {
    fn decode(_buf: Self) -> Result<RespFrame, String> {
        todo!()
    }
}

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Null(RespNull),
    NullArray(RespNullArray),
    Array(RespArray),
    Boolean(bool),
    Double(FloatWrapper),
    Map(RespMap),
    Set(RespSet),
}

#[derive(Debug)]
pub struct FloatWrapper(f64);
impl Deref for FloatWrapper {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl Eq for FloatWrapper {}
impl PartialOrd for FloatWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FloatWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl LowerExp for FloatWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Display for FloatWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleString(String);
impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleError(String);
impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BulkString(Vec<u8>);
impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespArray(Vec<RespFrame>);
impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl RespArray {
    pub fn new(v: Vec<RespFrame>) -> Self {
        Self(v)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNull;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNullArray;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNullBulkString;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespMap(BTreeMap<String, RespFrame>);
impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<BTreeMap<String, RespFrame>> for RespMap {
    fn from(v: BTreeMap<String, RespFrame>) -> Self {
        Self(v)
    }
}
impl RespMap {
    pub fn new_map() -> BTreeMap<String, RespFrame> {
        BTreeMap::new()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespSet(BTreeSet<RespFrame>);
impl Deref for RespSet {
    type Target = BTreeSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl RespSet {
    pub fn new(v: Vec<RespFrame>) -> Self {
        Self(v.into_iter().collect())
    }
}
