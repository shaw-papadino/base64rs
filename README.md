# base64rs
The cli tool for Base64.

# usage

## encode

```sh
cargo run e rust
cnVzdA==

or

cargo run rust
cnVzdA==
```
## decode

```sh
cargo run d cnVzdA==
rust
```

## help

```sh
cargo run -h
Name:
	base64rs

Author:


Description:


Usage:
	base64rs [args]

Flags:
	-h, --help : Show help

Commands:
	e, encode : encode
	d, decode : decode

Version:
	0.1.0
```