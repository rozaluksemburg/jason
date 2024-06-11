// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Создайте структуру с именем "Adult", которая представляет человека в возрасте 21 года и старше:
// * Структура должна содержать имя и возраст человека
// * Реализуйте функциональность отладочной печати с помощью `derive`


struct Adult {
    age: u8,
    name: String
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("<21")
        }
    }
}

fn main() {
    let girl = Adult::new(21, "Jenis");
    match girl {
        Ok(i) => println!("name: {}\nage: {}", i.name, i.age),
        Err(e) => println!("{e}")
    }
}


/*
я на поезде ехал с Петрушкиным 
он тогда уже был почти Пушкиным
это было понятно и так




*/






















/*
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("<21")
        }
    }
}

fn main() {
    let girl = Adult::new("Luda", 19);
    let woman = Adult::new("Lucy", 22);

    match girl {
        Ok(i) => println!("name: {}\nage: {}", i.name, i.age),
        Err(e) => println!("{e}")
    }

    match woman {
        Ok(i) => println!("name: {}\nage: {}", i.name, i.age),
        Err(e) => println!("{e}")
    }

}
*/











































/*
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("<21")
        }
    }
}

fn main() {
    let woman = Adult::new(32, "Julia");
    let girl = Adult::new(19, "Lavanda");

    match woman {
        Ok(good_woman) => println!("name: {}\nage: {}", good_woman.name, good_woman.age),
        Err(e) => println!("{e}")
    }

    match girl {
        Ok(good_woman) => println!("name: {}\nage: {}", good_woman.name, good_woman.age),
        Err(e) => println!("{e}")
    }
}

*/













/*
#[derive(Debug)]
struct Adult {
    name: String,
    age: u8
}

impl Adult {
    fn new(name: String, age: u8) -> Result<Adult, String> {
        if age >= 21 {
            Ok(Adult {name, age})
        } else {
            Err(String::from("зеленая еще"))
        }
    }

}

fn main() {
    let piter = Adult::new("Piter".to_owned(), 19);

    let ledy = Adult::new("Violetta".to_owned(), 49);

    match piter {
        Ok(pi) => println!("имя: {}\nвозраст: {:?}", pi.name, pi.age),
        Err(e) => println!("{}", e)
    }

    match ledy {
        Ok(pi) => println!("имя: {}\nвозраст: {:?}", pi.name, pi.age),
        Err(e) => println!("{}", e)
    }


}

*/





















/*
#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

// Реализация методов для структуры `Adult`
impl Adult {
    // Создаем новый экземпляр взрослого человека
    fn new(name: String, age: u8) -> Result<Adult, String> {
        if age >= 21 {
            Ok(Adult { name, age })
        } else {
            Err(String::from("The person must be aged 21 or older!"))
        }
    }
}

fn main() {
    // Создаем новые структуры `Adult`
    let john_adult = Adult::new(String::from("John"), 22);
    let sam_adult = Adult::new(String::from("Sam"), 20);

    // Для каждой структуры "Взрослый" печатаем соответствующее сообщение:
    match john_adult {
        Ok(adult) => println!("Adult created successfully: {:?}", adult),
        Err(e) => println!("Error: {}", e)
    }

    match sam_adult {
        Ok(adult) => println!("Adult created successfully: {:?}", adult),
        Err(e) => println!("Error: {}", e)
    }
}

// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message
// * Используйте "сопоставление", чтобы распечатать сообщение для каждого "взрослого":
// * Для варианта "Ок" распечатайте любое сообщение, которое вы хотите
// * Для варианта "Ошибка" распечатайте сообщение об ошибке
*/
















/*
#[derive(Debug)]
enum Condition {
    Play,
    Stop,
    Pause
}

fn end_condition(go: &str) -> Result<(), String> {
    let i = first_condition(go)?;
    middle_condition(i);
    Ok(())
}

fn first_condition(name: &str) -> Result<Condition, String> {
    match name {
        "play" => Ok(Condition::Play),
        "stop" => Ok(Condition::Stop),
        "pause" => Ok(Condition::Pause),
        _ => Err("ошибочка".to_owned())
    }
}

fn middle_condition(con_clear: Condition) {
    println!("{:?}", con_clear);
}

fn main() {
    let _ = end_condition("play");
}

*/





















