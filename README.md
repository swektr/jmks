# Jmks
**J**i**m**aku**k**en**s**aku - 字幕検索 - Subtitle search

Command line tool to search a directory of subtitle files for words/grammars/etc. 

This is my second Rust program! It's another rewrite of an old Bash script I made that does the same thing.


# Dependencies 
**Rust**: built with version "1.70.0", but older versions likely work fine.

# Building
Run: `cargo build`

# Usage
Run program like this `jmks [OPTIONS] <PATTERN>`

<u>Arguments:</u>
* `PATTERN` (required) Regex pattern.

<u>Options:</u>
* `-s, --subdir` -- Set the subtitle directory. 
* `-d, --depth` -- Set maximum directory seach depth. (DEFAULT=2)
* `-h, --help   ` -- Print usage help.

# Configuration
 You can avoid specifying the `--subdir` and `--depth` by creating a config.toml at `$XDG_CONFIG_HOME/jmks/config.toml`. If `XDG_CONFIG_HOME` is not set, then use `$HOME/.config/jmks/config.toml`.

Here's an example of a config.toml file
```toml
subdir = "/path/to/download/desired/location"
depth = 2 #Optional 
```

:warning: **WARNING:** Must use absolute paths, environment variables will not be parsed and tilde `~` home will not work either.
