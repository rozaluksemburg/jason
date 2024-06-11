// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

// Тема: Хэш-карта
//
// Требования:
// * Напечатайте название и количество товаров на складе для мебельного магазина
// * Если количество товаров равно 0, вместо 0 напечатайте "нет в наличии"
// * В магазине есть:
// * 5 стульев
// * 3 кровати
// * 2 стола
// * 0 диванов
// * Выведите общее количество товаров на складе
//
// Примечания:
// * Используйте хэш-карту для ассортимента мебельного магазина


use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Beds", 3);
    stock.insert("Tables", 2);
    stock.insert("Couches", 0);

    let mut total_quantity = 0;

    for (item, quantity) in stock.iter() {
        total_quantity = total_quantity + quantity;
        let stock_count = if quantity == &0 {
            "нет товара на складе".to_owned()
        } else { // stock_count если не равно 0 то равна quantity - то есть количеству
            format!("{:?}", quantity) 
        }; // format! ставится потому что он в переменной и нужно значения, а не вывод для следующих коллабораций 

        println!("товар: {:?}, количество: {:?}", item, stock_count);

    }
    println!("Общее количество товаров на складе: {:?}", total_quantity);

}



































// use std::collections::HashMap;

// fn main() {
//     let mut stock = HashMap::new();
//     stock.insert("Chair", 5);
//     stock.insert("Bed", 3);
//     stock.insert("Table", 2);
//     stock.insert("Couch", 0);

//      let mut total_stock = 0;
    
//     for (item, quantity) in stock.iter() { 
//         total_stock = total_stock + quantity;
//         let stock_count = if quantity == &0 {
//             "нет на складе".to_owned()
//         } else {
//             format!("{:?}", quantity)
//         };
//         println!("товар: {:?}, количество: {:?}", item, stock_count);
//     }
//     println!("общее количество на складе: {:?}", total_stock);
// }






















//     let mut total_stock = 0;
//     for (item, quantity) in stock.iter() {
//         total_stock = total_stock + quantity;
//         let stock_count = if quantity == &0 {
//             "out of stock".to_owned()
//         } else {
//             format!("{:?}", quantity)
//         }; 
//         println!("товар={:?}, количество={:?}", item, stock_count);
//     }
//     println!("общее количество товаров на складе={:?}", total_stock);
// }


























// use std::collections::HashMap;
// use std::fmt;
// use std::fmt::Debug;


// #[derive(Debug, Eq, Hash, PartialEq)]
// struct WarehouseStore {
//     name: String,
// }

// impl fmt::Display for WarehouseStore {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.name)
//     }
// }



// fn main() {
//     let mut store = HashMap::new();
//     store.insert(WarehouseStore {
//         name: "Chairs".to_owned(),
//     }, 5);
//     store.insert(WarehouseStore {
//         name: "Beds".to_owned(),
//     }, 3);
//     store.insert(WarehouseStore {
//         name: "Tables".to_owned(),
//     }, 2);
//     store.insert(WarehouseStore {
//         name: "Couches".to_owned(),
//     }, 0);

//     println!("название товара / количество"); 

//     for (i, number) in store.iter() {
//         if number == &0 {
//             println!("{} нет в наличии", i.name)
//         } else {
//             println!("{} / {}", i.name, number)
//         }
//     }
// }














// use std::collections::HashMap;

// struct HappyPussy {
//     name: String,
// }

// fn main() {
//      let mut pussy = HashMap::new();
//      pussy.insert(5, HappyPussy {
//         name: "Julia".to_owned(),
//      });
//      pussy.insert(3, HappyPussy {
//         name: "Violetta".to_owned(),
//      });

//      println!("девочка / оценка:");

//      for (value, key) in pussy {        
//         println!("{:?} / {:?} ", key.name, value);
//      }







// }

/*
к сожалению или к счастью 
или к счастью и сожалению 
я снимаю налипший пластырь 
с твоих бедер 

*/
    // создание пустой HashMap
    // let mut people = HashMap::new();
    // people.insert("Susan", 21);
    // people.insert("Violetta", 30);
    // people.insert("Will", 14);
    // people.insert("Cathy",22);
    // people.remove("Susan");

    // match people.get("Cathy") {
    //     Some(age) => println!("Cathy is {} years old", age),
    //     None => println!("дело дрянь"),
    // }

    // for (key, value) in people.iter() {
    //     println!("key: {:?}, value: {:?}", key, value);
    // }

    // for person in people.keys() {
    //     println!("key: {:?}", person);
    // }

    // for age in people.values() {
    //     println!("age: {:?}", age);
    // }




    // // вставка данных
    // baza.insert("Violetta", 30);
    // baza.insert("Brody", 23);

    // println!("{:?}", baza);

    // // чтобы извлечь значение по ключу, можно использовать метод get
    // let age = baza.get("Violetta");
    // match age {
    //     Some(age) => println!("Violetta is {} years old", age),
    //     None => println!("данные тютю")
    // }

    // // для удаления пары ключ-значение используется метод remove
    // // baza.remove("Brody");
    // // println!("{:?}", baza);

    // // Итерация по ключам и значениям - выводит имена и значения
    // // чтобы пройтись по всем парам ключ-значение, использовать метод iter 
    // for (key, value) in baza.iter() {
    //     println!("{}: {}", key, value);
    // } // так показываем все пары ключ значение

    // // а так возвращает итератор только по ключам то выводит имена
    // for key in baza.keys() {
    //     println!("{:?}", key);
    // }

    // // а так только по значениям
    // for value in baza.values() {
    //     println!("{:?}", value);

    // }






    // insert(key, value);
    // добавляет значение по ключу 

    // get(key);
    // возвращает ссылку на значение, связанное с ключом

    // remove(key);
    // удаляет значение по ключу

    // iter() 
    // возвращает ссылку на итератор

    // key() 
    // возвращает итератор только по ключам 
    
    // value() 
    // возвращает итератор только по значениям























// use std::collections::HashMap;
// fn main() {
//     // создание пустой HashMap
//     let mut ages = HashMap::new();

//     // вставка данных 
//     ages.insert("Alice", 30);
//     ages.insert("Brody", 23);
//     ages.insert("Diana", 25);

//     // Вывод HashMap
//     println!("{:?}", ages);

//     // чтобы извлечь значение по ключу, можно использовать метод get 
//     let age_of_alice = ages.get("Alice");
    
//     match age_of_alice {
//         Some(age) => println!("Alice is {} years old", age),
//         None => println!("не найдена")
//     }

// }































/*
и вот мой Иордан 
прилив, отлив, туман
все, как ты хочешь, здесь 
распятие или лес 
а между - ничего 
лишь только в дождь огонь 
пульсирует на взвесь 


а дальше - торжество 
и



*/