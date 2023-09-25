use std::fs;
use std::path::{Path, PathBuf};
use std::io::Error as IOError;
use std::env::var;
use regex::Regex;
use serde_derive::Deserialize;
use clap::Parser;

pub const DEFAULT_DEPTH: u32 = 2;
pub mod string_carousel;

trait PathExt {
    fn has_extension(&self, e: &str) -> bool;
}

impl PathExt for PathBuf {
    fn has_extension(&self, ext: &str) -> bool {
        match self.extension() {
            Some(x) if x == ext => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ConfigWrap {
    pub subdir: Option<PathBuf>,
    pub depth:  Option<u32>,
}

impl Into<Config> for ConfigWrap {
    fn into(self) -> Config {
        Config {
            subdir: self.subdir.unwrap(),
            depth:  self.depth.unwrap(),
        }
    }
}

pub struct Config {
    pub subdir: PathBuf,
    pub depth:  u32,

}

#[derive(Parser, Debug)]
pub struct Cli {
    /// Pattern to search
    pub pattern: String,
    /// Set the subtitle directory
    #[clap(long,short,action)]
    pub subdir: Option<PathBuf>,
    /// Set max search depth
    #[clap(long,short,action)]
    pub depth: Option<u32>,
    /// Ignore lines that contain this pattern
    #[clap(long,short,value_name="NEGATIVE PATTERN",action)]
    pub ignore: Option<String>,
    /// Lines of context before & after match
    #[clap(long,short = 'C',value_name="N LINES",action)]
    pub context: Option<u32>,
    /// Lines of context before match
    #[clap(long,short = 'B',value_name="N LINES",action)]
    pub before: Option<u32>,
    /// Lines of context after match
    #[clap(long,short = 'A',value_name="N LINES",action)]
    pub after: Option<u32>,
}

pub fn get_sub_files(paths: &mut Vec<PathBuf>, dir: &Path, depth: u32) -> Result<(), IOError> {
    if depth <= 0 {
        return Ok(());
    }

    let items = fs::read_dir(dir)?;

    for item in items {
        let item = item?;
        let path = item.path();

        if path.is_file() && path.has_extension("ass") {
            paths.push(path);
        } else if path.is_dir() {
            get_sub_files(paths, &path, depth - 1)?;
        }
    }

    Ok(())
}

pub fn load_config(args: &Cli) -> Option<Config> {
    let mut config = match read_config_file() {
        Ok(config) => config,
        Err(_) => {
            ConfigWrap {subdir: None, depth: None}
        }
    };
    // Cli args take precedence:
    if args.subdir.is_some() {
        config.subdir = args.subdir.clone();
    }
    if args.depth.is_some() {
        config.depth = args.depth.clone();
    }
    
    // "subdir" MUST be set one way or another:
    if config.subdir.is_none() {
        eprintln!("Must set subdir in config.toml OR using \"--subdir=\" with cli.");
        return None;
    }
    if config.depth.is_none() {
        config.depth = Some(DEFAULT_DEPTH);
    }

    // This appends a "/" to the end of the path if one is not there
    config.subdir.as_mut().unwrap().push("");
    
    // return Some config with unwrapped members:
    Some(config.into())
}

fn read_config_file() -> Result<ConfigWrap,Box<dyn std::error::Error>> {
    let config_path = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config/jmks/config.toml",home)))?;
    let contents = fs::read_to_string(&config_path)?;
    let config = toml::from_str(&contents)?;
    
    Ok(config)
}

pub fn extract_sub_ass(line: &String) -> Option<(&str, &str, &str)>{
    let start = &line[12..22]; 
    let end = &line[23..33];
    let mut text = &line[34..];
    
    //Get index of Effect feild
    let (char_idx, byte_idx) = index_of_nth_tgt(text, 5, ',').unwrap();

    // Make sure Effect feild is empty
    // Ignore lines with effects because they're usually are not actual dialogue
    text = match text.chars().nth(char_idx+1) {
        Some(',') => &text[byte_idx+2..],
        _ => return None,
    };

    // Text feilds that start with "{" usually are not dialogue either, so ignore
    match text.chars().nth(0) {
        Some('{') => None,
        _ => Some((start,end,text)),
    }
}

fn index_of_nth_tgt(s: &str, n: usize, tgt: char) -> Option<(usize,usize)> {
    let mut count = 0;
    for (char_idx, (byte_idx, char)) in s.char_indices().enumerate() {
        if char == tgt {
            count += 1;
            if count == n {
                return Some((char_idx, byte_idx)) ;
            }
        }
    }
    None
}

fn splice_out<'a>(line: &'a str, tgt: &str) -> Option<(&'a str, &'a str)> {
    let start = match line.find(tgt) {
        Some(start) => start,
        None => return None,
    };
    let end = start+tgt.len();
    Some((&line[..start], &line[end..]))
}

pub fn splice_out_all_and_replace_into<'a>(into: &mut String, line: &'a str, tgt_cut: &str, tgt_repl: char) {
    into.clear();
    let mut next = line;
    while let Some((left, right)) = splice_out(next, tgt_cut) {
        into.push_str(left);
        into.push(tgt_repl);
        next = right;
    }
    into.push_str(next);
    
}

pub fn highlight_matches(formated_line: &mut String, text: &str, re: &Regex) {
    let mut slice = text;
    while let Some(m) = re.find(slice){
        if m.start() == m.end() { 
            break; 
        }
        formated_line.push_str(&slice[..m.start()]); // before match
        for item in ["\x1b[1;31m", m.as_str(), "\x1b[0m"] {
            formated_line.push_str(item);
        }
        slice = &slice[m.end()..];
    }
    formated_line.push_str(slice);

}