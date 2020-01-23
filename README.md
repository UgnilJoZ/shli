[![Crates.io](https://img.shields.io/crates/v/shli.svg)](https://crates.io/crates/shli)
[![Build Status](https://travis-ci.org/UgnilJoZ/shli.svg?branch=master)](https://travis-ci.org/UgnilJoZ/shli)
[![Documentation](https://docs.rs/shli/badge.svg)](https://docs.rs/crate/shli/)
[![dependency status](https://deps.rs/crate/shli/0.3.0/status.svg)](https://deps.rs/crate/shli/0.3.0)

# shli
Rust crate for shell-like TUIs

## Purpose
If you once saw nslookup, glusterfs or shelldap and admired their shell-like terminal interfaces, this might be a crate for you.

This crate provides basic building blocks for providing users of your software such an interface.

## Example usage
See `examples/simple.rs`.

A `cargo run --example simple` will run it.

You will see a prompt. Type p, and then press TAB.
```
> p
```
It will autocomplete to `print`, an example command! Now issue this:

```
> print Hello
```
It will print "Hello", which is not spectacular. If you now press the up key, you will be able to edit your last command.

With the left and right keys, the user is able to edit the current commandline.

## Documentation
https://docs.rs/shli/