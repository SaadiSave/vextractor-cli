#[macro_use]
extern crate clap;
use clap::App;
extern crate vextractor;
use vextractor::vex;
use vex::Vextract;
use std::time::{Duration, Instant};

fn main() {
    let conf = load_yaml!("cli.yml");
    let matches = App::from_yaml(conf).get_matches();

    let infile = matches.value_of("infile").unwrap();
    let mut outfile: Option<&str> = None;
    let pr = matches.is_present("print");
    let mut alist: Vec<&str> = Vec::new();
    let mut plist: Vec<&str> = Vec::new();

    if let Some(s) = matches.value_of("outfile") {
        outfile = Some(s);
    }

    if let Some(s) = matches.value_of("alist") {
        for i in s.split(' ') {
            alist.push(i.clone());
        }
    }

    if let Some(s) = matches.value_of("plist") {
        for i in s.split(' ') {
            plist.push(i.clone());
        }
    }

    let mut now: Option<Instant> = None;
    let mut elapsed: Option<Duration> = None;

    if matches.is_present("perf") {
        now = Some(Instant::now());
    }

    let out = Vextract::new(infile, alist, plist);

    if let Some(s) = now {
        elapsed = Some(s.elapsed());
    }

    if pr == true {
        println!("{}", out.get_sorted_pretty_vocab())
    }

    if let Some(s) = outfile {
        out.write_to_file(s);
    }

    if let Some(s) = elapsed {
        println!("OK\nExec Time: {:?}", &s);
    }
}
