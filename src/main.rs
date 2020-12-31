extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("스무고개 게임!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("비밀번호: {}", secret_number);

    loop {
        println!("숫자를 자신있게! 입력해주세요");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{} 값을 자신있게 입력 하셨습니다!", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("너무 작아!"),
            Ordering::Greater   => println!("너무 커!"),
            Ordering::Equal     => {
                println!("정답!!");
                break;
            }
        }
    }
}
