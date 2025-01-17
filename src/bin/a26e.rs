// Topic: External Modules
//
// Summary:
// The existing program is complete, but all the code exists
// in a single module. This code can benefit from being organized
// into multiple external modules.
//
// Requirements:
// * Organize the code into two external modules based on their functionality:
//   - msg: string formatting functions
//   - math: math functions
// * Update the main function to use the functionality from the modules
//
// Notes:
// * Update your Cargo.toml to include a library file
// * After moving the functions into modules, try running
//   `cargo check --bin a26c` to get a listing of required code changes

// Тема: Внешние модули
//
// Резюме:
// Существующая программа завершена, но весь код существует
// в одном модуле. Этот код можно выгодно организовать
// в несколько внешних модулей.
//
// Требования:
// * Организуйте код в два внешних модуля на основе их функциональности:
// - msg: функции форматирования строк
// - math: математические функции
// * Обновите главную функцию, чтобы она использовала функциональность из модулей.
//
// Примечания:
// * Обновите ваш Cargo.toml, чтобы включить файл библиотеки.
// * После перемещения функций в модули попробуйте выполнить команду
// ``cargo check --bin a26c``, чтобы получить список необходимых изменений кода


mod msg {
fn trim(msg: &str) -> &str {
    msg.trim()
}

fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
    if let Some(letter) = msg.get(0..1) {
        format!("{}{}", letter.to_uppercase(), &msg[1..msg.len()]).into()
    } else {
        msg.into()
    }
}

fn exciting(msg: &str) -> String {
    format!("{}!", msg)
}

}

fn add(lhs: isize, rhs: isize) -> isize {
    lhs + rhs
}
fn sub(lhs: isize, rhs: isize) -> isize {
    lhs - rhs
}
fn mul(lhs: isize, rhs: isize) -> isize {
    lhs * rhs
}

fn main() {
    // Part 1: math functions
    let result = {
        let two_plus_two = add(2, 2);
        let three = sub(two_plus_two, 1);
        mul(three, three)
    };

    // Ensure we have a correct result.
    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    // Part 2: string functions
    let hello = {
        let msg = "hello ";
        let msg = trim(msg);
        capitalize(msg)
    };
    let world = {
        let msg = "world";
        exciting(msg)
    };
    let msg = format!("{}, {}", hello, world);

    // Ensure we have a correct result.
    assert_eq!(&msg, "Hello, world!");
    println!("{}", msg);
}
