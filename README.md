# Jmks
**J**i**m**aku**k**en**s**aku - 字幕検索 - Subtitle search

Command line tool to search a directory of subtitle files for words/grammars/etc. Only works for SSA V4+ subtiltes (.ass files)

This is my second Rust program! It's another rewrite of an old Bash script I made that does the same thing.


# Dependencies 
**Rust**: built with version "1.70.0", but older versions likely work fine.

# Building
Run: `cargo build --release`

# Usage

```
jmks [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  Pattern to search

Options:
  -s, --subdir <SUBDIR>            Set the subtitle directory
  -d, --depth <DEPTH>              Set max search depth
  -i, --ignore <NEGATIVE PATTERN>  Ignore lines that contain this pattern
  -C, --context <N LINES>          Lines of context before & after match
  -B, --before <N LINES>           Lines of context before match
  -A, --after <N LINES>            Lines of context after match
  -h, --help                       Print help

```
# Configuration
 You can avoid specifying the `--subdir` and `--depth` by creating a config.toml at `$XDG_CONFIG_HOME/jmks/config.toml`. If `XDG_CONFIG_HOME` is not set, then use `$HOME/.config/jmks/config.toml`.

Here's an example of a config.toml file
```toml
subdir = "/path/to/download/desired/location"
depth = 2 #Optional 
```

:warning: **WARNING:** Must use absolute paths, environment variables will not be parsed and tilde `~` home will not work either.
