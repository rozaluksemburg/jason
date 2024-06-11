// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info



enum Ticket {
    Backstage(u32, String),
    Standard(u32),
    Vip(u32, String)
}

fn main() {
    let ticket = vec![
        Ticket::Backstage(12000, "Arnold".to_owned()),
        Ticket::Standard(5000),
        Ticket::Vip(8000, String::from("Bell"))
    ];

    for tickets in ticket {
        match tickets {
            Ticket::Backstage(price, holder) => {
                println!("Билет за кулисами (backstage) стомостью {:?} принадлежит {:?}", price, holder)
            }
            Ticket::Standard(price) => println!("Стандартный билет стоимостью {:?} принадлежит всем и никому и потому безымянный", price),
            Ticket::Vip(price, holder) => println!("Билет за кулисами (vip) стомостью {:?} принадлежит {:?}", price, holder)
        }
    }

}













/*
struct BackstageVip {
    name: String,
    price: u32
}
л
struct OnlyPrice {
    price: u32,
}

enum Tickets {
    Backstage(BackstageVip),
    Vip(BackstageVip),
    Standart(OnlyPrice)
}

fn main() {
    let ticket = vec![
        Tickets::Backstage(BackstageVip { name: String::from("Jess"), price: 12000 }),
        Tickets::Vip(BackstageVip { name: "Anna".to_string(), price: 10000 }),
        Tickets::Standart(OnlyPrice { price: 5000 }),
    ];

    for tics in ticket {
        match tics {
            Tickets::Backstage(p) => println!("Билет стоимостью {:?} типа Backstage вручается {:?} , ", p.name, p.price),
            Tickets::Vip(r) => println!("Билет стоимостью {:?} типа Vip вручается {:?} ", r.name, r.price),
            Tickets::Standart(a) => println!("Билет стоимостью {:?} типа Standart безымяннен ", a.price)
        }
    }

}

*/




/*

свет падает на снег, снег падает на свет
таков у них обычай

когда нас брали ночью и стаскивали в дождь
и мы лежали с кем-то



 */









/*
struct Ticket {
    name: String,
    price: u32,
}

enum Tickets {
    Backstage(Ticket),
    Vip(Ticket),
    Standart(u32)
}

fn main() {
    let ticket = vec![
        Ticket {
            name: String::from("Arnold"),
            price: 12000,
        },
        Ticket {
            name: String::from("Julia"),
            price: 10000,
        },
        Ticket {
            name: String::from("Pussy"),
            price: 5000,
        }

    ];

    for ti in ticket {
        match ti {
           Ticket { price: 12000, name } => println!("Билет типа Backstage ценой 12000 для {:?}", name ),
            Ticket { price: 10000, name} => println!("Билет типа Vip ценой 10000 для {:?}", name ),
            Ticket { price: 5000, .. } => println!("Билет типа Standart стоимостью 5000"),
            _ => (),
        }
    }
}
*/



//
// enum Discount {
//     Percent(i32),
//     Flat(i32),
// }
//
// struct Ticket {
//     event: String,
//     price: i32,
// }
//
// fn main() {
//     let n = 3;
//     match n {
//         3 => println!("three"),
//         _ => println!("number: {:?}", n),
//     }
//
//     let discount = Discount::Flat(2);
//     match discount {
//         Discount::Flat(2) => println!("Fix discount: 2"),
//         Discount::Flat(amount) => println!("получите распишитесь: {:?}", amount),
//         _ => (),
//     }
//
//     let ticket = Ticket {
//         event: "концерт".to_owned(),
//         price: 45,
//     };
//     match ticket {
//         // здесь мы устанавливаем цену и берем событие event из данных выше концерт где созданный как новая строка методом to_owned()
//         Ticket { price: 49, event } => println!("события за 50 = {:?}", event),
//         // далее просто выводим цену двоеточие .. игнорирует другие данные
//         Ticket { price, .. } => println!("price = {:?}", price),
//     }
//
//





//
//
//     // то есть мы можем присвоить переменной одно из значений
//     // перечисления и плюсом вложить в него конкретное значение
//     // которое соответствует типу данных пункта перечисления
//     // на 18 строчке кода Flat(i32) i32 соотствует 2
//     let flat = Discount::Flat(2);
//     // выше мы присвоили Flat(2) и теперь перебираем значения в виде пункта перечисления Discount
//         match flat {
//     // сначала мы проверяем значение сравнивая значение с цифрой 2
//     // если значение строго равно 2 то выводится flat 2
//         Discount::Flat(2) => println!("flat 2"),
//     // если у переменной flat любое другое значение то оно выводится
//         Discount::Flat(amount) => println!("скидка: {:?}", amount),
//         _ => (), // заглушка
//     }
//
//     let concert = Ticket {
//         event: "conсert".to_owned(),
//         price: 50
//     };
//
//     match concert {
//         Ticket { price: 50, event } => println!("event 50 = {:?}", event),
//         Ticket { price, .. } => println!("price = {:?}", price),
//
//     }
//
//
// }
/*
то есть обращаемся через match к чему угодно, например,
к перечислениям
 */

/*
enum Discount {
    Percent(f32),
    Fixed(i32)
}
fn main() {
    let disc = Discount::Fixed(2);

    match disc {
        Discount::Fixed(amount) =>
            println!("Fixed discount of amount {}", amount),
        Discount::Percent(_other) => (),
    }
}
*/

//     let n = 3;
// // match используется для сопоставления с образцами значения переменной n в данном варианте
//     match n {
//         3 => println!("в точку g"),
//         _other => println!("Ваше число {} ", _other),
//     }
//
//     enum Mouse {
//         LeftClick,
//         RightClick,
//         MiddleClick,
//         Scroll(i32),// + вверх - вниз
//         Move(i32, i32) // положение мыши по оси x y
//     }
//
//     enum PromoDiscount {
//         NewUser,
//         Holiday(String)
//     }
//
//     enum Discount {
//         Percent(f64),
//         Flat(i32),
//         Promo(PromoDiscount),
//         Custom(String),
//     }
// }

// enum Position {
//     Manager,
//     Supervisor,
//     Worker,
// }
//
// struct Employee {
//     position: Position,
//     work_hours: i16,
// }
//
// fn main() {
//     let me = Employee {
//         position: Position::Manager,
//         work_hours: 11,
//     };
//
//     match me.position {
//         Position::Manager => println!("")
//     }
// }









/*
Роберто Карлос легший на газон 
припоминает что-то из времен 

зависли бутсы 



*/


// #[derive(Clone, Copy, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point1 = Point { x: 10, y: 20 };
//     // создаем копию point1 
//     let point2 = point1; 
//     // это возможно благодаря трейту Copy
//     println!("point1: {:?}\npoint2: {:?}", point1, point2);

// }
