# consul-convert

Example project for converting key/value files to single file, which can be imported into consul using `consul kv import`. Key is a filename, value is a file content(usually json).

## Build

```sh
cargo build --release
```

## Usage

Create a file *import.json* from all files in directory */input/path/* with keys path/to/key/$filename:

```sh
consul-convert --src /input/path --dest import.json --key-path path/to/key
```

Import all kys to consul:

```sh
consul kv import @import.json
```
