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

#[cfg(test)]
mod test {
    use crate::*; // импортирует все элементы из корневого (crate root) модуля нашей программы
    #[test]
    fn clamb_lower() {
  // Ensures n is >= lower and <= upper.
  // Убедитесь, что n >= нижнее и <= верхнее.
  let verif = clamp(10, 100, 1000);
  let lower = 100;
        assert_eq!(verif, lower, "значение должно быть равно 100");

    }
    fn clamp_upper() {
        let start = clamp(5000, 100, 1000);
        let end = 1000;
        assert_eq!(start, end, "верх не открывается");
    }
}

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

#[cfg(test)]
mod tesla {
    use crate::*;
    #[test]
    fn pop_diva () {
        assert_eq!(div(10, 10), Some(1), "not divides");
        assert_eq!(div(10, 0), None, "деление на ноль запрещено мной, но не мне");

    }

}
/// Divides a and b. делит a на b 
fn div(a: i32, b: i32) -> Option<i32> {
    a.checked_div(b) // это можно записать так a делим на b и в случае если b равно 0
        // то метод checked_div вернет значение None
}


#[cfg(test)]
mod string {
    use crate::*;
    #[test]
    fn str() {
        assert_eq!(concat("Dionis", "Ionesko"), "Dionis Ionesko", "not connect string" );
    }

}
/// Takes two strings and places them immediately one after another. Берет две строки и помещает их друг за другом.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {
    let result = clamp(10, 0, 11);
    println!("result: {}", result);
    println!("{}", concat("Dionis","Ionesko"));
    print
}
