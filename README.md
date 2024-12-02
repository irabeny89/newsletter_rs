# Newsletter Server

A server that controls newsletter service of sending emails to subscribers.

## Development

This project uses [Trunk based development](https://cloud.google.com/solutions/devops/devops-tech-trunk-based-development). In trunk-based development we should be able to deploy our main branch at any point in time.

To speed up the linking phase (compile and run) you have to install the alternative linker on your machine and add this
configuration file to the project:

```yml
# .cargo/config.toml
# On Windows
# ```
# cargo install -f cargo-binutils
# rustup component add llvm-tools-preview
# ```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
# - Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
# On MacOS, `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

We can also mitigate the impact on our productivity by reducing the perceived compilation time -
i.e. the time you spend looking at your terminal waiting for cargo check or cargo run to complete.

Tooling can help here - let’s install cargo-watch:

```bash
cargo install cargo-watch
```

cargo-watch monitors your source code to trigger commands every time a file changes.
For example:

```bash
# this will run cargo check after every code change.
cargo watch -x check
```

cargo-watch supports command chaining as well:

```bash
# 1. It will start by running cargo check.
# 2. If it succeeds, it launches cargo test.
# 3. If tests pass, it launches the application with cargo run.
cargo watch -x check -x test -x run
```

For code coverage, let's install `cargo-tarpaulin`:
```bash
cargo install cargo-tarpaulin
```

For example:
```bash
# compute code coverage and ignore test functions
cargo tarpaulin --ignore-tests
```
