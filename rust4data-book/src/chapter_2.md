# Quickstart

## Installing Rust and Python

You will need Rust and Python installed to follow along with the examples here.
There are many ways to setup an environment, especially in Python, I've listed
the one I like here.

For Rust, you can install Rust by going to [rustup.rs](https://rustup.rs/).

For Python, I recommend using [pyenv](https://github.com/pyenv/pyenv) along with [pyenv-virtualenv](https://github.com/pyenv/pyenv-virtualenv).
With both installed, here is my typical workflow:

```bash
# Create a folder
mkdir -p ~/projects/somepyproj
cd ~/projects/somepyproj

# Install a recent version of Python
pyenv install 3.10

# Create a virtualenv for the project named proj310
pyenv virtualenv 3.10 proj310
pyenv local proj310

# Confirm pyenv is working
pyenv version
> proj310 (set by /Users/username/projects/somepyproj/.python-version)```
```

## Installing the Code

The code for this book can be found here: `https://github.com/PedramNavid/rust-for-data`

```bash
git clone git@github.com:PedramNavid/rust-for-data.git
```

