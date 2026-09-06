# Chapter 1 - Introduction

{{#include ../../README.md:book_overview}}


## Should I use Rust for Data Engineering?

Probably not. Rust is a great language, it is fun, it is pleasant to use, and
it is fast. But choosing a language for a project is more than choosing a
language that is fun. There are [cautionary tales](https://mdwdotla.medium.com/using-rust-at-a-startup-a-cautionary-tale-42ab823d9454)
about using Rust at a startup, and I think they are worth reading.

There are many reasons why you might not want to use Rust for data engineering.
Start with the libraries and integrations your project needs: check whether
they support the features you use, how they are maintained, and what support
is available. An earlier version of this chapter said there were no Rust
libraries for querying Snowflake; that is no longer true, as projects such as
[snowflake-connector-rs](https://docs.rs/snowflake-connector-rs/) demonstrate.
If your team already knows Python, learning Rust also adds time to development
and onboarding.

There may be good reasons to use Rust for data engineering, however. Rust can
reduce runtime and memory use for some workloads, particularly when replacing
work done in Python loops. But a Python program may already do most of its
work inside a native library such as Polars, or spend most of its time waiting
on a network. Changing the language alone does not guarantee an improvement.

The examples in this book compare particular implementations on particular
workloads. We will look at what each benchmark includes, whether the programs
do equivalent work, and where the time goes. Treat the results as measurements
to investigate and reproduce, rather than promises about your own pipeline.

I can't tell you when to use Rust and when to use Python, but I do believe that
by understanding both languages, their merits and pitfalls, you will be
better positioned to make that decision for yourself.

## Why Should I Learn Rust?

Because it is fun to learn new things. I can't promise you that anything
you learn here will ever have a material impact on your life or career. But
if you enjoy learning and tinkering, then you might want to tinker with this.
If you are like me, and you like learning for learning's sake, then you will
enjoy this experience too. I learned vim and lua not because it was useful,
but because I was curious about it. I did end up benefiting from it, but I never
approached it from a purely utilitarian perspective. There are better ways to
spend your time if your goal is purely career advancement.

But, if you are curious about Rust, and if you like to have fun, then I think
you will be pleasantly surprised by what Rust has to offer.
