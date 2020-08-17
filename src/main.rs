use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

fn main() {
    let a: [u8; 2] = [0b11111111, 0b11111111];
    let instruction = (&a[0..2]).read_u16::<BigEndian>().unwrap();
    let pos = 0;

    println!("{}", instruction);
    println!("{}", a[0]);
    println!("{:?}", &a[pos..pos+2]);
}
