#[macro_use]
extern crate clap;
use clap::App;
extern crate vextractor;
use vextractor::vex;
use vex::Vextract;
fn main() {
    let conf = load_yaml!("cli.yml");
    let matches = App::from_yaml(conf).get_matches();

    let infile = matches.value_of("infile").unwrap();
    let mut outfile: Option<&str> = None;
    let pr = matches.is_present("print");
    let mut alist: Vec<&str> = Vec::new();
    let mut plist: Vec<&str> = Vec::new();

    match matches.value_of("outfile") {
        Some(s) => outfile = Some(s),
        None => ()
    }

    match matches.value_of("alist") {
        Some(s) => {
            for i in s.split(' ') {
                alist.push(i.clone());
            }
        },
        None => ()
    }

    match matches.value_of("plist") {
        Some(s) => {
            for i in s.split(' ') {
                plist.push(i.clone());
            }
        },
        None => ()
    }

    let out = Vextract::new(infile, alist, plist);
    if pr == true {
        println!("{}", out.get_sorted_pretty_vocab())
    }
    
    match outfile {
        Some(s) => out.write_to_file(s),
        None => ()
    }
}
