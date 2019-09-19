use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const MIN_ARGS: usize = 3;
const USAGE_STR: &str = "Usage: ./genx <domain(s)> <wordlist.txt> <out_file>\n\tdomains - a string or file containing a list of domains";

const GENERATED_SUBS: &str = "gen_subdomains.txt";

const KEEP_INTERMEDIATE_FILE: bool = false;

fn is_file(name: &str) -> bool {
    return Path::new(name).exists();
}

fn generate_subdomains_from_wordlist(wordlist_fname: &str, out_fname: &str) {
    let f1 = File::open(wordlist_fname).unwrap();
    let f2 = File::open(wordlist_fname).unwrap();

    let mut out_writer = BufWriter::new(File::create(&Path::new(&out_fname)).unwrap());

    for sub1 in BufReader::new(f1).lines() {
        let s1 = sub1.unwrap();
        for sub2 in BufReader::new(&f2).lines() {
            // have to do it one by one since rust's format! only allows string literals (probably
            // to protect from format strings)
            let s2 = sub2.unwrap();
            writeln!(out_writer, "{}-{}\n", s1, s2).unwrap();
            writeln!(out_writer, "{}{}\n", s1, s2).unwrap();
            writeln!(out_writer, "{}_{}\n", s1, s2).unwrap();
            /*
            if numbered {
                for i in 0..10 {
                    writeln!(out_writer, "{}-{}\n", s1, s2)).unwrap(); 
                    writeln!(out_writer, "{}{}\n", s1, s2)).unwrap();
                    writeln!(out_writer, "{}_{}\n", s1, s2)).unwrap();
                }
            }*/
        }
    }
}

fn combine_domain_wordlist(domain: &str, wordlist_fname: &str, out_writer: &mut BufWriter<File>) {
    for line in BufReader::new(File::open(wordlist_fname).unwrap()).lines() {
        let word = line.unwrap();
        writeln!(out_writer, "{}.{}", word, domain);
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

    let mut wordlist_fname = args[2].clone();
    if !is_file(&wordlist_fname) {
        eprintln!("File: {} does not exist", wordlist_fname);
        return;
    }

    if args.len() > MIN_ARGS+1 {
        // assume we have to generate the subdomains first
        generate_subdomains_from_wordlist(&args[2], GENERATED_SUBS);
        wordlist_fname = GENERATED_SUBS.to_string();
    }

    let out_fname = args[3].clone();
    let mut out_writer = BufWriter::new(File::create(&Path::new(&out_fname)).unwrap());

    if is_file(&args[1]) {
        combine_list_domains_wordlist(&args[1], &wordlist_fname, &mut out_writer);
    } else {
        combine_domain_wordlist(&args[1], &wordlist_fname, &mut out_writer);
    }

    if !KEEP_INTERMEDIATE_FILE {
        fs::remove_file(GENERATED_SUBS);
    }
}
