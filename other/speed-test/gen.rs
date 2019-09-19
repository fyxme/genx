use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const MIN_ARGS: usize = 3;
const USAGE_STR: &str = "Usage: ./gen <domains.txt> <words.txt> <out_file>\n";

fn is_file(name: &str) -> bool {
    return Path::new(name).exists();
}

fn combine_domain_wordlist(domain: &str, wordlist_fname: &str, out_writer: &mut BufWriter<File>) {
    for line in BufReader::new(File::open(wordlist_fname).unwrap()).lines() {
        write!(out_writer, "{}.{}\n", line.unwrap(), domain);
    }
}

fn combine_list_domains_wordlist(domains_fname: &str, wordlist_fname: &str, out_writer: &mut BufWriter<File>) {
    for domain_line in BufReader::new(File::open(domains_fname).unwrap()).lines() {
        let domain = domain_line.unwrap();
        combine_domain_wordlist(&domain, &wordlist_fname, out_writer);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    eprintln!("Arguments: {:?}", args);
    if args.len() <= MIN_ARGS {
        eprintln!("{}", USAGE_STR);
        return;
    }

    let wordlist_fname = args[2].clone();
    if !is_file(&wordlist_fname) {
        eprintln!("File: {} does not exist", wordlist_fname);
        return;
    }

    let out_fname = args[3].clone();
    let mut out_writer = BufWriter::new(File::create(&Path::new(&out_fname)).unwrap());

    combine_list_domains_wordlist(&args[1], &wordlist_fname, &mut out_writer);
}

