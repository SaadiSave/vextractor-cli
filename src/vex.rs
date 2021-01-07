#![allow(dead_code)]

use fs::read_to_string;
use std::collections::HashSet;
use std::fs;

fn contains_str(vec: &Vec<String>, s: &String) -> bool {
    vec.iter().any(|x| x == s)
}

#[derive(Clone)]
pub struct Vextract {
    punc: String,
    plist: Vec<String>,
    alist: Vec<String>,
    voc: HashSet<String>,
    pub text: String,
    pub vocab: Vec<String>,
}

impl Vextract {
    pub fn new(fileurl: &str, al: Vec<&str>, pl: Vec<&str>) -> Vextract {
        let mut vset: HashSet<String> = HashSet::new();
        let ftext = read_to_string(fileurl).expect("unable to read file");

        for i in ftext.split('\n') {
            for j in i.split(' ') {
                vset.insert(j.to_string());
            }
        }

        let mut tmp = Vextract {
            punc: "!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~“”„‚‘’（）".to_string(),
            plist: pl.iter().map(|s| s.clone().to_string()).collect(),
            alist: al.iter().map(|s| s.clone().to_string()).collect(),
            voc: vset,
            text: ftext,
            vocab: Vec::new(),
        };
        tmp.pstrip();
        tmp.make_lower();
        tmp.remove_nums();

        tmp
    }

    pub fn pstrip(&mut self) {
        let mut tmp: HashSet<String> = HashSet::new();
        let mut tmpv: Vec<String> = Vec::new();

        for i in self.voc.iter() {
            let mut cpunc = true;
            let mut j = i.clone();

            while cpunc {
                let mut x = (j.chars().count() - 1) as i64;
                if x < 0 {
                    x = 0;
                }

                let y = x as usize;

                if contains_str(&self.alist, i) {
                    tmp.insert((*i.clone()).to_string());
                    tmpv.push((*i.clone()).to_string())
                } else {
                    if self.punc.contains(j.chars().nth(y).unwrap_or('0')) {
                        j.pop();
                        continue;
                    }
                    if self.punc.contains(j.chars().nth(0).unwrap_or('0')) {
                        let b = j.chars().nth(0).unwrap();
                        j = j.replace(b, "");
                        continue;
                    }
                }

                cpunc = false;
            }

            tmp.insert(j.clone());
            tmpv.push(j);
        }

        self.voc.clear();
        self.voc.extend(tmp);
        self.vocab.clear();
        self.vocab.extend(tmpv);
    }

    pub fn make_lower(&mut self) {
        let mut tmp: HashSet<String> = HashSet::new();
        let mut tmpv: Vec<String> = Vec::new();

        for i in self.voc.iter() {
            let j = i.clone();

            if contains_str(&self.plist, &j) || contains_str(&self.alist, &j) {
                tmp.insert(j.clone());
                tmpv.push(j);
            } else {
                tmp.insert(j.clone().to_lowercase());
                tmpv.push(j.to_lowercase());
            }
        }

        self.voc.clear();
        self.voc.extend(tmp);
        self.vocab.clear();
        self.vocab.extend(tmpv);
    }

    pub fn remove_nums(&mut self) {
        let mut tmp: HashSet<String> = HashSet::new();
        let mut tmpv: Vec<String> = Vec::new();

        for i in self.voc.iter() {
            let j = i.clone();
            match j.parse::<f64>() {
                Ok(_s) => (),
                Err(_s) => {
                    tmp.insert(j.clone());
                    tmpv.push(j);
                }
            }
        }

        self.voc.clear();
        self.voc.extend(tmp);
        self.vocab.clear();
        self.vocab.extend(tmpv);
    }

    pub fn add_punctuation(&mut self, s: &str) {
        self.punc += s;
    }

    pub fn get_vocab(&self) -> Vec<String> {
        self.vocab.clone()
    }

    pub fn get_pretty_vocab(&self) -> String {
        let mut x = String::new();
        for y in self.vocab.clone() {
            x += format!("{}\n", y).as_str();
        }

        x
    }

    pub fn get_sorted_vocab(&self) -> Vec<String> {
        let mut x = self.vocab.clone();
        x.sort();
        x
    }

    pub fn get_sorted_pretty_vocab(&self) -> String {
        let mut x = String::new();
        let mut y = self.vocab.clone();
        y.sort();

        for y in y {
            x += format!("{}\n", y).as_str();
        }

        x
    }

    pub fn get_len(&self) -> u32 {
        self.vocab.len() as u32
    }

    pub fn write_to_file(&self, fileurl: &str) {
        fs::write(fileurl, self.get_sorted_pretty_vocab()).expect("Err");
    }
}
