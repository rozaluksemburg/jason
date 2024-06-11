fn part_1() -> bool {
    maybe_access("admin").is_some()
}

fn maybe_access(status: &str) -> Option<AccessLevel> {
    match status {
        "admin" => Some(AccessLevel::Admin),
        "gary" => Some(Access::User),
        _ => None
    } 
}

enum AccessLevel {
    Admin,
    User,
    Guest
}

fn part_2() -> Option<AccessLevel> {
    maybe_access("root").or_else(|| root())
}

fn root() -> Option<AccessLevel> {
    Some(AccessLevel::Admin)
} // то есть maybe_access ничего не возвращает, потому что в ней нет root 
// и поэтому or_else подставляет вместо None результат функции root() 

fn part_3() -> Option<AccessLevel> {
    maybe_access("Alice").unwrap_or_else(|| AccessLevel::Guest)
} // метод unwrap_or_else 


/*
все ведь примерно знают, чем все закончится
*/