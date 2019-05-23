use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, BufWriter, Write};
use std::io;

const MIN_ARGS: usize = 2;
const USAGE_STR: &str = "Usageb ./genx <wordlist.txt> <domains>\n\tdomains - a string or file containing a list of domains";

#[derive(Default)]
struct ArgOptions {
    mode: u8, // the mode to use when generating data: specific vs all up to
    level: u8, // level of the mode: 0, 1, 2
    domain: String, // domain name
    in_fname: String, // input wordlist
    out_fname: Option<&'static str> // output file (Optional, will print to stdout by default)
}

fn is_file(name: &str) -> bool {
    return Path::new(name).exists();
}

fn combine(opts: &ArgOptions) -> Result<()> {
    let file = File::open(&opts.in_fname).unwrap();
    let mut out_writer = BufWriter::new(match opts.out_fname {
            None => Box::new(io::stdout()) as Box<Write>,
            Some(ref fname) => Box::new(File::create(&Path::new(fname)).unwrap()) as Box<Write>,
    });
    

    for line in BufReader::new(file).lines() {
        out_writer.write(format!("{}.{}\n", line?, opts.domain).as_bytes()).unwrap();
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    if args.len() <= MIN_ARGS {
        eprintln!("{}", USAGE_STR);
        return;
    }

    let filename = args[1].clone();
    if !is_file(&filename) {
        eprintln!("File: {} does not exist", filename);
        return;
    }

    let out_fname = "out";

    let opts = ArgOptions { 
        domain: args[2].clone(),
        in_fname: args[1].clone(),
        out_fname: Some(out_fname.clone()),
        ..Default::default()
    };

    combine(&opts);
}

