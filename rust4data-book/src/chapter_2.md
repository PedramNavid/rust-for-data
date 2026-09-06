# Prerequisites

## Installing Rust and Python

You will need Rust and Python installed to follow along with the examples here.

For Rust, go to [rustup.rs](https://rustup.rs/). This gives you `rustc` and
`cargo`. Follow the platform-specific instructions there for a linker and
other build prerequisites, too.

For Python, I recommend [uv](https://docs.astral.sh/uv/). It replaces the pile
of tools I used to reach for — `pyenv` for interpreters, `virtualenv` for
environments, `pip` for packages — with a single one, and it will install the
right Python for you if you don't have it.

```bash
# macOS / Linux
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Open a new terminal after installation if the commands are not found. Check
that the tools are available:

```bash
rustc --version
cargo --version
uv --version
```

For Windows installation instructions, see the
[uv installation guide](https://docs.astral.sh/uv/getting-started/installation/).

A small practice project looks like this. Create it outside the book's
repository; the book's sample projects are already set up.

```bash
# Create a project
uv init --no-package --python 3.13 somepyproj
cd somepyproj

# Pin the Python version for this project
uv python pin 3.13

# Add a dependency
uv add polars

# Run the generated script inside the project environment
uv run python main.py
```

You should see `Hello from somepyproj!`. The `--no-package` flag explicitly
selects a simple script layout with `main.py`; it avoids relying on uv's
default project layout. Choosing Python 3.13 at creation also keeps the
project's Python requirement compatible with the version we pin afterward.

`uv` creates the virtual environment when a command such as `uv add` or
`uv run` needs it, so
there is no separate "activate the venv" step. `uv add` writes the dependency
into `pyproject.toml` and records the exact resolved version in `uv.lock`,
which is the Python equivalent of Rust's `Cargo.lock`.

This practice project resolves the currently available Polars release. To use
the book's dependency versions, use the checked-in projects below instead.

## A note on versions

Both sample projects pin their direct dependencies exactly and commit their
lockfiles to record resolved dependency versions. The versions listed for the
examples are:

| | Version |
|:---|:---|
| Rust | 1.97.0 |
| Python | 3.13 |
| polars (Rust) | 0.55.2 |
| polars (Python) | 1.44.1 |
| pandas | 3.0.5 |

Both of these ecosystems move quickly, and `polars` in particular has changed
its API substantially over the years. Use the committed lockfiles rather than
recreating the dependencies with `uv add` or `cargo add`.

The Python project selects the 3.13 series in `.python-version`, rather than
an exact patch release. The repository does not pin the Rust compiler with a
`rust-toolchain.toml` file. Lockfiles do not pin your compiler, operating
system, hardware, or build flags, and do not guarantee identical benchmark
times. Record those details when comparing results.

## Installing the Code

The code for this book is on [GitHub](https://github.com/PedramNavid/rust-for-data).
You will need Git installed. From the directory where you keep your projects:

```bash
git clone https://github.com/PedramNavid/rust-for-data.git
cd rust-for-data

# Install the Python dependencies from the committed lockfile
uv sync --locked --project wxpy

# Check that the Rust examples compile, using the committed lockfile
cargo check --locked --manifest-path wxrs/Cargo.toml --bins
```

The Rust examples live in `wxrs`, and the Python examples live in `wxpy`.
The first Rust check can take a while because it compiles dependencies.
`cargo check` checks the code without producing runnable binaries; later
chapters cover running examples and building release binaries for benchmarks.

`--locked` makes these commands fail if the lockfile needs updating, rather
than silently changing the dependency resolution. Keep the lockfile; any
dependency upgrade needs the examples and benchmarks to be checked again.

The repository's convenience commands also require `make`. From the repo
root, `make setup` performs the same locked Python installation. You do not
need an API key or the extracted bird dataset for these setup checks; those
requirements are introduced with the examples that use them.
