// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Human {
    gender: String,
    name: String,
    age: u8,
    profession: String,
}

fn main() {
    let humans = vec![
        Human {
            gender: "Woman".to_owned(),
            name: "Jessica".to_owned(),
            age: 24,
            profession: String::from("escort girl")
        },

        Human {
            gender: "Man".to_owned(),
            name: "Billy".to_owned(),
            age: 38,
            profession: String::from("Gangster"),
        }
    ];

    for i in humans {
        println!("gender: {:?}\nname: {:?}\nage: {:?},\nprofession: {:?}", i.gender,i.name, i.age, i.profession);
    }
}






















// * Use a struct for a persons age, name, and favorite color
// struct Person {
//     name: String,
//     age: u8,
//     color: String,
// }

// fn print_color(name: &str) {
//     println!("{:?}", name);
// }

// fn main() {
//     let person_data = vec![
//         Person {
//             name: "Виолетта".to_owned(),
//             age: 29,
//             color: String::from("любимый цвет - цвет месячных в полнолуние"),
//         },
//         Person {
//             name: "Розабелла".to_owned(),
//             age: 31,
//             color: String::from("любимый цвет - цвет запаха мужчины")
//         },
//         Person {
//             name: "Перлита".to_owned(),
//             age: 24,
//             color: String::from("любимый цвет - цвет уздечки Виолетты")
//         },
//     ];

//     for person in person_data {
//         if person.age < 30 {
//         println!("name: {:?}, age: {:?}", person.name, person.age);
//         print_color(&person.color);
//     } else {
//         println!("{:?} вам уже {:?} что вроде бы больше 30, ", person.name, person.age); 
//         print_color(&person.color);
//     }





// }

// } 

// struct Girl {
//  name: String,
//    age: u8,
//    bust_size: f64,
//    pussy_size: String,
//     }
    
//     fn print_name(name: &str) {
//         println!("{:?}", name);
//     }

//    fn main() {
//    let girl_new = vec![
//    Girl {
//    name: "Monica".to_owned(),
//    age: 18,
//    bust_size: 4.2,
//    pussy_size: String::from("small"),
//    },
//    Girl {
//    name: "Anna".to_owned(),
//    age: 24,
//    bust_size: 3.8,
//    pussy_size: String::from("medium"),
//    }
    
//     ];
    
//    for girl in girl_new {
//    print_name(&girl.name);
//    println!("age: {:?}\nbust_size: {:?}\npussy_size: {:?}", girl.age, girl.bust_size, girl.pussy_size);  
    
//    }
    
//     }



// struct Girl {
//     bust: f64,
//     name: String,
//     pussy: String,
// }

// fn main() {
//     let girls_new = vec![
//         Girl {
//             bust: 3.2,
//             name: "миссис Sweet".to_owned(), 
//             pussy: "small".to_owned(),
//         },
//         Girl {
//             bust: 1.6,
//             name: "Baby Snow".to_owned(),
//             pussy: "crazy small".to_owned(),
//         }

//     ];

//     for girl in girls_new {
//         println!("name: {:?}, bust: {:?}, pussy: {:?}", girl.name, girl.bust, girl.pussy);
     
//     }

// }



























// struct LineItem {
//     name: String,
//     count: i32,
// }

// fn main() {
//     let receipt = vec![
//         LineItem { 
//             name: "cereal".to_owned(),
//             count: 1,
//         },
//         LineItem {
//             name: String::from("fruit"),
//             count: 3,
//         },    
        
//     ];

//     for item in receipt {
//         println!("name: {:?}, count: {:?}", item.name, item.count );

//     }
// }





// fn print_it(data: &str) {
//     println!("{:?}", data);
// }
//     struct Worker {
//         name: String,
//     }


// fn main() {
//     // просто вкладываем строковое значение в функцию выше
//     print_it("миссис Sweet");

//     let owned = "любит".to_owned();
//     let miss_like = String::from("сосать только большие");
//     print_it(&owned);
//     print_it(&miss_like);