/*
#[derive(Debug)]
enum Choice {
    Main,
    Start,
    Quit
}

fn print_get(name: &str) -> Result<Choice, String> {
    match name {
        "main" => Ok(Choice::Main),
        "start" => Ok(Choice::Start),
        "quit" => Ok(Choice::Quit),
        _ => Err("ошибочка".to_owned())

    }
}

fn print_clear(clear: Choice) {
    println!("{:?}", clear)
}

fn print_pick(pick: &str) -> Result<(), String> {
    let i = print_get(pick)?;
    print_clear(i);
    Ok(())

}




fn main() {
    let _ = print_pick("main");
}

*/

















/*
#[derive(Debug)]
enum Choice {
    Menu,
    Start,
    Quit
}

fn print_get(name: &str) -> Result<Choice, String> {
    match name {
        "mainmenu" => Ok(Choice::Menu),
        "start" => Ok(Choice::Start),
        "quit" => Ok(Choice::Quit),
        _ => Err("ошибочка".to_owned()),
    }
}

// эта функция тоже ничего не возвращает просто это не указано явно,
// и значение тоже самое что и в функции ниже
fn print_out(tunes: &Choice) {
    println!("{:?}", tunes);
}

// Result<(), _> скобки () обозначают что функция ничего не возвращает - тут явно указываем
fn pick_choice(input: &str) -> Result<(), String> {
    let choice = print_get(input)?; // ? выполняет тоже что и match ниже
    print_out(&choice);
    Ok(())

}

fn main() {
    let _ = pick_choice("mainmenu");



}

*/





/*
ты отдалась под клофелином
тому кто не смотрел фелини
верней фелини он смотрел
но далеко был не фелини

*/
















/*
#[derive(Debug)]
enum MainChoice {
    MenuMain,
    Start,
    Quit,
}

fn print_get(name: &str) -> Result<MainChoice, String> {
    match name {
        "mainmenu" => Ok(MainChoice::MenuMain),
        "start" => Ok(MainChoice::Start),
        "quit" => Ok(MainChoice::Quit),
        _ => Err("ошибочка".to_owned())
    }
}

fn print_choice(choice: &MainChoice) {
    println!("{:?}", choice);
}

fn main() {
    let mainmenu = print_get("mainmenu");
    match mainmenu {
        Ok(clear_menu) => print_choice(&clear_menu),
        Err(e) => println!("{:?}", e),
    }
}
*/

/*
осязаема как вечер
ты идешь

 */



















































/*
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit)
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn main() {
    let choice: Result<MenuChoice, _> = get_choice("mainmenu");
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("{:?}", e)
    }


   // println!("choice = {:?}", choice);

}
*/



/*
fn sound_cash(name: &str) -> Result<String, String> {
    if name == "alert" {
        Ok("норм".to_owned())
    } else {
        Err("не норм".to_owned())
    }
}

fn main() {
    let sound = sound_cash("alert");
    match sound {
        Ok(_) => println!("все ok"),
        Err(e) => println!("все не ок потому что {:?}", e)
    }
}
*/
















/*

fn get_sound(name: &str) -> Result<String, String> {// здесь мы ждем выходное значения с одним и
// тем же типом данных String в данном случае
    if name == "alert" {
        Ok("alert".to_owned())
    } else {
        Err("mistake".to_owned())
    }
}
// то есть Result можно использовать как в структурах так и в сигнатурах функций

fn main() {
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("Дионис Ионеско мульти долларовый миллионер"),
        Err(e) => println!("big mistakes {:?}", e)
    }
}

*/















/*
fn get_sound(name: &str) -> Result<String, String> {
    if name == "alert" {
        Ok("alert".to_owned())
    } else {
        Err("ошибочка".to_owned())
    }
}

fn main() {
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("music works"),
        Err(e) => println!("error: {:?}", e), // выводим причину ошибки
    }
}
*/