#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::string::String;
use std::collections::BTreeMap;
use std::vec::Vec;

pub struct Config {
    strings: BTreeMap<String, String>,
    numbers: BTreeMap<String, f32>,
    bools: BTreeMap<String, bool>,
    path: String,
}

impl Config {
    pub fn new(path: &str) -> Config {
        return Config {
                   bools: BTreeMap::new(),
                   strings: BTreeMap::new(),
                   numbers: BTreeMap::new(),
                   path: path.to_string(),
               };
    }

    pub fn save(&self) {
        let mut file = match OpenOptions::new()
                  .write(true)
                  .truncate(true)
                  .create(true)
                  .open(&self.path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't open {}: {}", self.path, why.description()),
        };
        let mut lines: Vec<String> = Vec::new();
        for (key, value) in self.bools.iter() {
            lines.push(format!("{}: {}\n", key, value))
        }
        for (key, value) in self.numbers.iter() {
            lines.push(format!("{}: {}\n", key, value))
        }
        for (key, value) in self.strings.iter() {
            lines.push(format!("{}: {}\n", key, value))
        }
        lines.sort();
        for line in lines {
            match file.write_all(line.as_bytes()) {
                Ok(_) => {}
                Err(why) => panic!("Couldn't write {}: {}", self.path, why.description()),
            };
        }
    }

    pub fn get_string(&mut self, key: &str, default: &str) -> String {
        let result = match self.strings.get(key) {
            None => default.to_string(),
            Some(val) => val.clone(),
        };
        self.strings.insert(key.to_string(), result.clone());
        return result;
    }

    pub fn get_number(&mut self, key: &str, default: &f32) -> f32 {
        let result = match self.numbers.get(key) {
            None => default.clone(),
            Some(val) => val.clone(),
        };
        self.numbers.insert(key.to_string(), result);
        return result;
    }

    pub fn get_bool(&mut self, key: &str, default: &bool) -> bool {
        let result = match self.bools.get(key) {
            None => default.clone(),
            Some(val) => val.clone(),
        };
        self.bools.insert(key.to_string(), result);
        return result;
    }

    pub fn set_string(&mut self, key: &str, value: &str) {
        self.strings.insert(key.to_string(), value.to_string());
    }

    pub fn set_number(&mut self, key: &str, value: &f32) {
        self.numbers.insert(key.to_string(), value.clone());
    }

    pub fn set_bool(&mut self, key: &str, value: &bool) {
        self.bools.insert(key.to_string(), value.clone());
    }
}

pub fn load(path: &str) -> Config {
    let mut config_string = "".to_string();
    match File::open(path) {
        Err(why) => println!("Couldn't open {}: {}", path, why.description()),
        Ok(file) => {
            let mut file = file;
            match file.read_to_string(&mut config_string) {
                Err(why) => println!("Couldn't read {}: {}", path, why.description()),
                Ok(_) => {}
            };
        }
    };

    let mut config = Config::new(path);

    for line in config_string.lines() {
        let i = match line.find(":") {
            None => continue,
            Some(i) => i,
        };
        let key = line[0..i].trim();
        let strvalue = line[i + 1..line.len()].trim();
        match strvalue.to_lowercase().parse::<bool>() {
            Ok(val) => {
                config.bools.insert(key.to_string(), val);
                continue;
            }
            Err(_) => {}
        }
        match strvalue.parse::<f32>() {
            Ok(val) => {
                config.numbers.insert(key.to_string(), val);
                continue;
            }
            Err(_) => {}
        }
        config
            .strings
            .insert(key.to_string(), strvalue.to_string());
    }

    return config;
}