//     let worker_name = "Monica".to_owned();
//     let worker_name2 = String::from("Jess");
    
//     let public_home = Worker {
//         name: worker_name
//     };

//     println!("а {:?} любит ласкать киску {:?}", public_home.name, worker_name2);

// }


// struct Employee {
//     name: String,
// }

// fn main() {
//     let emp_name = "миссис X";
//     let emp = Employee {
//         name: emp_name.to_string()
//     }

// }



// fn print_it(data: &str) {
//     println!("{:?}", data);
// }
// fn main() {
//     print_it("слайсы");
//// создание строки через метод to_owned()
//     let owned_string = "слоятся".to_owned();
//// создание строки через конструкцию String::from()
//     let another_owned = String::from("по-другому");
//     print_it(&owned_string);
//     print_it(&another_owned); 


// } 



/*
коллаборации холмов или холмов коллаборации 
я начал было про любовь, но и любовь - коллаборация 

расставляя вещи по местам 



заправлялся пивом 
а потом исчез 
был такой красивый 
золота на вес 


*/

























// 1. Создание и использование собственных строк
    
    // создание пустой строки
//     let mut s = String::new();

//     // создание строки из строкового литерала 
//     let _s1 = "Hello".to_string();

//     // создание строки с использованием функции String::from
//     let s2 = String::from("world");

//     // конкатенация строк
//     s.push_str("сиреневоголовый ");
//     s.push_str(&s2);
//     println!("{:?}", s);
    


// // 2. Строковые фрагменты и передача их в функцию
//     // строковый фрагмент
//     let s = "Hello, rusty, how do you think";

//     // передача строкового фрагмента в функцию
//     print_str1(s);

//     // функция принимает строковый фрагмент 
//     fn print_str1(s: &str) {
//         println!("{:?}", s);
//     }

// // 3. хранение строк в структуре 

//  struct Skils {
//     name: String,
//  }

//  impl Skils {
//     fn new(name: &str) -> Self {
//         Self { name: name.to_string(), }
//     }

//     fn print_skill(&self) {
//         println!("{:?}", self.name);
//     }
//  }

//     let skils = Skils::new("poet coder");

//     skils.print_skill();






// }





//     struct QuantumDetails {
//         name: String,
//     }

//     impl QuantumDetails {
//         fn new(name: &str) -> Self {
//             Self { name: name.to_string(), }
//         }

//         fn print_name(&self) {
//             println!("{:?}", self.name);
//         }
//     }

//     let quantum = QuantumDetails::new("Quantum");

//     quantum.print_name();

// }

