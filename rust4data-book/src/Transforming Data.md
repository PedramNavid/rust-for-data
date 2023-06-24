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
{{#include ../../wxpy/wxpy/ch5/birds.py}}
```
