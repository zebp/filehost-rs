use std::fs;
use std::io::Result;
use std::path::PathBuf;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name_generator: NameGenerator,
    pub keys: Vec<String>,
    pub max_file_size: usize,
    pub upload_directory: PathBuf,
    pub redirect_template: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        if let Ok(contents) = fs::read_to_string("config.yml") {
            let config: Self = serde_yaml::from_str(&contents).expect("could not read config");
            fs::create_dir_all(&config.upload_directory)?;
            Ok(config)
        } else {
            eprintln!("Could not find config.yml, creating template.");
            fs::write("config.yml", include_str!("default_config.yml"))?;
            Self::load()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NameGenerator {
    Alphanumeric {
        length: usize,
    },
    Gyfcat {
        adjective_count: usize,
        noun_count: usize,
    },
    Numeric,
}

impl NameGenerator {
    pub fn generate_name(&self) -> String {
        match *self {
            Self::Numeric => rand::random::<u32>().to_string(),
            Self::Alphanumeric { length } => {
                let mut rng = thread_rng();
                std::iter::repeat(())
                    .map(|()| rng.sample(Alphanumeric))
                    .take(length)
                    .collect()
            },
            Self::Gyfcat { adjective_count, noun_count } => {
                let adjectives: String = std::iter::repeat_with(adjective).take(adjective_count).collect();
                let nouns: String = std::iter::repeat_with(noun).take(noun_count).collect();

                format!("{}{}", adjectives, nouns)
            },
        }
    }
}

fn noun() -> String {
    let nouns: Vec<String> = include_str!("nouns.txt")
        .lines()
        .map(String::from)
        .map(capitalize_word)
        .collect();

    nouns[rand::random::<usize>() % nouns.len()].clone()
}

fn adjective() -> String {
    let adjectives: Vec<String> = include_str!("adjectives.txt")
        .lines()
        .map(String::from)
        .map(capitalize_word)
        .collect();

    adjectives[rand::random::<usize>() % adjectives.len()].clone()
}

fn capitalize_word(word: String) -> String {
    if word.len() < 2 {
        return word.to_uppercase()
    }

    let first_letter = &word[0..1].to_uppercase();
    let second_part = &word[1..].to_lowercase();
    format!("{}{}", first_letter, second_part)
}
