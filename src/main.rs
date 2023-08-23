use std::{process::ExitCode, 
          path::PathBuf,
          fs::File,
          io::{BufReader, BufRead}};
use clap::Parser;
use regex::Regex;
use jmks::*;

fn main() -> ExitCode {
    // Set up
    let args = Cli::parse();

    let config = match load_config(&args) {
        Some(config) => config,
        None => return ExitCode::FAILURE,
    };

    let re = Regex::new(&args.pattern).unwrap();
    let subdir_length = config.subdir.as_os_str().to_str().unwrap().len();
    
    // Find subtitle files and sort
    let mut paths: Vec<PathBuf> = Vec::new();
    get_sub_files(&mut paths, &config.subdir, config.depth).unwrap();
    paths.sort();

    let mut formated_line = String::new();
    let mut text = String::new();
    for path in &paths {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let path = path.as_os_str().to_str().unwrap();
        let trunc_path_str = &path[subdir_length..];
        
        for line in reader.lines() {
            let line = line.unwrap();
            if  line.len() < 10 || &line.as_bytes()[..10] != b"Dialogue: " {
                continue;
            }
            let (start,end,text_slice) = match extract_sub_ass(&line) {
                Some(x) => x,
                _ => continue,
            };
            // Slightly faster than str.replace()
            splice_out_all_and_replace_into(&mut text, text_slice, r"\N", ' ');

            if ! re.is_match(&text) {
                continue; 
            }
            // Details about the line of text
            for item in [trunc_path_str, ": ", start, " ", end, " => "] {
                formated_line.push_str(item);
            }
            
            highlight_matches(&mut formated_line, &text, &re);

            println!("{formated_line}");
            formated_line.clear(); 
        }
    }
    ExitCode::SUCCESS
}
