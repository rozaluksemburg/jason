// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)



use std::io; // Импортируем модуль для работы с вводом-выводом

// Перечисление возможных состояний питания
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

// Функция для преобразования пользовательского ввода в PowerState
fn parse_input(input: &str) -> Option<PowerState> {
    let input_lower = input.to_lowercase(); // Приводим ввод к нижнему регистру
    match input_lower.as_str() { // Сравниваем строку с возможными значениями
        "off" => Some(PowerState::Off),
        "sleep" => Some(PowerState::Sleep),
        "reboot" => Some(PowerState::Reboot),
        "shutdown" => Some(PowerState::Shutdown),
        "hibernate" => Some(PowerState::Hibernate),
        _ => None, // Если ввод не соответствует ни одному из вариантов, возвращаем None
    }
}

// Функция для вывода сообщения на основе состояния питания
fn print_power_message(state: PowerState) {
    match state {
        PowerState::Off => println!("Turning off"), // Выключение
        PowerState::Sleep => println!("Going to sleep"), // Переход в режим сна
        PowerState::Reboot => println!("Rebooting"), // Перезагрузка
        PowerState::Shutdown => println!("Shutting down"), // Завершение работы
        PowerState::Hibernate => println!("Hibernating"), // Гибернация
    }
}

fn main() {
    println!("Введите состояние питания (Off, Sleep, Reboot, Shutdown, Hibernate):");

    let mut input = String::new(); // Создаем строку для ввода пользователя
    io::stdin().read_line(&mut input).expect("Не удалось прочитать ввод"); // Считываем ввод

    let input = input.trim(); // Удаляем возможные пробелы в начале и конце строки

    match parse_input(input) {
        Some(state) => print_power_message(state), // Если получаем Some(state), выводим сообщение
        None => println!("Ошибка: неверное состояние питания"), // Если получаем None, выводим сообщение об ошибке
    }
}








// fn _add_fn(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {
//     // let sum1 = add(1, 1);
//     // println!("{:?}", sum);

//     // замыкание пошло
//     // let add = |a: i32, b: i32| -> i32 {
//     //     a + b
//     // };


       
//         let _add1 = |a: i32, b: i32| -> i32 {
//             a + b
//         }; // делает то же самое 
//         let add = |a, b| a + b; // что и здесь 
    
//         let sum = add(1, 1);
//         println!("{:?}", sum);
        













//     // let add = |a, b| a + b;
//     //  let sum = add(1, 3);
//     //  println!("{:?}", sum);

//     //  let x = 10;
//     //  // замыкание захватывает переменную x из окружения 
//     //  let closure = |y| x + y;

//     //  // вызов замыкания с аргументом 5
//     //  println!("Результат замыкания: {}", closure(5));

// }











// fn apply<F>(f: F, a: i32, b: i32) -> i32
//     where
//     F: Fn(i32, i32) -> i32,
//     {
//         f(a, b)
//     }

//     fn main() {
//         let add = |a, b| a + b;
//         let result = apply(add, 1, 1);
//         println!("{}", result);

//     }

/*
я уехал в город Ебург 
потому что я филолог 
если был бы математик 
то уехал бы в Саратов 

там в Саратове раздолье 
людям, занятым любовью
с математикой подругой
или музыкой женой,
где Есенин под Калугой 
ходит мертвый, как живой 




где 
и любовницей в одном 
теле 





*/






// о замыканиях коротко 
// fn main() {
//     let add = |a: i32, b: i32| {
//         a + b
//     };

//     let add2 = |a, b| {
//         a + b
//     };

//     let result = add(1, 1);    
//     println!("{}", result);

//     let result3 = add2(1, 2);
//     println!("{}", result3);
// }
