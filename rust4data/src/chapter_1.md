# Chapter 1 - Introduction


## What is this book about?

This series of posts is about using Rust for data engineering tasks for people
who are already familiar with Python and are curious about Rust. It will not
cover every aspect of Rust, or of Python. Instead, it aims to give practical
examples of how common engineering tasks done in Python might be done in Rust,
along with representative benchmarks.

We will cover a variety of topics, including:
- Getting data from an API
- Scraping a website
- Parsing data and using structs
- Data Transformation
- Writing Data
- Other topics as I feel like it

This book is not an introduction to either Rust or Python. There are many
great resources to both out there. If you are not familiar with Python,
the official [Python Tutorial](https://docs.python.org/3/tutorial/) is a great
starting point.

As for Rust, the [Rust Book](https://doc.rust-lang.org/book/) is a great
introduction to the language and a must read. I can also recommend
[Rust in Action](https://www.manning.com/books/rust-in-action) as well.

In particular, I think it's important to understand some of the core principals
behind static and dynamic typing, as well as memory safety and ownership. The
borrow-checker in Rust is well-known as a steep hurdle to climb, but once you
manage to understand it, you start writing better code. Don't be discouraged,
it takes time and I am still on the learning journey with you.


## Should I use Rust for Data Engineering?

Probably not. Rust is a great language, it is fun, it is pleasant to use, and
it is fast. But choosing a language for a project is more than choosing a
language that is fun. There are [cautionary tales](https://mdwdotla.medium.com/using-rust-at-a-startup-a-cautionary-tale-42ab823d9454)
about using Rust at a startup, and I think they are worth reading.

There are many reasons why you might not want to use Rust for data engineering.
The first is that Rust is not as mature as Python. There are many libraries
that are missing. For example, as of this writing, there are no Snowflake
libraries for querying data in that warehouse. Most people do not know Rust,
and it is harder to hire and harder to train people.

There may be good reasons to use Rust for data engineering however. When it
comes to cost and performance, Rust is clearly faster than Python for many
types of tasks. Memory usage is also much lower, which can be important when
you are constrained by small IoT devices, for example.

I can't tell you when to use Rust and when to use Python, but I do believe that
by understanding both languages, their merits and pitfalls, you will be
better positioned to make that decision for yourself.

## Why Should I Learn Rust?

Because it is fun to learn new things. I can't promise you that anything
you learn here will ever have a material impact on your life or career. But
if you enjoy learning and tinkering, then you might want to tinker with this.
If you are like me, and you like learning for learning's sake, then you will
enjoy this experience too. I learned vim and lua not because it was useful,
but because I was curious about it. I did end up benfiting from it, but I never
approached it from a purely utilitarian perspective. There are better ways to
spend your time if your goal is purely career advancement.

But, if you are curious about Rust, and if you like to have fun, then I think
you will be pleasantly surprised by what Rust has to offer.




