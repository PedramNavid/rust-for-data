## Rust For Data

This book is available for free online at [rustfordata.com](https://rustfordata.com)
You can find the source code for book in `./rust4data-book`
This book is very early, so expect code to change quite a bit.

Feel free to open an issue with any bugs, mistakes, or requests you may have.

<!-- ANCHOR: book_overview -->
## What is this book about?

This series of posts is about using Rust for data engineering tasks for people
who are already familiar with Python and are curious about Rust. It will not
cover every aspect of Rust, or of Python. Instead, it aims to give practical
examples of how common engineering tasks done in Python might be done in Rust,
along with representative benchmarks.

The current chapters cover:

- Getting data from an API
- Parsing data and using structs
- Transforming data with Polars

Concurrency is a work in progress. Writing data and web scraping are planned
topics.

This book is not an introduction to either Rust or Python. There are many
great resources to both out there. If you are not familiar with Python,
the official [Python Tutorial](https://docs.python.org/3/tutorial/) is a great
starting point.

As for Rust, the [Rust Book](https://doc.rust-lang.org/book/) is a great
introduction to the language and a must read. I can also recommend
[Rust in Action](https://www.manning.com/books/rust-in-action) as well.

In particular, I think it's important to understand some of the core principles
behind static and dynamic typing, as well as memory safety and ownership. The
borrow-checker in Rust is well-known as a steep hurdle to climb, but once you
manage to understand it, you start writing better code. Don't be discouraged,
it takes time and I am still on the learning journey with you.
<!-- ANCHOR_END: book_overview -->
