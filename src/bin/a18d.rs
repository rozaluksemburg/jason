// Topic: Result & the question mark operator
//
// Summary:
//   This small program simulates unlocking a door using digital keycards
//   backed by a database. Many errors can occur when working with a database,
//   making the question mark operator the perfect thing to use to keep
//   the code managable.
//
// Requirements:
// * Write the body of the `authorize` function. The steps to authorize a user
//   are:
//     1. Connect to the database
//     2. Find the employee with the `find_employee` database function
//     3. Get a keycard with the `get_keycard` database function
//     4. Determine if the keycard's `access_level` is sufficient, using the
//        `required_access_level` function implemented on `ProtectedLocation`.
//        * Higher `access_level` values grant more access to `ProtectedLocations`.
//          1000 can access 1000 and lower. 800 can access 500 but not 1000, ...
// * Run the program after writing your `authorize` function. Expected output:
//     Ok(Allow)
//     Ok(Deny)
//     Err("Catherine doesn't have a keycard")
// * Use the question mark operator within the `authorize` function.
//
// Notes:
// * Only the `authorize` function should be changed. Everything else can remain
//   unmodified.

// Тема: Результат и оператор вопросительного знака
//
// Резюме:
// Эта небольшая программа имитирует отпирание двери с помощью цифровых карточек-ключей.
// с использованием базы данных. При работе с базой данных может возникнуть множество ошибок,
// что делает оператор вопросительного знака идеальным средством для сохранения
// код управляемым.
//
// Требования:
// * Напишите тело функции `authorize`. Шаги для авторизации пользователя
// следующие:
// 1. Подключиться к базе данных
// 2. Найти сотрудника с помощью функции базы данных `find_employee`.
// 3. Получить ключ-карту с помощью функции базы данных `get_keycard`.
// 4. Определяем, достаточен ли `уровень_доступа` ключ-карты, используя функцию
// функцию `required_access_level`, реализованную на `ProtectedLocation`.
// * Более высокие значения `уровня доступа` предоставляют больший доступ к `Защищаемым местам`.
// 1000 может получить доступ к 1000 и ниже. 800 может получить доступ к 500, но не к 1000, ...
// * Запустите программу после написания функции `authorize`. Ожидаемый результат:
// Ok(Allow)
// Ok(Deny)
// Err(«У Екатерины нет карточки»)
// * Используйте оператор вопросительного знака внутри функции `authorize`.
//
// Примечания:
// * Только функция `authorize` должна быть изменена. Все остальное может оставаться
// не модифицированным.



#[derive(Clone, Copy, Debug)] // скажи почему тут именно эти операции обозначены
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
} 

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            ProtectedLocation::All => 1000,
            ProtectedLocation::Office => 800,
            ProtectedLocation::Warehouse => 500
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Database)
    }

    fn find_employee(&self, employee_name: &str) -> Result<Employee, String> {
        match employee_name {
            "Anita" => Ok(Employee { name: "Anita".to_string() }),
            "Brody" => Ok(Employee { name: "Brody".to_string() }),
            _ => Err(String::from("шпион без карты и ключа"))
        }
    }

    fn employee_to_number(&self, blondy: &Employee) -> Result<KeyCard, String> {
        match blondy.name.as_str() { // здесь происходит доступ к имени структуры Employee
            "Anita" => Ok(KeyCard { access_level: 1000 }),
            "Brody" => Ok(KeyCard { access_level: 500 }),
            other => Err(format!("{other} не имеет карточки"))
        }
    }
}

#[derive(Clone, Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(employee_name: &str, location: ProtectedLocation) -> Result<AuthorizationStatus, String> {
    let database = Database::connect()?;
    let employee_choice = database.find_employee(&employee_name)?;
    let employee_number = database.employee_to_number(&employee_choice)?; // а почему тут есть & а выше нет? 
    if employee_number.access_level >= location.required_access_level() {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }


}


fn main() {
    let anita_auth = authorize("Anita", ProtectedLocation::All);
    println!("{anita_auth:?}");
    let brody_auth = authorize("Brody", ProtectedLocation::Office);
    println!("{brody_auth:?}");
    let cat_auth = authorize("Cat", ProtectedLocation::Warehouse);
    println!("{cat_auth:?}");

}









































// #[derive(Clone, Copy, Debug)]
// enum ProtectedLocation {
//     All,
//     Office,
//     Warehouse,
// }

// impl ProtectedLocation {
//     fn required_access_level(&self) -> u16 {
//         match self {
//             Self::All => 1000,
//             Self::Office => 800,
//             Self::Warehouse => 500,
//         }
//     }
// }

// #[derive(Debug)]
// struct Database;

// impl Database {
//     fn connect() -> Result<Database, String> {
//         Ok(Database)
//     }
// // слово self ниже относится к Database что через нее работает 
//     fn find_employee(&self, name: &str) -> Result<Employee, String> {
//         match name {
//             "Anita" => Ok(Employee {
//                 name: "Anita".to_string(),
//             }),
//             "Brody" => Ok(Employee {
//                 name: "Brody".to_string(),
//             }),
//             "Cat" => Ok(Employee {
//                 name: "Cat".to_string()
//             }),
//             _ => Err(String::from("работник не найден")),
//         }
//     }

//     fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
//         match employee.name.as_str() {
//             "Anita" => Ok(KeyCard { access_level: 1000 }),
//             "Brody" => Ok(KeyCard { access_level: 500 }),
//             other => Err(format!("{other} doesn't have a keycard")),

//         }

//     }


// }

// #[derive(Debug)]
// struct KeyCard {
//     access_level: u16,
// }

// #[derive(Clone, Debug)]
// struct Employee {
//     name: String,
// }

// #[derive(Clone, Copy, Debug)]
// enum AuthorizationStatus {
//     Allow,
//     Deny,
// }


// fn authorize(employee_name: &str, location: ProtectedLocation) -> Result<AuthorizationStatus, String> {
//     let database = Database::connect()?;
//     let employee = database.find_employee(employee_name)?;
//     // получаем что employee = структуре Employee { name: "имя снизу".to_string() преобразованное в String }
//     let keycard = database.get_keycard(&employee)?;
//     // keycard = структуре KeyCard { access_level: 1000(например в зависимости от поступившего
//     // имени или по-правильному ключа - например Anita или Brody или если другое имя, то пользователь не имеет ключа }
//     if keycard.access_level >= location.required_access_level() {
//         Ok(AuthorizationStatus::Allow)
//     } else {
//         Ok(AuthorizationStatus::Deny)
//     }

// }

// fn main() {
//     let anita_auth = authorize("Anita", ProtectedLocation::All);
//     println!("{anita_auth:?}");
//     let brody_auth = authorize("Brody", ProtectedLocation::Office);
//     println!("{brody_auth:?}");
//     let cat_auth = authorize("Cat", ProtectedLocation::Warehouse);
//     println!("{cat_auth:?}");

// }




