# Project 1: `ripgzip`

In this project, you'll implement a simple `gzip` files decompressor.

_Note_: the project name is just a reference to [`ripgrep`](https://github.com/BurntSushi/ripgrep), blazingly fast `grep` implementation in pure Rust.

## Specification

The specification of `gzip` and `deflate` formats can be found in the following RFCs:

- [RFC1951 DEFLATE Compressed Data Format Specification](https://datatracker.ietf.org/doc/html/rfc1951)
- [RFC1952 GZIP file format specification](https://datatracker.ietf.org/doc/html/rfc1952)

## Implementation

Some abstractions were already designed for your convenience. It's suggested to implement them in order:

1. `BitReader` - reads the stream byte by byte. To run unit tests, use `cargo test bit_reader`.
2. `TrackingWriter` - a writer with a 32Kb buffer that tracks the count of written bytes and CRC32 control sum. To run unit tests, use `cargo test tracking_writer`.
3. `HuffmanCoding` - Huffman algorithm token decoder. To run unit tests, use `cargo test huffman_coding`. Generic over token type:
   - `TreeCodeToken` - encodes lengths of Huffman codes.
   - `LitLenToken` - encodes the literal or the end of the block.
   - `DistanceToken` - encodes distance.
4. `GzipReader` - reads header and footer of `gzip` format.
5. `DeflateReader` - reades the header of `deflate` format.
6. The actual `decompress` function.

After implementing, also run `./test.py` or `rover test` since this problem has additional tests.

## I don't like how everything is designed

The only things you cannot change are:

- `decompress` function in the file `lib.rs`: is must accept the input and write to the output, since it's tested.
- `main.rs` file is already implemented for you, but if you want to change it just make sure the binary accepts the file by `stdin` and outputs the compressed result to `stdout`.

You can change other details whatever you like, create new `.rs` files, delete old ones, create directories inside, and so on.

## Error handling

The `anyhow` crate is used for error handling. Don't forget to use `?`, `.context()`, `.with_context()` and `bail!`.

The tests verify that errors have specific substring for some cases in this cases:

- The number of bytes in the `gzip` footer is not as expected: "length check failed".
- The CRC32 is not equal to the one on the `gzip` footer: "crc32 check failed".
- Wrong values of the first two bytes in the `gzip` header: "wrong id values".
- The CRC16 is not equal to the one on the `gzip` header: "header crc16 check failed".
- Unknown compression method in `gzip` header: "unsupported compression method".
- Unknown block type in `deflate` header: "unsupported block type".
- In block `BTYPE = 00` the `LEN == !NLEN` is violated: "nlen check failed".

## Tips

- For logging, use the `log` crate. The most important macros from it are `error!`, `warn!`, `info!`, `debug!` and `trace!`. Only errors and warnings are logged by default. Use keys `-v`, `-vv`, and `-vvv` to log more levels.
- For convenient bytes reading, use trait `ReadBytesExt` from `byteorder` crate. You'll defenitely need `.read_u8()`, `.read_u32::<LittleEndian>()` and may be some more functions.
- To calculate CRC, use `crc` crate. You'll need `Crc<u32>` and `CRC_32_ISO_HDLC` type of algorithm.
