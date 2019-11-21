extern crate csv;
extern crate regex;

use std::collections::{HashMap};
use std::collections::hash_map::RandomState;

use csv::Error;

use crate::utils::*;
use std::io;

pub fn train(content_dir: &str) -> Result<(), Error> {
    println!("TRAINING...");
    let mut classes = read_reviews(content_dir).unwrap();
    let mut joined_reviews:Vec<String> = vec!["".to_owned(); 3];

    for (class_name, reviews) in classes.iter().enumerate() {
        for review in reviews.iter() {
            joined_reviews[class_name].push_str(review);
        }
    }

    let mut class_word_count : Vec<HashMap<String, usize>> = Vec::new();

    for (class, reviews) in joined_reviews.iter().enumerate() {
        let mut map = HashMap::new();

        for s in reviews.split_whitespace() {
            let count = map.entry(s.to_owned()).or_insert(0);
            *count += 1;
        }
        class_word_count.push(map);
    }

    println!("counting occurrences...");
    let mut writer = csv::Writer::from_path([content_dir, CLASS_WORDS_BY_COUNT].concat())?;
    writer.write_record(&["class", "word", "count"]);

    for (class_name, class_map) in class_word_count.iter().enumerate() {
        for (word, occurrences) in class_map {
            writer.write_record(&[&class_name.to_string(), &word, &occurrences.to_string()]);
        }
    }

    writer.flush().expect("Unable to flush");
    println!("counting words by class...");
    let mut writer = csv::Writer::from_path([content_dir, OVERALL_WORDS_COUNT_PER_CLASS].concat())?;
    writer.write_record(&["class", "count"]);

    for (class_name, class_reviews) in classes.iter().enumerate() {
        let mut count = 0;
        for review in class_reviews.iter() {
            count += review.split_whitespace().count();
        }
        writer.write_record(&[&class_name.to_string(), &count.to_string()]);
    }

    writer.flush().expect("Unable to flush");
    println!("counting reviews...");

    count_reviews(&classes, content_dir).unwrap();
    Ok(())
}

fn count_reviews(reviews_per_class: &Vec<Vec<String>>, content_dir: &str) -> Result<(), Error> {
    let mut writer = csv::Writer::from_path([content_dir, REVIEWS_COUNT_PER_CLASS].concat())?;
    writer.write_record(&["class", "count"]);

    for (class_name, class_reviews) in reviews_per_class.iter().enumerate() {
        writer.write_record(&[&class_name.to_string(), &class_reviews.len().to_string()]);
    }

    writer.flush().expect("Unable to flush");
    Ok(())
}

fn read_reviews(content_dir: &str) -> Result<Vec<Vec<String>>, io::Error>{
    let mut negative = Vec::new();
    let mut neutral = Vec::new();
    let mut positive = Vec::new();
    let data = read_text([content_dir, REVIEWS_PATH].concat());
    let mut reader = csv::Reader::from_reader(data.as_bytes());

    for record in reader.records() {
        let record = record?;
        let review_type = &record[1];
        let review_text = compile_regex(&record[2].to_lowercase().to_string()).to_string();

        match review_type{
            "negative" => {
                negative.push(review_text);
            },
            "neutral" => {
                neutral.push(review_text);
            },
            _ => {
                positive.push(review_text);
            }
        }
    }
    Ok(vec![negative, neutral, positive])
}