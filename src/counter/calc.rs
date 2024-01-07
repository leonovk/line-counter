use std::fs::File;
use std::io::{self, BufRead};

pub fn count_lines(file_path: &String) -> u64 {
      let file = File::open(file_path).expect("could not open one of the files");
      let reader = io::BufReader::new(file);

      let mut line_count: u64 = 0;
  
      for _line in reader.lines() {
          line_count += 1;
      }
  
      line_count
}
