use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}

    loop {
        println!("Please input your guess.");   // ほら、予想を入力してね

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");     // 行の読み込みに失敗しました

        let guess: u32 = guess.trim().parse().expect("Please type a number!");    // 数字を入力してください

        println!("You guessed: {}", guess);     // 次のように予想しました: {}

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),    // 小さすぎです
            Ordering::Greater => println!("Too big!"),   // 大きすぎです
            Ordering::Equal => {
                println!("You win!");     // あたりです
                break;
            }
        }
    }
}
