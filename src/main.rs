use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数字游戏，数字范围为：1 - 100");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("很遗憾，猜小了！"),
            Ordering::Greater => println!("很遗憾，猜大了！"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！！！");
                break;
            }
        }
    }
}
