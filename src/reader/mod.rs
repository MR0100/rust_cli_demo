use crate::parse_config::ParseConfig;

use std::fs;

// read the content from the file and print the result on console. 
pub fn read(config: ParseConfig) -> Result<(), std::io::Error> { 
  
  // "?" at the end will check if the "fs::read_to_string" method returns error or not. 
  // if the error is returned then will return from that line directly.
  let content =  fs::read_to_string(config.filename)?;

  println!("{}", content);

  Ok(())
}

#[cfg(test)]
mod tests { 

  use super::*;

  #[test]
  #[should_panic(expected = "File Not Found!")]
  fn file_not_exists() {
    let config = ParseConfig { 
      query: String::from("Search"), 
      filename: String::from("main.txt")
    };

    read(config).unwrap_or_else(|_err| { 
      panic!("File Not Found!");
    });
  }

  #[test]
  fn file_found() { 
    let config = ParseConfig { 
      query: String::from("Search"), 
      filename: String::from("poem.txt")
    };

    let content = read(config).unwrap_or_else(|_err| { 
      panic!("File Not Found!");
    });    

    assert_eq!(content, ());

  }
}