use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, BufWriter, Write};
use std::io;

const MIN_ARGS: usize = 3;
const USAGE_STR: &str = "Usage: ./genx <domain(s)> <wordlist.txt> <out_file>\n\tdomains - a string or file containing a list of domains";

const GENERATED_SUBS: &str = "gen_subdomains.txt";

const SUB_COMBINATIONS: &'static [&'static str] = &["{}-{}","{}{}","{}_{}"];

const KEEP_INTERMEDIATE_FILE: bool = false;

/*
#[derive(Default)]
struct ArgOptions {
    mode: u8, // the mode to use when generating data: specific vs all up to
    level: u8, // level of the mode: 0, 1, 2
    domain: String, // domain name
    in_fname: String, // input wordlist
    //domains_fname: Option<&'static str> // input domains list
    out_fname: Option<&'static str> // output file (Optional, will print to stdout by default)
}
*/

fn is_file(name: &str) -> bool {
    return Path::new(name).exists();
}

fn generate_subdomains_from_wordlist(wordlist_fname: &str, out_fname: &str) -> Result<()> {
    let f1 = File::open(wordlist_fname).unwrap();
    let f2 = File::open(wordlist_fname).unwrap();

    let mut out_writer = BufWriter::new(File::create(&Path::new(&out_fname)).unwrap());

    for sub1 in BufReader::new(f1).lines() {
        let s1 = sub1?;
        for sub2 in BufReader::new(&f2).lines() {
            // have to do it one by one since rust's format! only allows string literals (probably
            // to protect from format strings)
            let s2 = sub2?;
            out_writer.write(format!("{}-{}\n", s1, s2).as_bytes()).unwrap(); 
            out_writer.write(format!("{}{}\n", s1, s2).as_bytes()).unwrap(); 
            out_writer.write(format!("{}_{}\n", s1, s2).as_bytes()).unwrap();
            /*
            if numbered {
                for i in 0..10 {
                    out_writer.write(format!("{}-{}\n", s1, s2).as_bytes()).unwrap(); 
                    out_writer.write(format!("{}{}\n", s1, s2).as_bytes()).unwrap(); 
                    out_writer.write(format!("{}_{}\n", s1, s2).as_bytes()).unwrap();
                }
            }*/
        }
    }
    Ok(())
}

fn combine_domain_wordlist(domain: &str, wordlist_fname: &str, out_writer: &mut BufWriter<File>) -> Result<()> {
    let file = File::open(wordlist_fname).unwrap();

    for line in BufReader::new(file).lines() {
        out_writer.write(format!("{}.{}\n", line?, domain).as_bytes()).unwrap();
    }
    Ok(())
}

fn combine_list_domains_wordlist(domains_fname: &str, wordlist_fname: &str, out_writer: &mut BufWriter<File>) -> Result<()> {
    let file = File::open(domains_fname).unwrap();

    for line in BufReader::new(file).lines() {
        combine_domain_wordlist(&line?, &wordlist_fname, out_writer);
    }
    Ok(())
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

/*
    let mut out_writer = BufWriter::new(match out_fname {
            None => Box::new(io::stdout()) as Box<Write>,
            Some(ref fname) => Box::new(File::create(&Path::new(fname)).unwrap()) as Box<Write>,
    });
 
    let opts = ArgOptions { 
        domain: args[2].clone(),
        in_fname: args[1].clone(),
        out_fname: Some(out_fname.clone()),
        ..Default::default()
    };
*/

    if is_file(&args[1]) {
        combine_list_domains_wordlist(&args[1], &wordlist_fname, &mut out_writer);
    } else {
        combine_domain_wordlist(&args[1], &wordlist_fname, &mut out_writer);
    }

    if !KEEP_INTERMEDIATE_FILE {
        fs::remove_file(GENERATED_SUBS);
    }
}
