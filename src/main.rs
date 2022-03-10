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

fn main() -> io::Result<()> {
  
  let hashed_passwd = bcrypt::hash("somethingelse").unwrap();
  println!("password hashed: {}", &hashed_passwd);
  assert_eq!(bcrypt::verify("somethingelse", &hashed_passwd), true);

  //let unix_hash_test = "$y$j9T$p4cf7dtrBK3rLMAP5uAEK0$1SVQKLujPJyYraDEAYjVOTNOIlNif/kbIjIYM9bQKM9"; // scrypt hashing algorithm
  //assert_eq!(unix::verify("123456789", unix_hash_test), true);

  let mut total_lines: usize = 0;
  if let Ok(lines) = read_lines("./src/dictionary.txt") {
    total_lines = lines.count();
  }

  if let Ok(lines) = read_lines("./src/dictionary.txt") {
    for (i, line) in lines.enumerate() {
      if let Ok(plain_text) = line {
        if bcrypt::verify(&plain_text, &hashed_passwd) {
          println!("{}, is the correct password. passphrase: {}/{}", &plain_text, &i + 1, &total_lines);
          break;
        }
      }
    }
  }
  Ok(())
}
