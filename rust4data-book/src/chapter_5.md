# Transforming Data using Polars

In this chapter, we'll look at how to transform data using Polars in both
Python and Rust.

Polars is a "blazing fast DataFrame library" available in both Python and
Rust. It is similar to pandas, although has fewer capabilities, however, it
supports a wide-variety of common transformation tasks.

It is several times faster than pandas, and is a good choice for data
transformation tasks.

The [Polars documentation](https://pola-rs.github.io/polars-book/getting-started/intro/)
is a great resource for getting started, and the API docs have even more
details on syntax.

Let's look at some key differences between the syntax in Python and Rust.

## Python

```python
{{#include ../../wxpy/wxpy/ch5/ch5.py}}
```

The Python code very concise, columns can be selected as a list of strings,
the `sort` function takes a simple `descending` argument, and the general
API is very straightforward.

I've also included an attempt at the same logic in pandas. While largely
similar, there are a few differences, for example, in how we filter for
valid results.

```python
{{#include ../../wxpy/wxpy/ch5/ch5_pandas.py}}
```

Now let's compare the above to Rust code.

## Rust

```rust
{{#include ../../wxrs/src/bin/ch5.rs}}
```

In Rust, the code 75% longer and the syntax is more verbose. There are a lot of
`unwrap` calls to handle errors, although some of these could be replaced with
`?` in a real application.

The `sort` function takes a `SortOptions` struct, which is a bit more verbose.
Overall, the API is very similar.

## Benchmarks

Let's look at some benchmarks for polars in both Python and Rust, as well
as similar code in Pandas.

{{#include ../../benchmarks/ch5.md}}

The Rust version is the fastest again. The Python-polars code is 1.6x slower
than the Rust code, but the Pandas code is exceptionally slow, taking over 5 
seconds to complete while both Polars versions take less than 1 second.
