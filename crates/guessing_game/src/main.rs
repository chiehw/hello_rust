use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
  let secret_number = rand::thread_rng().gen_range(0..100);
  println!("[*] Guess the number!");
  println!("[*] The secret number is: {}", secret_number);
  println!("[*] Please input your guess");

  let mut guess = String::new();
  loop {
    guess.clear();
    // 在同一块内存上持续读入会出错，还可以把 String::new() 放在循环内。
    io::stdin().read_line(&mut guess).expect("[*] Failed to read line");
    let guess = match guess.trim().parse::<i32>() {
      Ok(num) => num,
      Err(_) => {
        println!("[*] Please input number!");
        continue;
      }
    };
    println!("[*] You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Equal => {
        print!("Equal, You win.");
        break;
      }
      Ordering::Less => {
        println!("Less")
      }
      Ordering::Greater => {
        println!("Greater")
      }
    }
  }
}
