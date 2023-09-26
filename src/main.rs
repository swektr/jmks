use std::{process::ExitCode, 
          path::PathBuf,
          fs::File,
          io::{BufReader, BufRead}};
use clap::Parser;
use regex::Regex;
use jmks::{*, string_carousel::StringCarousel};

fn main() -> ExitCode {
    let args = Cli::parse();
    let config = match load_config(&args) {
        Some(config) => config,
        None => return ExitCode::FAILURE,
    };
    let re = Regex::new(&args.pattern).unwrap();
    let ignore_re = match args.ignore {
        Some(p) => Some(Regex::new(&p).unwrap()),
        None => None,
    };

    let (mut ctx_before, mut ctx_after) = match args.context {
        Some(val) => (val, val),
        None      => (0,0),
    };
    if let Some(val) = args.before {
        ctx_before = val;
    }
    if let Some(val) = args.after {
        ctx_after = val;
    }


    let subdir_length = config.subdir.as_os_str().to_str().unwrap().len();
    
    // Find subtitle files and sort
    let mut paths: Vec<PathBuf> = Vec::new();
    get_sub_files(&mut paths, &config.subdir, config.depth).unwrap();
    paths.sort();

    let mut formated_line = String::new();
    let mut line_buf = String::new();
    let mut context_buf = StringCarousel::init_with(ctx_before as usize, || String::with_capacity(100));
    let mut num_matches = 0;
    let mut after_lines_left = 0;
    let mut finished_printing_after_context = false;
    for path in &paths {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let path = path.as_os_str().to_str().unwrap();
        let trunc_path_str = &path[subdir_length..];
        for line in reader.lines() {
            let line = line.unwrap();
            // filter out non-^Dialogue lines:
            if  line.len() < 10 || &line.as_bytes()[..10] != b"Dialogue: " {
                continue;
            }
            // Extract valid dialogue lines (ignore lines with effects)
            let (start, end, text_slice) = match extract_sub_ass(&line) {
                Some(x) => x,
                _ => continue,
            };
            // .ass subtitles will use a "\N" to specify new lines, this makes searching
            // inconvinient, so replace them with spaces instead
            splice_out_all_and_replace_into(&mut line_buf, text_slice, r"\N", ' ');
            let re_match = re.is_match(&line_buf);
            let ig_match = ignore_re.as_ref().is_some_and(|pat| pat.is_match(&line_buf));
            // Ignore lines that dont match UNLESS they are a part of the after-context
            if (!re_match || ig_match) && after_lines_left == 0 {
                if ctx_before > 0 {
                    context_buf.insert(
                        &["\x1b[0;35m", trunc_path_str, "\x1b[0m: ", start, " ", end, "\x1b[0;34m : \x1b[0m", &line_buf]
                    );
                }
                continue; 
            }
            // Details about the line of text
            for item in ["\x1b[0;35m", trunc_path_str, "\x1b[0m: ", start, " ", end, "\x1b[0;34m : \x1b[0m"] {
                formated_line.push_str(item);
            }
            // Higlight matches and start/reset after-context
            if re_match && !ig_match {
                after_lines_left = ctx_after;
                highlight_matches(&mut formated_line, &line_buf, &re);
                num_matches += 1;
            } else {
                formated_line.push_str(&line_buf);
                after_lines_left -= 1;
            }
            // Add visual seperation between matches when there is context
            if num_matches > 1 && ctx_before + ctx_after > 0 && finished_printing_after_context  {
                finished_printing_after_context = false;
                println!("--");
            }
            if after_lines_left == 0 {
                finished_printing_after_context = true;
            }
            context_buf.into_iter().for_each(|ctx| println!("{ctx}"));
            context_buf.clear_all();
            println!("{formated_line}");
            formated_line.clear(); 
        }
    }
    ExitCode::SUCCESS
}
