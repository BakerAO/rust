
fn main() {
  a();
  b();
  c();
}

fn a() {
  let x = 5;
  assert_eq!("i32".to_string(), type_of(&x));

  println!("Success!");
}

fn type_of<T>(_: &T) -> String {
  format!("{}", std::any::type_name::<T>())
}

fn b() {
  let mut sum = 0;
  for i in -3..2 {
      sum += i
  }

  assert!(sum == -5);

  for i in 'a'..='z' {
      print!("{}", i as u8);
  }
  println!();
}

fn c() {
  assert!(1u32 + 2 == 3);
  assert!(1i32 - 2 == -1);
  assert!(1i8 - 2 == -1);
  
  assert!(3 * 50 == 150);
  assert!(9.6_f32 / 3.2 == 3.0);

  assert!(24 % 5 == 4);
  assert!(true && false == false);
  assert!(true || false == true);
  assert!(!true == false);

  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}