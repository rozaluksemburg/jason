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



/// Ensures n is >= lower and <= upper.
/// Убедитесь, что n >= нижнее и <= верхнее.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {
    println!("{}", clamp(5, 9, 7));
    println!("{}", clamp(5000, 100, 1000));
    println!("{}", concat("Dionis Ionesko", "dollar millionaire"));
    println!("{:?}", div(1, 1));
}
// обязательно написать тест как в 80 видео уроке
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_lower() {
        let start = clamp(10, 100, 1000); 
        let end = 100;
        assert_eq!(start, end, "n < lower не срабатывает"); 
    }
    #[test]
    fn clamp_upper() {
        let start = clamp(5000, 100, 1000);
        let end = 1000;
        assert_eq!(start, end, "n < upper не срабатывает");
    }
    #[test] 
    fn clamp_n() {
        let start = clamp(5000, 100, 7000);
        let end = 5000;
        assert_eq!(start, end, "n не срабатывает");
    }
    #[test]
    fn div_check() {
        let start = div(1, 1);
        let end = Some(1);
        assert_eq!(start, end, "should be 1");    

    }
    #[test]
    fn check_div_zero() {
        let start = div(1, 0);
        let end = None;
        assert_eq!(start, end, "should be None");
    }



    #[test]
    fn concat_check() {
        let start = concat("Dionis Ionesko", "dollar millionaire");
        let end = String::from("Dionis Ionesko dollar millionaire");
        assert_eq!(start, end, "should be Dionis Ionesko dollar millionaire");
    }
    





}








