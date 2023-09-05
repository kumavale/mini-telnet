# mini-telnet

[![Actions Status](https://github.com/kumavale/mini-telnet/workflows/Rust/badge.svg)](https://github.com/kumavale/mini-telnet/actions)
[![license](https://img.shields.io/badge/License-MIT-blue.svg?style=flat)](LICENSE-MIT)
[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat)](LICENSE-APACHE)

Telnetクライアントの実装

## 機能

- WINDOW_SIZE (固定サイズ)
- SUPPRESS_GO_AHEAD

## 使い方

```
$ cargo run -- --help
Usage: mini-telnet <HOSTNAME>

Arguments:
  <HOSTNAME>  hostname:port

Options:
  -h, --help     Print help
  -V, --version  Print version
```
