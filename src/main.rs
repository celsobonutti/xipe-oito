use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

fn main() {
  let a = 'a';
  let b = &a.to_string()[..];
  println!("{}", a);
}
