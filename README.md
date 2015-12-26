# JSON Pretty Print
> Yet another json pretty printer

## Installation

```bash
$ git clone git@github.com:jonmorehouse/rust-jsonpp
$ cd rust-jsonpp
$ cargo build
```

## Basic Usage

```bash
$ echo '{"key": "value"}' | ./target/debug/jsonpp
```

```bash
{
  "key": "value"
}
```

## Multiple Lines

Some archive files embed many different json objects as single line, json strings. To prettify such json files, simple call `jsonpp` with the `-l` argument.

```bash
$ echo '{"key": "value"}\n{"key":"value"}' | ./target/debug/jsonpp -l
```
```bash
{
  "key": "value"
}
{
  "key": "value"
}
```
