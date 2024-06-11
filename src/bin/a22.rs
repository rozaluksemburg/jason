// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

// Требования:
// * Написать тесты для существующей программы, чтобы убедиться в ее работоспособности.
//
// Примечания:
// * Создайте как минимум два тестовых случая для каждой функции.
// * Используйте `cargo test` для тестирования программы.
// * В программе есть намеренные ошибки, которые необходимо исправить.
// * Проверьте комментарии в документации к функциям, чтобы.
// * Определите, как они должны работать.


/*
/// Ensures n is >= lower and <= upper.
/// Убедитесь, что n >= нижнее и <= верхнее.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n >= lower {
        lower
    } else if n <= upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}
*/

























use std::io;

fn input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut vector_input = vec![];
    let mut quant = 0;
    while quant < 2 {
        match input() {
            Ok(words) => {
                vector_input.push(words);
                quant += 1;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            },
        }
    }

    for gold in &vector_input {
        println!("было: {:?}, стало: {:?}", gold, gold.to_uppercase());
    }
}


























