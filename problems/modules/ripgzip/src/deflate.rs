#![forbid(unsafe_code)]

use std::{
    convert::TryFrom,
    io::{BufRead, Write},
};

use anyhow::{anyhow, ensure, Context, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::bit_reader::BitReader;
use crate::huffman_coding::{self, LitLenToken};
use crate::tracking_writer::TrackingWriter;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct BlockHeader {
    pub is_final: bool,
    pub compression_type: CompressionType,
}

#[derive(Debug)]
pub enum CompressionType {
    Uncompressed = 0,
    FixedTree = 1,
    DynamicTree = 2,
    Reserved = 3,
}

////////////////////////////////////////////////////////////////////////////////

pub struct DeflateReader<T> {
    bit_reader: BitReader<T>,
    // TODO: your code goes here.
}

impl<T: BufRead> DeflateReader<T> {
    pub fn new(bit_reader: BitReader<T>) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn next_block(&mut self) -> Option<Result<(BlockHeader, &mut BitReader<T>)>> {
        // TODO: your code goes here.
        unimplemented!()
    }
}

// TODO: your code goes here.
