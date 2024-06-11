// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// Тема: Работа с выражениями
//
// Требования:
// * Выводим «его большое», если переменная > 100
// * Выводим «она маленькая», если переменная <= 100
//
// Примечания:
// * Используйте логическую переменную, равную результату
// выражение if..else для хранения значения
// это > 100 или <= 100
// * Используем функцию для печати сообщений
// * Используйте выражение соответствия, чтобы определить, какое сообщение
//   печатать

fn print_bool(a: bool) {
    match a {
        true => println!("its big"),
        false => println!("its small"),
    }

}

fn main() {
    let x = 100;
    let is_gt_100 = x > 100;

    print_bool(is_gt_100);

    

}



//     let logic = if x > 100 {
//         true 
//     } else {
//         false
//     };

//     match logic {
//         true => println!("its big"),
//         false => println!("its small")
//     }

// }










// #[allow(dead_code)]
// enum Access {
//     Admin,
//     User,
//     Guest,
// }

// fn main() {
//     let access_level = Access::Admin;
//     let access_file = match access_level {
//         Access::Admin => true,
//         _ => false,
//     };

//     println!("{:?}", access_file);
// }






// #[allow(dead_code)]
// enum Menu {
//     Milk,
//     Water,
// }

// fn main() {
//     let item = Menu::Milk;
//     let upsell_drink = "water";
//     let order = match item {
//         Menu::Milk => if upsell_drink == "water" {
//             true
//         } else {
//             false
//         }

//         _ => true,
    
//     };

//     println!("{:?}", order);

// }



// из 3 блока




// fn main() {
//     let n = 4;

//     // первый способ 
//     let b = if n < 5 {
//         true
//     } else {
//         false
//     };
//     // а это тоже самое что выше в более лаконичном коде
//     let b = n < 5;

//     let my_num = 3;
//     let message = match my_num {
//         1 => "you win",
//         _ => "you win twice",
//     };

    // 3 блок
    // enum Menu {
    //     Milk,
    //     Water,
    // }

    // fn main() {
    //     let _paid = true;
    //     let item = Menu::Milk;
    //     let drink_type = "water";
    //     let _order_placed = match item {
    //         Menu::Milk => {
    //             if drink_type == "water" {
    //                 true
    //             } else {
    //                 false
    //             }
    //         }
    //         _ => true,
    //     };


    // }

   

// }
//     /*
    
//      */

    







// }



