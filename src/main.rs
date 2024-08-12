use rand::Rng;
use std::io;

fn main() {
    start_yubaba();
}

fn start_yubaba() {
    let mut origin_name = String::new();
    println!("契約書だよ、そこに名前を書きな。");
    io::stdin()
        .read_line(&mut origin_name)
        .expect("Failed to read line");

    origin_name = origin_name.trim().to_string();
    println!("ふん、{}というのかい、贅沢な名だね", origin_name);

    let name_length = origin_name.chars().count();
    // println!("{}", name_length);

    let new_name_index = rand::thread_rng().gen_range(0..name_length);
    // println!("{}", new_name_index);

    let new_name = origin_name.chars().nth(new_name_index).unwrap();

    println!(
        "今からお前の名前は{}だ、いいかい、わかったら返事をするんだ！{}!!",
        new_name, new_name
    )
}
