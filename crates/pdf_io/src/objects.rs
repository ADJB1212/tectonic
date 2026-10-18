// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

//! Document-owned PDF values. Edges are IDs and never own another node.
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ObjectId(pub(crate) usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ObjectNumber(pub(crate) u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FileId(pub(crate) usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StreamId(pub(crate) usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResourceId(pub(crate) u32);

impl ResourceId {
    pub(crate) fn new(category: usize, index: usize) -> Option<Self> {
        (category < 9 && index <= u16::MAX as usize)
            .then_some(Self(((category as u32) << 16) | index as u32))
    }

    pub(crate) fn category(self) -> usize {
        (self.0 >> 16) as usize
    }
    pub(crate) fn index(self) -> usize {
        (self.0 & 0xffff) as usize
    }
}

pub(crate) struct Resource {
    pub(crate) object: Option<ObjectId>,
    pub(crate) reference: Option<ObjectId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct IndirectReference {
    pub(crate) source: Option<FileId>,
    pub(crate) target: Option<ObjectId>,
    pub(crate) number: ObjectNumber,
    pub(crate) generation: u16,
}

pub(crate) struct Boolean {
    pub(crate) value: core::ffi::c_char,
}
pub(crate) struct Number {
    pub(crate) value: f64,
}

pub(crate) struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    pub(crate) fn new(bytes: &[u8]) -> Self {
        let mut owned = Vec::with_capacity(bytes.len() + 1);
        owned.extend_from_slice(bytes);
        owned.push(0);
        Self { bytes: owned }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.bytes.len() - 1]
    }
    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.bytes.as_mut_ptr()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct DictionaryEntry {
    pub(crate) key: ObjectId,
    pub(crate) value: Option<ObjectId>,
}

#[derive(Clone, Copy)]
pub(crate) struct Stream {
    pub(crate) dictionary: ObjectId,
    pub(crate) storage: StreamId,
}

pub(crate) enum Value {
    Boolean(Boolean),
    Number(Number),
    String(Bytes),
    Name(Bytes),
    Array(Vec<Option<ObjectId>>),
    Dictionary(Vec<DictionaryEntry>),
    Stream(Stream),
    Null,
    Indirect(IndirectReference),
    Undefined,
    Retired,
}

pub(crate) struct Object {
    pub(crate) id: ObjectId,
    pub(crate) label: u32,
    pub(crate) generation: u16,
    pub(crate) flags: i32,
    pub(crate) value: Value,
}

impl Object {
    pub(crate) fn kind(&self) -> i32 {
        match self.value {
            Value::Boolean(_) => 1,
            Value::Number(_) => 2,
            Value::String(_) => 3,
            Value::Name(_) => 4,
            Value::Array(_) => 5,
            Value::Dictionary(_) => 6,
            Value::Stream(_) => 7,
            Value::Null => 8,
            Value::Indirect(_) => 9,
            Value::Undefined => 10,
            Value::Retired => -1,
        }
    }
}
