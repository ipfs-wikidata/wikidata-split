extern crate rustc_serialize;

use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::fs::create_dir_all;
use std::path::Path;

use rustc_serialize::json::Json;

static INPUT_NAME: &'static str = "latest-all.json";
static OUTPUT_DIR: &'static str = "split";

fn main() {
    let mut file = File::open(INPUT_NAME).unwrap();
    let mut reader = BufReader::new(&file);
    // println!("{:?}", &reader.lines().count());
    for line in reader.lines() {
        let l = line.unwrap();
        let line_length = l.len();
        // only first and last lines are going to be one character long
        if line_length == 1 {
            continue;
        }
        let json_part = &l[0..(line_length - 1)];
        let data = Json::from_str(&json_part).unwrap();
        let id = data.as_object().unwrap().get("id").unwrap().as_string().unwrap();

        let mut dir_name = OUTPUT_DIR.to_owned();
        let mut skip_chars = 0;
        if id.len() > 3 {
            dir_name = dir_name.to_owned() + "/";
            dir_name = dir_name.to_owned() + &id[0..3];
            skip_chars = 3;
        }
        if id.len() > 6 {
            dir_name = dir_name.to_owned() + "/";
            dir_name = dir_name.to_owned() + &id[3..6];
            skip_chars = 6;
        }
        if id.len() > 9 {
            dir_name = dir_name.to_owned() + "/";
            dir_name = dir_name.to_owned() + &id[6..9];
            skip_chars = 9;
        }
        let file_name = dir_name.to_owned() + "/" + &id[skip_chars..] + ".json";
        println!("{} - {}", id, file_name);

        create_dir_all(dir_name).unwrap();
        let mut out_file = File::create(file_name).unwrap();
        out_file.write_all(&json_part.as_bytes()).unwrap();
    }
}
