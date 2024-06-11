// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    // * Use a variable set to either true or false
    // * Используйте переменную, для которой установлено значение true или false
    let only_true = true;

    // * Use an if..else block to determine which message to display
    // * Используйте блок if..else, чтобы определить, какое сообщение отображать
    if only_true == true {
        // * Use the println macro to display messages to the terminal
        // * Используйте макрос println для вывода сообщений на терминал
        println!("если всегда только правда, то это ложь")
    } else {
        println!("в противном случае всё наоборот");
    }   
}









// * Use a variable set to either true or false
// * Используйте переменную, для которой установлено значение true или false


// fn main() {
//     let age = 15;
//     if age >= 21 {
//         println!("можно"); 
//     } else {
//         println!("зелен ещё"); 
//     }
// }
