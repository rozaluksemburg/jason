// #[derive(Debug)]
// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
// * Используйте перечисление для создания различных вкусов напитков
// * Используйте структуру для хранения информации о вкусе напитка и количестве жидкости в унциях
// * Используйте функцию для вывода вкуса напитка и количества жидкости в унциях
// * Используйте выражение соответствия для вывода вкуса напитка


enum Flavor {
    Apple,
    Vodka,
}

struct Drink {
    flavor: Flavor,
    ostatok: f32,
}

fn print_appvodka(appvodka: &Drink) {
    match appvodka.flavor {
        Flavor::Apple => println!("Яблочного запивона осталось: "),
        Flavor::Vodka => println!("Водки осталось: "),
    }

    println!("{:?} литров", appvodka.ostatok);
}

fn main() {
    let apple = Drink {
        flavor: Flavor::Apple,
        ostatok: 32.3,
    };

    let vodka = Drink {
        flavor: Flavor::Vodka,
        ostatok: 29.8,
    };

    print_appvodka(&apple);
    print_appvodka(&vodka);
}











// enum Flavor {
//     Juice,
//     Wine,
// }

// struct Drink {
//     flavor: Flavor,
//     remains: f32,
// }

// fn print_drink(drink: &Drink) {
//     match drink.flavor {
//         Flavor::Juice => println!("Сока осталось: {:?} литров", drink.remains),
//         Flavor::Wine => println!("Вина осталось: {:?} литров", drink.remains), 
//     }
// }



// fn main() {
//     let juice = Drink {
//         flavor: Flavor::Juice,
//         remains: 32.3,
//     };
//     print_drink(&juice);

//     let wine = Drink {
//         flavor: Flavor::Wine,
//         remains: 33.4,
//     };   
//     print_drink(&wine);



// }

















// enum Flavor {
//     Juice,
//     Vodka,
// }

// struct Drink {
//     flavor: Flavor,
//     fluid_ounces: f32,    
// }

// fn data_drink(outdate: &Drink) {
//     match outdate.flavor {
//         Flavor::Juice => println!("Сока осталось: {:?} литров", outdate.fluid_ounces),
//         Flavor::Vodka => println!("Водки осталось: {:?} литров", outdate.fluid_ounces),
//     };
// }

// fn main() {
//     let juice = Drink {
//         flavor: Flavor::Juice,
//         fluid_ounces: 42.3,
//     };

//     let vodka = Drink {
//         flavor: Flavor::Vodka,
//         fluid_ounces: 43.4,
//     };


//     data_drink(&juice);
//     data_drink(&vodka);


// }

















// первый не удачный вариант
// // * Use an enum to create different flavors of drinks
// enum DrinkFlavor {
//     Coffee,
//     Juice,
//     Vodka,
// }

// // * Use a struct to store drink flavor and fluid ounce information
// // * Используйте структуру для хранения информации о вкусе напитка и количестве жидкости в унции

// struct FlavorOunces {
//     Coffee: i32,
//     Juice: i32,
//     Vodka: i32,
// }

// fn main() {

// let testy = FlavorOunces {
//     Coffee: 9,
//     Juice: 3,
//     Vodka: 4,
// };

// match testy {
//     DrinkFlavor::Coffee => println!("Кофе осталось: {:?}", testy.Coffee),
//     DrinkFlavor::Juice => println!("Сока осталось: {:?}", testy.Juice),
//     DrinkFlavor::Vodka => println!("Водки осталось: {:?}", testy.Vodka),
// }


// }



// struct ZapasPrice1 {
//     stock: i32,
//     price: f64,
// }

// fn main() {

// let zapas_price2 = ZapasPrice1 {
//     stock: 10,
//     price: 3.2,
// };

// println!("Запас товара на складе: {:?}", zapas_price2.stock);
// println!("Цена за единицу: {:?}", zapas_price2.price);



// }
















// структуры нужны для объединения похожих элементов 
// struct ShippingBox { 
//     depth: i32,
//     width: i32,
//     height: i32,
// }

// fn main() {
//     let my_box = ShippingBox {
//         depth: 3,
//         width: 4,
//         height: 5,
//     };  
// // доступ к полям внутри структуры можно получить через точку .
//     let tall = my_box.width;
//     println!("{:?}", tall);


// }
