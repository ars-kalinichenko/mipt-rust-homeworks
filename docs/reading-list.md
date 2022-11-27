# Additional reading list

This document contains additional reading links and videos. It's not necessary to read or watch them, but it may help you understand Rust deeply.

## "Rustbooks"

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Rustbook, the first book to be read by newbie.
- [The Rust Reference](https://doc.rust-lang.org/stable/reference/) - The Rust language reference. Something like explained docs.
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) - Just writing multiple linked lists. It may help to understand the basics of ownership.
- [The Cargo Book](https://doc.rust-lang.org/cargo/) - The Cargo package manager reference.
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html) - Explains how to solve simple, widespread programming tasks.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html) - Official set of recommendations on how to design and present APIs.
- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html) - Starter book about Rust Macros.
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html) - Good practices of Rust programming.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) - The book about Unsafe Rust. In addition, it covers some advanced safety topics such as an ownership.
- [Rust's Unsafe Code Guidelines Reference (UCG WG)](https://rust-lang.github.io/unsafe-code-guidelines/introduction.html)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html) - The book about optimizing Rust code.
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) - Introduction to asynchronous programming.
- [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained/introduction.html) - Good book on futures and executors.
- [Guide to Rustc Development](https://rustc-dev-guide.rust-lang.org) - The guide to compiler development for enthusiasts.
- [What is rustc?](https://doc.rust-lang.org/rustc/what-is-rustc.html) - Describes some advanced usages of the `rustc` compiler.
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/) - Programming for embedded systems in Rust.

## Articles

- [Improved portability and performance](https://pngquant.org/rust.html) - How [libimagequant](https://github.com/ImageOptim/libimagequant) library became more performant and **portable (!)** by rewriting C code to Rust.
- [Rust Collections Case Study: BTreeMap](https://cglab.ca/~abeinges/blah/rust-btree-case/) - Great post about the implementation of BTree in Rust from its author.
- [The Error Model](http://joeduffyblog.com/2016/02/07/the-error-model/) - Article with a perfect, step-by-step explanation of error handling models, including Rust's one, and is good and how it solves mistakes of other generations of error handling.
- [Finding Closure in Rust](https://huonw.github.io/blog/2015/05/finding-closure-in-rust/) - Detailed post with "straightforward" explanation of closures design.
- [How to organize your Rust tests](https://blog.logrocket.com/how-to-organize-your-rust-tests/) - Answers the question "How to organize your Rust tests" :)

## Writing idiomatic Rust code

- [Writing Idiomatic Libraries in Rust](https://www.youtube.com/watch?v=0zOg8_B71gE) - Good talk on writing idiomatic code.

## Conference talks about Rust

- [The History of Rust](https://www.youtube.com/watch?v=79PSagCD_AY) - Just a bit of History of Rust.
- [How Rust Views Tradeoffs](https://www.youtube.com/watch?v=2ajos-0OWts) - Good talk about the aims of Rust language.
- [RustConf 2021 - Move Constructors: Is it Possible? by Miguel Young de la Sota](https://www.youtube.com/watch?v=UrDhMWISR3w)
- [Rust's Journey to Async/Await](https://www.youtube.com/watch?v=lJ3NC-R3gSI) - The talk about the high-level design of async in Rust.
- [The Talk You've Been Await-ing for](https://www.youtube.com/watch?v=NNwK5ZPAJCk) - Introduction to async in Rust.
- [Rust Programming Techniques](https://www.youtube.com/watch?v=vqavdUGKeb4) - Good practices of writing idiomatic code.

## YouTube channels

- [Jon Gjengset YouTube channel](https://www.youtube.com/c/JonGjengset/featured) - Excellent channel about Rust, especially "Crust of Rust" series, where several topics covered from intermediate to advanced level.
- [Aleksey Kladov YouTube channel](https://www.youtube.com/channel/UCLd3PQ6J0C-VuNBozsXGUWg/featured) - If you're interested in how `rust-analyzer` works - it's the best channel and the best speaker.

## Research papers

- [GhostCell: Separating Permissions from Data in Rust](http://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf) - Tricky usage of Rust type system.
- [Stacked Borrows: An Aliasing Model for Rust](https://plv.mpi-sws.org/rustbelt/stacked-borrows/paper.pdf) - On checking `unsafe` code.

## Books

- ["Rust for Rustaceans - Idiomatic Programming for Experienced Developers" by Jon Gjengset](https://nostarch.com/rust-rustaceans) - Perfect book that covers **a lot** of details about the language in-depth.
- [Zero To Production In Rust](https://www.zero2prod.com/index.html?country=Netherlands&discount_code=VAT20)

## Blogs

- [Alastair Reid blog](https://alastairreid.github.io) - You can find great posts about automatic verification tools in this blog.
- [Aleksey Kladov blog](https://matklad.github.io) - Some random things about Rust.
- [Waffle blog](https://ihatereality.space) - Some random things about Rust.
- [Lloyd Chan blog](https://beachape.com) - The author usually writes on advanced usages of the type system.
