extern crate csv;
extern crate regex;

use std::collections::{HashMap, HashSet};

use csv::Error;
use crate::utils::*;
use std::io;

pub fn classify(content_dir: &str, text_file: &str) -> Result<(), Error>{
    println!("CLASSIFYING...");

    let class_word_count = make_class_word_count(content_dir).unwrap();

    let unique_words_count = count_unique_words(&class_word_count);

    let overall_words_count_per_class = make_overall_word_count(content_dir).unwrap();

    let review_count_by_class = make_reviews_count_by_class(content_dir).unwrap();

    let classifying_text_words_with_counts = make_classifying_text(content_dir, text_file).unwrap();

    let overall_reviews_count = count_all_reviews_in_training_set(&review_count_by_class);

    let mut posterior_probability = compute_posterior_probability(&review_count_by_class, overall_reviews_count); // P(c) = |Dc|/|D|

    let mut prior_probability = compute_prior_probability(
        &overall_words_count_per_class,
        &classifying_text_words_with_counts,
        &class_word_count,
        unique_words_count
    );

    let (max, max_probability) = find_max_probability(&posterior_probability, &prior_probability);
    println!("max: {} with full probability: {}", format_class(&max), max_probability);

    Ok(())
}

fn make_class_word_count(content_dir: &str) -> Result<Vec<HashMap<String, usize>>, io::Error>{
    let data = read_text([content_dir, CLASS_WORDS_BY_COUNT].concat());
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let mut class_word_count: Vec<HashMap<String, usize>> = vec![HashMap::new(); 3];
    for record in reader.records() {
        let record = record?;
        class_word_count[record[0].parse::<usize>().unwrap_or_default()].insert(record[1].to_string(), record[2].parse::<usize>().unwrap_or_default());
    }
    Ok(class_word_count)
}

fn make_overall_word_count(content_dir: &str) -> Result<Vec<usize>, io::Error>{
    let data = read_text([content_dir, OVERALL_WORDS_COUNT_PER_CLASS].concat());
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let mut overall_words_count_per_class = Vec::new();
    for record in reader.records() {
        let record = record?;
        overall_words_count_per_class.push(record[1].parse::<usize>().unwrap_or_default());
    }
    Ok(overall_words_count_per_class)
}


fn count_unique_words(word_counts_in_class: &Vec<HashMap<String, usize>>) -> usize{
    let mut set = HashSet::new();
    for class_map in word_counts_in_class.iter(){
        for (word, _) in class_map {
            set.insert(word);
        }
    }
    set.len()
}

fn make_reviews_count_by_class(content_dir: &str) -> Result<Vec<usize>, io::Error>{
    let data = read_text([content_dir, REVIEWS_COUNT_PER_CLASS].concat());
    let mut reader = csv::Reader::from_reader(data.as_bytes());

    let mut review_count_by_class = Vec::new();
    for record in reader.records() {
        let record = record?;
        review_count_by_class.push(record[1].parse::<usize>().unwrap_or_default());
    }
    Ok(review_count_by_class)
}

fn make_classifying_text(content_dir: &str, text_file: &str) -> Result<HashMap<String, usize>, io::Error>{
    let text= read_text([content_dir, "/", text_file].concat());
    let result = compile_regex(text.as_str());
    let words = result.split_whitespace();

    let mut classifying_text_words_with_counts = HashMap::new();
    for word in words {
        let count = (&mut classifying_text_words_with_counts).entry(word.to_owned()).or_insert(0);
        *count += 1;
    }
    Ok(classifying_text_words_with_counts)
}

fn count_all_reviews_in_training_set(review_count_by_class: &Vec<usize>) -> usize{
    let mut overall_reviews_count = 0;
    for review_count_in_class in review_count_by_class.iter() {
        overall_reviews_count += *review_count_in_class;
    }
    overall_reviews_count
}

fn compute_posterior_probability(review_count_by_class: &Vec<usize>, overall_reviews_count: usize) -> Vec<f64>{
    let mut posterior_probability = Vec::new();
    for (class_name, reviews_count) in review_count_by_class.iter().enumerate() {
        posterior_probability.push((*reviews_count as f64 /*Dc*/ /overall_reviews_count as f64 /*D*/).log2()); // P(c) = |Dc|/|D|
    }
    posterior_probability
}

fn compute_prior_probability(overall_words_count_per_class: &Vec<usize>,
                             classifying_text_words_with_counts: &HashMap<String, usize>,
                             class_word_count: &Vec<HashMap<String, usize>>,
                             unique_words_count: usize) -> Vec<f64>{
    let mut prior_probability = Vec::new();

    for class in NEGATIVE..=POSITIVE {

        let count_words_in_class = overall_words_count_per_class[class];
        let mut sum = 0.0;
        for (word, count) in classifying_text_words_with_counts.iter() {

            let word_occurrences_count = count * class_word_count[class].get(word).unwrap_or(&0)+1; //Nci + a  a = 1
            sum /*p(wi| c)*/ += (word_occurrences_count as f64 /*Nci + a*/ / (count_words_in_class + unique_words_count) as f64 /*Nc + a|V|*/).log2();

        }
        prior_probability.push(sum);

    }
    prior_probability
}

fn find_max_probability(posterior_probability: &Vec<f64>, prior_probability: &Vec<f64>) -> (usize, f64){
    let mut v = Vec::new();
    for class_name in NEGATIVE..=POSITIVE {
        let full_probability = posterior_probability[class_name] + prior_probability[class_name];
        println!("{}: {} + {} = {}", format_class(&class_name), posterior_probability[class_name], prior_probability[class_name], &full_probability);
        v.push(full_probability);
    }

    let mut max_value = v.iter().fold(v[0], |mut max, &val| {
        if val > max {
            max = val;
        }
        max
    });
    let class_name = v.iter().position(|&r| r == max_value).unwrap();
    (class_name, max_value)
}