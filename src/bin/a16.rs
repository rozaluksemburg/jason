// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// Option<T> может быть как в enum Option<T>






struct Student {
    name: String,
    locker_number: Option<i32>
}

fn main() {
    let piter = Student {
        name: "piter".to_owned(),
        locker_number: Some(23)
    };
    println!("student: {:?}", piter.name);
    match piter.locker_number {
        Some(no_ok) => println!("номер шкафчика {:?}", no_ok),
        None => println!("шкафчиком не обзавелся"),
    }

    let sous = Student {
        name: "cake".to_owned(),
        locker_number: None
    };
    println!("student: {:?}", sous.name);
    match sous.locker_number {
        Some(okey) => println!("номер шкафчика: {:?}", okey),
        None => println!("шкафчиком не обзавелся")
    }



}






/*
struct TypeAnswers {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>
}

fn main() {
    let answers = TypeAnswers {
       q1: Some(24),
        q2: Some(true),
        q3: Some("Олеся".to_owned())
    };
    match answers.q1 {
        Some(age) => println!("Не полных лет: {:?}", age),
        None => println!("q1 нет возраста у женщины - увы!")
    }
    match answers.q2 {
        Some(age) => println!("Не полных лет: {:?}", age),
        None => println!("q2 нет возраста у женщины - увы!")
    }
    match answers.q3 {
        Some(age) => println!("Не полных лет: {:?}", age),
        None => println!("q3 нет возраста у женщины - увы!")
    }

}
*/




























/*
struct Servey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>
}

fn main() {
    let answers = Servey {
        q1: None,
        q2: Some(true),
        q3: Some("Блез Паскаль".to_owned())
    };
    match answers.q1 {
        Some(years) => println!("Лет: {:?}", years),
        None => println!("q1 no response")
    }
    match answers.q2 {
        Some(years) => println!("Лет: {:?}", years),
        None => println!("q2 no response")
    }
    match answers.q3 {
        Some(years) => println!("Лет: {:?}", years),
        None => println!("q3 no response")
    }
}



*/


















/*
struct Student {
    name: String,
    number: Option<i32>
}

fn main() {
    let students = vec![
        Student {
            name: "Петька".to_owned(),
            number: Some(3)
        },
        Student {
            name: "Зося Писичкина".to_owned(),
            number: None
        }
    ];
    for learner in students {
        println!("name student: {:?}", learner.name);
        match learner.number {
            Some(number) => {
                println!("locker number: {:?}", number)
            },
            None => println!("locker number: не насосала")
        }
    }
}
*/




/* хорошая программа поиска товара
use std::io;
struct ProductSearch {
    name: String,
    units: u32
}

fn product_search(query: &str) -> Option<u32> {
    let tovary_vnalichii = vec![
        ProductSearch {
            name: "Бананы".to_owned(),
            units: 24
        },
        ProductSearch {
            name: "Киви".to_owned(),
            units: 19
        },
    ];
    for search in tovary_vnalichii {
        if search.name == query {
            return Some(search.units);
        }
    }
    None
}

fn main() {
    let mut search_user = String::new();
    io::stdin().read_line(&mut search_user).expect("что-то не то");
    let search_user = search_user.trim().to_owned();


    let search_result = product_search(&search_user);
    match search_result {
        Some(units) => {
            println!("{:?} имеются в наличии в количестве {:?} ", search_user, units);
        }
        None => println!("{:?} нет в наличии", search_user),
    }
}

*/














/*
в большом Господнем доме
все кажется знакомо
все кажется избытком
но ты ведь не из тех, кто
выполнял за Бога работу по ошибкам


собрал по звездам небо, работу по ошибкам,



собирал по звездам, как будто бы по ниткам
распутывая небо,

галактики реестр

из предрассудков выткан




 */




/*
struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
   let mark = Customer {
       age: Some(22),
       email: "mark@gmail.com".to_owned(),
   };
    let julia = Customer {
        age: None,
        email: "julia".to_owned()
    };
    match julia.age {
        Some(age) => println!("Джулии всего-то {:?}", age),
        None => println!("Джули не предоставила данные о возрасте"),
    }
}
*/

/*

в этом месте всегда обрывается связь



если видишь забор, то конечно же крась





struct GroceryItem {
    name: String,
    qty: i32
}

fn find_quantity(name: &str) -> Option<i32> {
    GroceryItem {

    }

}

*/















/*
struct GroceryItem {
    name: String,
    quantity: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name: "bananas".to_owned(), quantity: 4 },
        GroceryItem { name: "eggs".to_owned(), quantity: 12 },
        GroceryItem { name: "bread".to_owned(), quantity: 1 }
    ];
    for item in groceries {
        if item.name == name {
            println!("яйца имеются в количестве: {:?}", item.quantity );
            return Some(item.quantity);
        }
    }
    None
}

fn main() {
    find_quantity("eggs");
}
*/

/*
enum Option<T> {
    Some(T),
    None
}
*/

/*
struct Customer {
    age: Option<i32>, //
    email: String
}

fn main() {
    let _mark = Customer {
        age: Some(22),
        email: "mark@mail.ru".to_owned(),
    };

    let violetta = Customer {
        age: None,
        email: "violetta".to_owned(),
    };

    match violetta.age {
        Some(age) => println!("ты помолодела на {:?} лет ", age),
        None => println!("customer age not provided"),
    }

}
*/

/*
я нажимаю loop ты нажимаешь play
кольцуется





и вот я Одиссей танцую меж церцей





 */