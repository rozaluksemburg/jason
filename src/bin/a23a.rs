fn part_1() -> bool {
    maybe_access("admin").is_some()
}

fn maybe_access(user: &str) -> Option<Access> {
    match user {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None
}
}

enum Access {
    Admin,
    User,
    Guest
}

fn part_2() -> Option<Access> {
    maybe_access("root").or_else(|| root())
} // то есть maybe_access ничего не возвращает, потому что в ней нет root 
// и поэтому or_else подставляет вместо None результат функции root() 

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn part_3() -> Access {
    maybe_access("Alice").unwrap_or_else(|| Access::Guest)
}

fn main() {
    // Эта функция называется `main`. В Rust `main` является входной точкой любой исполняемой программы.
    // Когда вы запускаете программу, выполнение начинается именно с этой функции.
    // Здесь она пустая, потому что наше задание не требует выполнения кода при запуске программы.
    // Обычно здесь пишется основной исполняемый код программы.
}

// #[cfg(test)]
// mod test {
//     // Этот модуль содержит тесты для проверки правильности работы нашего кода.
    
//     use crate::*; // Импортируем все из текущего модуля верхнего уровня, чтобы использовать внутри тестов.

//     #[test]
//     fn check_part_1() {
//         // Проверяем, что функция part_1 возвращает true для пользователя "admin".
//         // `assert_eq!` сравнивает результат выполнения функции `part_1()` с ожидаемым значением (`true`).
//         // Если они не равны, тест не пройдет, и будет показано сообщение "Admins have an access level".
//         assert_eq!(part_1(), true, "Admins have an access level");
//     }

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_part_1() {
        assert_eq!(part_1(), true, "Admin красавчик"); 
    }

    #[test]
    fn check_part_2() {
        assert_eq!(
            part_2(),
            Some(Access::Admin),
            "Root users have Admin access"

        );
    }
}







    #[test]
    fn check_part_2() {
        // Проверяем, что функция part_2 возвращает Some(Access::Admin) для пользователя "root".
        // `assert_eq!` сравнивает результат выполнения функции `part_2()` с ожидаемым значением (`Some(Access::Admin)`).
        // Если они не равны, тест не пройдет, и будет показано сообщение "Root users have Admin access".
        assert_eq!(
            part_2(),
            Some(Access::Admin),
            "Root users have Admin access"
        );
    }

    #[test]
    fn check_part_3() {
        // Проверяем, что функция part_3 возвращает Access::Guest для пользователя "Alice".
        // `assert_eq!` сравнивает результат выполнения функции `part_3()` с ожидаемым значением (`Access::Guest`).
        // Если они не равны, тест не пройдет, и будет показано сообщение "Alice is a guest".
        assert_eq!(part_3(), Access::Guest, "Alice is a guest");
    }
}


