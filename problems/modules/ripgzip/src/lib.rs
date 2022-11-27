#![forbid(unsafe_code)]

use std::io::{BufRead, Write};

use anyhow::{Context, Result};
use log::*;

use crate::gzip::GzipReader;

mod bit_reader;
mod deflate;
mod gzip;
mod huffman_coding;
mod tracking_writer;

pub fn decompress<R: BufRead, W: Write>(input: R, mut output: W) -> Result<()> {
    // TODO: your code goes here.
    unimplemented!()
}
