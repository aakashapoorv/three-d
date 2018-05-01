use dust::loader;
use std::io::BufRead;
use std::str;

pub fn load<F, T>(name: &str, mut on_load: F) where F: FnMut(Vec<T>), T: str::FromStr
{
    loader::load( name, |text: Box<BufRead>|
    {
        println!("");
        println!("Loading: {}", name);
        let mut meta_data = MetaData {format: FORMAT::NONE, file_type: FILETYPE::NONE, no_attributes: -1};
        let mut data = Vec::new();
        let mut reading_data = false;
        for line in text.lines()
        {
            let l = line.unwrap();
            let mut words: Vec<&str> = l.trim().split(|x| (x == ' ') || (x == '(') || (x == ')') ).map(|s| s.trim()).collect();
            words.retain(|&i| i != "" && i != ")" && i != "(");

            if words.len() > 0
            {
                if *words.first().unwrap() == "//"
                {
                    reading_data = true;
                }
                if !reading_data
                {
                    read_meta_data_into(&words, &mut meta_data);
                }
                else {
                    if words.len() > 1
                    {
                        match meta_data.file_type {
                             FILETYPE::FACES => { words = words[1..].to_vec(); },
                            _ => {}
                        }
                    }
                    read_data_into(&words, &mut meta_data, &mut data);
                }
            }
        }
        println!("Format: {:?}", meta_data.format);
        println!("File type: {:?}", meta_data.file_type);
        on_load(data);
    });
}

#[derive(Debug)]
enum FORMAT {ASCII, BINARY, NONE}

#[derive(Debug)]
enum FILETYPE {POINTS, FACES, OWNER, NONE}

struct MetaData {
    format: FORMAT,
    file_type: FILETYPE,
    no_attributes: i32
}

fn read_data_into<T>(words: &Vec<&str>, meta_data: &mut MetaData, data: &mut Vec<T>) where T: str::FromStr
{
    for word in words {
        if meta_data.no_attributes == -1
        {
            match word.parse::<i32>() {
                Ok  (i) => { meta_data.no_attributes = i },
                Err(..) => {},
            }
        }
        else {
            match word.parse::<T>() {
                Ok  (i) => {data.push(i)},
                Err(..) => {},
            }
        }
    }
}

fn read_meta_data_into(words: &Vec<&str>, meta_data: &mut MetaData)
{
    if words.len() > 1
    {
        match *words.first().unwrap() {
            "format" => { meta_data.format = match words[1] {
                "ascii;" => FORMAT::ASCII,
                "binary;" => FORMAT::BINARY,
                _ => FORMAT::NONE
            }},
            "object" => { meta_data.file_type = match words[1] {
                "owner;" => FILETYPE::OWNER,
                "points;" => FILETYPE::POINTS,
                "faces;" => FILETYPE::FACES,
                _ => FILETYPE::NONE
            }},
            &_ => {}
        }
    }
}