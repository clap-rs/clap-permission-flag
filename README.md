# clap-permission-flag

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Drop permissions of a CLI using structopt.

- [Documentation][8]
- [Crates.io][2]

## Usage

```rust,no_run
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    permission: clap_permission_flag::Permission,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    args.permission.drop()?;
    Ok(())
}
```

This crate depends on Unix features - on other platforms a stub implementation
which has no flags and always succeeds if used.

### Example Output

```txt
clap-permission-flag 0.2.0
Yoshua Wuyts <yoshuawuyts@gmail.com>
Drop permissions of a CLI using structopt

USAGE:
    main [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --chroot <chroot>    Change the process root directory
    -g, --group <group>      Change the process group
    -u, --user <user>        Change the process user
```

## Installation

```sh
$ cargo add clap-permission-flag
```

Since `--chroot` may not be suitable for all applications, it is behind the
`chroot` feature and must be enabled explicitly:

```toml
[dependencies.clap-permission-flag]
version = "0.2"
features = ["chroot"]
```

## See Also

- https://github.com/jedisct1/rust-privdrop

## License

[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/clap-permission-flag.svg?style=flat-square
[2]: https://crates.io/crates/clap-permission-flag
[3]: https://img.shields.io/travis/rust-clique/clap-permission-flag.svg?style=flat-square
[4]: https://travis-ci.org/rust-clique/clap-permission-flag
[5]: https://img.shields.io/crates/d/clap-permission-flag.svg?style=flat-square
[6]: https://crates.io/crates/clap-permission-flag
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/clap-permission-flag
