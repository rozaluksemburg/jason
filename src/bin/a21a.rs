






fn maybe_word() -> Option<String>{

}


fn main() {
    let
    let word_length: Option<i32> = maybe_word()
        .map(|word| word.len())
        .map(|len| len * 2);


    // let plus_one = maybe_num().map(|num| num + 1);







}
/*
use std::io;

fn maybe_num(a: i32) -> Option<i32> {
       if a > 0 {
           Some(a)
       } else {
           None
       }
}

fn main() {
    println!("Напиши сумму, которой ты обладаешь, и я увеличу это сумму в реальной жизни сразу после ввода");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("в молоко");
    let input = input.trim();
    let number: i32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("введите корректное числовое значение");
            return;
        }
    };

    let neo = maybe_num(number).map(|num| num + 1_000_000);


    match neo {
        Some(blue) => println!("Получите {}$", blue),
        None => println!("заминусить задумали?")
    }

}

 */