# [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)

Studies from the official rust book.

## Installation

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Rustup metadata and toolchains will be installed into the Rustup
`home` directory, located at:

```bash
  /home/jaime/.rustup
```

This can be modified with the `RUSTUP_HOME` environment variable.

The Cargo home directory is located at:

```bash
  /home/jaime/.cargo
```

This can be modified with the `CARGO_HOME` environment variable.

The `cargo`, `rustc`, `rustup` and other commands will be added to
Cargo's `bin` directory, located at:

```bash
  /home/jaime/.cargo/bin
```

This path will then be added to your `PATH` environment variable by
modifying the profile files located at:

```bash
  /home/jaime/.profile
  /home/jaime/.bashrc
```

You can uninstall at any time with `rustup self uninstall` and
these changes will be reverted.

To get started you may need to restart your current shell.
This would reload your `PATH` environment variable to include
Cargo's `bin` directory (`$HOME/.cargo/bin`).

To configure your current shell, you need to source
the corresponding `env` file under `$HOME/.cargo`.

This is usually done by running one of the following (note the leading DOT):

```bash
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
```