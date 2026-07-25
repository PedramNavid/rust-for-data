# Prerequisites

## Installing Rust and Python

You will need Rust and Python installed to follow along with the examples here.

For Rust, go to [rustup.rs](https://rustup.rs/). This gives you `rustc` and
`cargo`, which is all we need.

For Python, I recommend [uv](https://docs.astral.sh/uv/). It replaces the pile
of tools I used to reach for — `pyenv` for interpreters, `virtualenv` for
environments, `pip` for packages — with a single one, and it will install the
right Python for you if you don't have it.

```bash
# macOS / Linux
curl -LsSf https://astral.sh/uv/install.sh | sh
```

A typical project looks like this:

```bash
# Create a project
uv init somepyproj
cd somepyproj

# Pin the Python version for this project
uv python pin 3.13

# Add a dependency
uv add polars

# Run something inside the project environment
uv run python -m somepyproj.main
```

`uv` creates the virtual environment for you the first time you need one, so
there is no separate "activate the venv" step. `uv add` writes the dependency
into `pyproject.toml` and records the exact resolved version in `uv.lock`,
which is the Python equivalent of Rust's `Cargo.lock`.

## A note on versions

Both sample projects in this book pin their dependencies exactly and commit
their lockfiles, so the code you build should behave the same as the code the
benchmarks were run against. At the time of writing that means:

| | Version |
|:---|:---|
| Rust | 1.97.0 |
| Python | 3.13 |
| polars (Rust) | 0.54.4 |
| polars (Python) | 1.43.0 |
| pandas | 3.0.5 |

Both of these ecosystems move quickly, and `polars` in particular has changed
its API substantially over the years. If you are reading this well after it was
written, expect some drift.

## Installing the Code

The code for this book can be found here: `https://github.com/PedramNavid/rust-for-data`

```bash
git clone git@github.com:PedramNavid/rust-for-data.git
```

The Rust examples live in `wxrs`, and the Python examples live in `wxpy`.
