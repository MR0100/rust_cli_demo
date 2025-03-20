pub struct ParseConfig { 
    pub query: String, 
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl ParseConfig { 
  pub fn parse_config(args: &[String]) -> Result<ParseConfig, &str> {
    if args.len() != 3 {
      return Err("\
Must provide 'Query' and 'Filename' to execute the program!
ex. 
cargo run [query] [filename]
");
    }

    let query = &args[1];
    let filename = &args[2];

    let case_sensitivity = match std::env::var("CASE_SENSITIVE") {
      Ok(val) => {
        val == "true"
      },
      Err(_) => false,
        
    };

    Ok(ParseConfig { 
        query: String::from(query), 
        filename: String::from(filename), 
        is_case_sensitive: case_sensitivity,
    })
  }
}

#[cfg(test)]
mod tests{ 
  use core::panic;

use super::*;

  #[test] 
  fn pass_if_have_exact_two_args() { 
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("--Path--"));
    args.push(String::from("qur"));
    args.push(String::from("poem.txt"));


    let data = ParseConfig::parse_config(&args).unwrap_or_else(|_err| {
      panic!("Failed to Parse!")
    });

    assert_eq!(data.query, args[1], "Query is not as Expected : {} != {}", data.query, args[1]);
    assert_eq!(data.filename, args[2]);
  }

  #[test] 
  #[should_panic(expected = "Failed to Parse!")]
  fn fail_if_have_more_or_less_args() { 
     let mut args: Vec<String> = Vec::new();
     args.push(String::from("--Path--")); // this is the default first parameter that is always passed from env.
    args.push(String::from("qur"));


    
    let _data = ParseConfig::parse_config(&args).unwrap_or_else(|_err| {
      panic!("Failed to Parse!")
    });
  }

}