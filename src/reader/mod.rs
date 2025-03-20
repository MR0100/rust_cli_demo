use crate::parse_config::ParseConfig;

use std::fs;

// read the content from the file and print the result on console. 
pub fn read(config: ParseConfig) -> Result<(), std::io::Error> { 
  
  // "?" at the end will check if the "fs::read_to_string" method returns error or not. 
  // if the error is returned then will return from that line directly.
  let content =  fs::read_to_string(&config.filename)?;

  println!("{}", content);

  let _ = search(&content, &config.query, config.is_case_sensitive);

  Ok(())
}

pub fn search<'a>(content: &'a String, query: &String, is_case_sensitive: bool) -> Result<(), &'a str> {  

  let mut search_found = false;

  println!("\n\n|------------------------\n| Searching for '{}'\n|------------------------", query);
  for line in content.lines() { 
    if is_case_sensitive {
      if line.contains(query) { 
        println!("| {}", line);
        search_found = true;
      }
    }else{
      if line.to_lowercase().contains(&query.to_lowercase()) { 
        println!("| {}", line);
        search_found = true;
      }
    }
    
  }

  if !search_found { 
   return Err("No Search Found!");
  }

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
      filename: String::from("main.txt"),
      is_case_sensitive: false,
    };

    read(config).unwrap_or_else(|_err| { 
      panic!("File Not Found!");
    });
  }

  #[test]
  fn file_found() { 
    let config = ParseConfig { 
      query: String::from("Search"), 
      filename: String::from("poem.txt"),
      is_case_sensitive: false,
    };

    let content = read(config).unwrap_or_else(|_err| { 
      panic!("File Not Found!");
    });    

    assert_eq!(content, ());

  }

  #[test]
  fn search_found_case_sensitive() { 
    let content = String::from("Hello World\nThis is a Test\nSearch for this\nThis is a Test\nHello World\n");
    let query = String::from("Search");

    let result = search(&content, &query, true).unwrap_or_else(|_err| { 
      panic!("Search Not Found!");
    });

    assert_eq!(result, ());
  }

  #[test]
  #[should_panic(expected = "No Search Found!")]
  fn search_not_found_case_sensitive() { 
    let content = String::from("Hello World\nThis is a Test\nSearch for this\nThis is a Test\nHello World\n");
    let query = String::from("Kite");

    let _result = search(&content, &query, true).unwrap_or_else(|_err| { 
      panic!("No Search Found!");
    });
  }

   #[test]
  fn search_found_case_insensitive() { 
    let content = String::from("Hello World\nThis is a Test\nSearch for this\nThis is a Test\nHello World\n");
    let query = String::from("search");

    let result = search(&content, &query, false).unwrap_or_else(|_err| { 
      panic!("Search Not Found!");
    });

    assert_eq!(result, ());
  }

  #[test]
  #[should_panic(expected = "No Search Found!")]
  fn search_not_found_case_insensitive() { 
    let content = String::from("Hello World\nThis is a Test\nSearch for this\nThis is a Test\nHello World\n");
    let query = String::from("kite");

    let _result = search(&content, &query, false).unwrap_or_else(|_err| { 
      panic!("No Search Found!");
    });
  }
}