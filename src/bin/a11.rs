// Topic: Ownership 
// Владение и заимствование 

// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


struct ShowBasket {
    quantity: u32,
    id_number: u32,
}

fn print_quantity(quant: &ShowBasket) {
    println!("количество на экран: {:?}", quant.quantity);
}

fn print_id_number(id: &ShowBasket) {
    println!("id номер на экран: {:?}", id.id_number);
}

fn main() {
    let data_basket = ShowBasket {
        quantity: 2,
        id_number: 21,
    };

    print_quantity(&data_basket); 
    print_id_number(&data_basket);
}









// no problem
// struct Shop {
//     quantity: u32,
//     id_number: u32,
// }

// fn print_quantity(quant: &Shop) {
//     println!("количество на экран: {:?}", quant.quantity);
// }

// fn print_id_number(id: &Shop) {
//     println!("id номер на экран: {:?}", id.id_number);
// }

// fn main() {
//     let food_basket = Shop {
//         quantity: 2,
//         id_number: 222,
//     };

//     print_quantity(&food_basket);
//     print_id_number(&food_basket);



// }


// struct Book {
//     pages: u32,
//     rating: f32,
// }

// // Измените функции, чтобы они принимали ссылку на Book
// fn print_pages(book: &Book) {
//     println!("страниц в книге: {}", book.pages);
// }

// fn print_rating(book: &Book) {
//     println!("рейтинг книги: {}", book.rating);
// }

// fn main() {
//     let book = Book { 
//         pages: 101,
//         rating: 4.4,
//     };

//     // Передаем ссылки в функции
//     print_pages(&book); 
//     print_rating(&book);
// }









// #[derive(Debug)]
// enum Light {
//     Bright,
//     Dull,
// }

// fn on_off(light: &mut Light) {
//     // разыменовываем ссылку звездочкой * указывая доступ к настоящим данным
//     *light = match *light {
//         Light::Bright => Light::Dull,
//         Light::Dull => Light::Bright,
//     }; // разыменовывая 
// }

// fn main() {
//     let mut dull = Light::Dull;
//     println!("было: {:?}", dull);

//     on_off(&mut dull);

//     println!("стало: {:?}", dull);

// }






// fn print_face(light: &Light) {
//     // это получается что входящие в эту функцию параметры
//     // должны обладать структурой и одной из опций enum Light?
//     match light {
//         Light::Bright => println!("Bright"),
//         Light::Dull => println!("Dull"),
//     }
// }
// // скажем если ниже будет переменная 

// fn main() {
//    let dull = Light::Dull;
//    print_face(&dull);
//    print_face(&dull);

// }






 // https://skr.sh/sPj4yQA0Jfk