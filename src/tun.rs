
use std::{fs::{self,OpenOptions}};

pub struct Tun {
  name: String, 
  file: fs::File,
}

impl Tun {
  pub fn new(name:&str) -> anyhow::Result<Self> {
    let mut options = OpenOptions::new();
    Ok(
      Self{
        name: name.to_string(),
        file: options.read(true).write(true).open("/dev/net/tun")?
      }
    )
  }
}

