<div align="center">
  <h1>Toor</h1>

<img src='docs/logo.svg' width=80px />

Find project root.

<a href="https://github.com/azzamsa/toor/actions/workflows/ci.yml">
    <img src="https://github.com/azzamsa/toor/actions/workflows/ci.yml/badge.svg" alt="Build status" />
  </a>

<a href="https://crates.io/crates/toor">
    <img src="https://img.shields.io/crates/v/toor.svg">
  </a>

<a href=" https://docs.rs/toor/">
    <img src="https://docs.rs/toor/badge.svg">
  </a>

<a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

<p><p/>

</div>

---

## Features

- Fancy error message and colorful output.
- Cross-platform and single binary.

## Usage

```bash
🦄 toor --help

🦄 toor
/home/user/playground/rust

~/playground
🦄 toor
Error: toor::no_project_root (link)

  × Project root is not found.
  help: Make sure the project root exists.
```

## Installation

### From binaries

The [release page](https://github.com/azzamsa/toor/releases) includes
pre-compiled binaries for GNU/Linux, macOS, and Windows.

### From source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
$ cargo binstall toor
```

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
$ cargo install toor
```

## Development

```bash
git clone https://github.com/azzamsa/toor

# Build
cd toor
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read [the development guide](docs/dev/README.md)

## Origin of the name

The term 'toor' is a whimsical variation of 'root'."

## Credits

- [bbatsov's `projectile-project-root`](https://github.com/bbatsov/projectile)
- [Noto Emoji](https://github.com/googlefonts/noto-emoji)
