use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use pwhash::bcrypt;
//use pwhash::unix;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  
  let hashed_passwd = bcrypt::hash("password").unwrap();
  println!("password hashed: {}", hashed_passwd);
  assert_eq!(bcrypt::verify("password", &hashed_passwd), true);

  //let unix_hash_test = "$y$j9T$p4cf7dtrBK3rLMAP5uAEK0$1SVQKLujPJyYraDEAYjVOTNOIlNif/kbIjIYM9bQKM9";
  //assert_eq!(unix::verify("123456789", unix_hash_test), true);

  if let Ok(lines) = read_lines("./src/dictionary.txt") {
    for line in lines {
      if let Ok(plain_text) = line {
        println!("{}", plain_text);
      }
    }
  }
}