/*
я сегодня вышел в дождь 
там вода вокруг лежит 
и сказал я - ну и что ж?
пусть вода вокруг лежит 

невесомая на вид
словно стыд, прозрачный стыд, 
превращенный в вещество 
звук и смертность как родство 

мы идем с тобой в кино
пресные от той воды 



и поэтому болит 
шоркает душа о дно 
у кувшина 

да пожалуй всех живых 


то которое лежит 









// Структура, хранящая собственную строку
struct MyStruct {
    name: String, // Поле структуры, которое содержит собственную строку типа String
}

impl MyStruct {
    // Функция для создания нового экземпляра структуры MyStruct
    // Принимает строковый фрагмент (строка с типом &str) в качестве параметра
    fn new(name: &str) -> MyStruct {
        // Возвращает экземпляр структуры MyStruct
        MyStruct {
            // Преобразуем строковый фрагмент (name) в собственную строку (String)
            // Это необходимо, чтобы структура владела строкой и могла управлять её памятью
            name: name.to_string(),
        }
    }

    // Метод для печати имени, хранящегося в структуре
    // Метод принимает ссылку на экземпляр структуры (self) и не изменяет его
    fn print_name(&self) {
        // Печатает значение поля name, используя макрос println!
        println!("{}", self.name);
    }
}

fn main() {
    // Создание объекта структуры MyStruct
    // Вызываем функцию new и передаем строковый фрагмент "Rust"
    // Функция new возвращает экземпляр MyStruct, который сохраняется в переменной my_struct
    let my_struct = MyStruct::new("Rust");

    // Вызов метода print_name на объекте my_struct
    // Метод print_name печатает значение поля name, хранящегося в my_struct
    my_struct.print_name(); // Вывод: Rust
}




























<html>
 <head>
  <title>
   OZON Product Page
  </title>
  <script src="https://cdn.tailwindcss.com">
  </script>
  <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" rel="stylesheet"/>
  <style>
   body {
            background-color: #1F2937;
            color: #FFFFFF;
        }
        .discount {
            color: #10B981;
        }
        .price-old {
            text-decoration: line-through;
            color: #6B7280;
        }
        .stock {
            color: #6B7280;
        }
        .button-blue {
            background-color: #3B82F6;
            color: white;
            padding: 8px 16px;
            border-radius: 8px;
            font-size: 14px;
        }
  </style>
 </head>
 <body>
  <div class="container mx-auto px-4">
   <div class="grid grid-cols-3 gap-4">
    <!-- Product 1 -->
    <div class="bg-gray-800 p-4 rounded-lg">
     <img alt="Ergonomic split keyboard with light gray keys and no keycaps, Russian layout" class="mb-4" height="150" src="https://oaidalleapiprodscus.blob.core.windows.net/private/org-sSsOulEoQY3dXtF2RiaK66Ja/user-XZ7Av8ZH1bvGvI0GdH7WwbAs/img-FVbM6mcEzPqdOu9tTs864ZWd.png?st=2024-05-19T14%3A51%3A45Z&amp;se=2024-05-19T16%3A51%3A45Z&amp;sp=r&amp;sv=2021-08-06&amp;sr=b&amp;rscd=inline&amp;rsct=image/png&amp;skoid=6aaadede-4fb3-4698-a8f6-684d7786b067&amp;sktid=a48cca56-e6da-484e-a814-9c849652bcb3&amp;skt=2024-05-19T00%3A10%3A23Z&amp;ske=2024-05-20T00%3A10%3A23Z&amp;sks=b&amp;skv=2021-08-06&amp;sig=jSsCQQLolEto2np7pVXu/Rs9SL8z21PoK3QQj8FNyzg%3D" width="200"/>
     <h2 class="text-lg font-semibold">
      Ergohaven Клавиатура проводная K03 без светилей и кейкапов, (Без клавиш), Русская раскладка
     </h2>
     <p class="text-sm">
      Тип: Клавиатура
     </p>
     <p class="text-sm">
      Тип соединения: проводная
     </p>
     <p class="text-sm">
      Тип клавиатуры: Механическая
     </p>
     <p class="text-sm">
      Количество клавиш клавиатуры: 60
     </p>
     <p class="text-sm">
      Раскладка клавиатуры: Русская раскладка
     </p>
     <div class="flex items-center justify-between mt-2">
      <span class="text-xl font-bold">
       13 000 Р
      </span>
      <span class="price-old">
       15 000 Р
      </span>
      <span class="discount">
       -13%
      </span>
     </div>
     <div class="flex items-center justify-between mt-2">
      <div class="stock">
       Осталось 10 шт
      </div>
      <div class="button-blue">
       24 мая
      </div>
     </div>
    </div>
    <!-- Product 2 -->
    <div class="bg-gray-800 p-4 rounded-lg">
     <img alt="Compact white mechanical keyboard with USB-C connection, Russian layout" class="mb-4" height="150" src="https://oaidalleapiprodscus.blob.core.windows.net/private/org-sSsOulEoQY3dXtF2RiaK66Ja/user-XZ7Av8ZH1bvGvI0GdH7WwbAs/img-NU8hok1ExU6ookCbdOllC9aA.png?st=2024-05-19T14%3A51%3A48Z&amp;se=2024-05-19T16%3A51%3A48Z&amp;sp=r&amp;sv=2021-08-06&amp;sr=b&amp;rscd=inline&amp;rsct=image/png&amp;skoid=6aaadede-4fb3-4698-a8f6-684d7786b067&amp;sktid=a48cca56-e6da-484e-a814-9c849652bcb3&amp;skt=2024-05-18T20%3A15%3A48Z&amp;ske=2024-05-19T20%3A15%3A48Z&amp;sks=b&amp;skv=2021-08-06&amp;sig=jkf1%2BtKW9ZuwE0PEkTeWzBFT4tW1e1ruBlqoxCTgpPk%3D" width="200"/>
     <h2 class="text-lg font-semibold">
      Ergohaven Клавиатура проводная K03, (Gateron Brown G Pro 3.0), Русская раскладка, белый
     </h2>
     <p class="text-sm">
      Тип: Клавиатура
     </p>
     <p class="text-sm">
      Тип соединения: проводная
     </p>
     <p class="text-sm">
      Интерфейс: USB Type-C
     </p>
     <p class="text-sm">
      Тип клавиатуры: Механическая
     </p>
     <p class="text-sm">
      Количество клавиш клавиатуры: 58
     </p>
     <div class="flex items-center justify-between mt-2">
      <span class="text-xl font-bold">
       19 000 Р
      </span>
      <span class="price-old">
       21 500 Р
      </span>
      <span class="discount">
       -11%
      </span>
     </div>
     <div class="flex items-center justify-between mt-2">
      <div class="stock">
       Осталось 2 шт
      </div>
      <div class="button-blue">
       Новинка
      </div>
     </div>
    </div>
    <!-- Product 3 -->
    <div class="bg-gray-800 p-4 rounded-lg">
     <img alt="Dark blue mechanical keyboard with USB connection and red Gateron switches, Russian layout" class="mb-4" height="150" src="https://oaidalleapiprodscus.blob.core.windows.net/private/org-sSsOulEoQY3dXtF2RiaK66Ja/user-XZ7Av8ZH1bvGvI0GdH7WwbAs/img-x0AaRTAJzIpvJShfIzvWiYK9.png?st=2024-05-19T14%3A51%3A47Z&amp;se=2024-05-19T16%3A51%3A47Z&amp;sp=r&amp;sv=2021-08-06&amp;sr=b&amp;rscd=inline&amp;rsct=image/png&amp;skoid=6aaadede-4fb3-4698-a8f6-684d7786b067&amp;sktid=a48cca56-e6da-484e-a814-9c849652bcb3&amp;skt=2024-05-19T04%3A11%3A39Z&amp;ske=2024-05-20T04%3A11%3A39Z&amp;sks=b&amp;skv=2021-08-06&amp;sig=wf35yGk/odoKvZlZWgBMu1fnIJUyC1WC9IuaRCm4XDo%3D" width="200"/>
     <h2 class="text-lg font-semibold">
      Keychron Клавиатура проводная Q1, (Gateron G Pro Red), темно-синий
     </h2>
     <p class="text-sm">
      Тип: Клавиатура
     </p>
     <p class="text-sm">
      Тип соединения: проводная
     </p>
     <p class="text-sm">
      Интерфейс: USB
     </p>
     <p class="text-sm">
      Тип клавиатуры: Механическая
     </p>
     <p class="text-sm">
      Количество клавиш клавиатуры: 91
     </p>
     <div class="flex items-center justify-between mt-2">
      <span class="text-xl font-bold">
       18 666 Р
      </span>
      <span class="price-old">
       23 994 Р
      </span>
      <span class="discount">
       -22%
      </span>
     </div>
     <div class="flex items-center justify-between mt-2">
      <div class="stock">
       Осталось 2 шт
      </div>
      <div class="button-blue">
       Скидка недели
      </div>
     </div>
    </div>
   </div>
  </div>
 </body>
</html>

*/


