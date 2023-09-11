# mini-telnet

[![Actions Status](https://github.com/kumavale/mini-telnet/workflows/Rust/badge.svg)](https://github.com/kumavale/mini-telnet/actions)
[![license](https://img.shields.io/badge/License-MIT-blue.svg?style=flat)](LICENSE-MIT)
[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat)](LICENSE-APACHE)

mini telnet client 🐭

## Features

- [ ] ECHO
- [x] SUPPRESS_GO_AHEAD
- [ ] STATUS
- [ ] TERMINAL_TYPE
- [x] WINDOW_SIZE
- [ ] TERMINAL_SPEED
- [ ] REMOTE_FLOW_CONTROL
- [ ] LINE_MODE
- [ ] X_DISPLAY_LOCATION
- [ ] ENVIRONMENT
- [ ] AUTHENTICATION
- [ ] ENCRYPT
- [ ] NEW_ENVIRONMENT

## Usage

```
$ cargo run -- --help
Usage: mini-telnet [OPTIONS] <HOSTNAME>

Arguments:
  <HOSTNAME>  hostname:port

Options:
  -v, --verbose  verbose
  -h, --help     Print help
  -V, --version  Print version
```

