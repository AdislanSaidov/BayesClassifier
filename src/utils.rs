use std::borrow::Cow;

use regex::Regex;
use std::fs;

pub const NEGATIVE: usize = 0;
pub const NEUTRAL: usize = 1;
pub const POSITIVE: usize = 2;

pub const REVIEWS_PATH: &str = "/reviews.csv";
pub const CLASS_WORDS_BY_COUNT: &str = "/class_words_by_count.csv";
pub const OVERALL_WORDS_COUNT_PER_CLASS: &str = "/overall_words_count_per_class.csv";
pub const REVIEWS_COUNT_PER_CLASS: &str = "/reviews_count_per_class.csv";


pub fn compile_regex(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^A-Za-z]").unwrap();
    }
    RE.replace_all(text, " ")
}


pub fn read_text(text_path: String) -> String{
    fs::read_to_string(text_path).expect("Unable to read file")
}


pub fn format_class(i: &usize) -> &'static str {
    match i {
        &NEGATIVE => "NEGATIVE",
        &NEUTRAL => "NEUTRAL",
        _=> "POSITIVE"
    }
}

