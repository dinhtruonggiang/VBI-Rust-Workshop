use std::io::{self, BufRead};
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use regex::RegexBuilder;

fn count_regex_matches<T: BufRead + Sized>(reader: T, re: Regex) -> usize {
    let mut total_count = 0;
    for line_ in reader.lines() {
        let line = line_.unwrap();
        total_count += re.find_iter(&line).count();
    }
    total_count
}

fn main() {
    let file_name = String::from("1-s2.0-S0960982203005347-mmc6.txt");
    let mut lookup_string: String = String::new();

    println!("Enter the lookup string:");
    io::stdin().read_line(&mut lookup_string).unwrap();
    lookup_string = lookup_string.replace(&['\n', '\r'][..], "");
    
    println!("input={}", lookup_string);
    let re = RegexBuilder::new(&lookup_string).case_insensitive(true).build().expect("Invalid Regex");
    
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let total_count = count_regex_matches(reader, re);

    println!("Số lượng '{}' là: {}", lookup_string, total_count);
}
