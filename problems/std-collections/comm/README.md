# Comm utility

## Task

Implement the command line utility called `comm`. It takes two files as an input parameter and outputs all common lines, i.e., lines that appear in both files. Please note _every unique line should be printed once_. Output order doesn't matter.

## Implementation tips

- To get a command line arguments, use [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html):

    ```rust
    let args = std::env::args().collect::<Vec<String>>();
    ```

- To read file line by line, use [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html) and [`std::io::BufReader`](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html):

    ```rust
    use std::{fs::File, io::BufRead, io::BufReader};

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // ...
    }
    ```

- To intersect the lines, you can use [HashSet](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html). You'll need `insert`, `contains` and `take` functions.
