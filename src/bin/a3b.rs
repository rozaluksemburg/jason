// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    // * Use a variable set to any integer value
    let bust_size = 4;

    // * Use an if..else if..else block to determine which message to display
    if bust_size > 3 {
        println!("если не рыхлые, то вы проходите")
    } else if bust_size == 1 {
        println!("а горошины твёрдые?"); 
    } else {
        println!("ни рыба ни мясо")
    }

}



// let x = 99;
// if x > 99 {
//     if x > 200 {
//         println!("big")
//     } else {
//         println!("norm")
//     }
// } else { 
//     println!("small")
// }

// // вот как графически работает if else
// // https://skrinshoter.ru/sPdY3SrH1h5

// let a = 99;
// if a > 200 {
//     println!("hard");
// } else if a > 99 {
//     println!("medium");
// } else {
//     println!("easy");
// }