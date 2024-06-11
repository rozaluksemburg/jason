// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
// * Распечатываем характеристики транспортировочной коробки
// * Должны быть указаны размеры, вес и цвет
// просить ии переписывать задачи понятно
// Notes:
// Примечания:


enum Color {
        Blue,
        Yellow,
    }
    
    impl Color {
        fn print(&self) {
        match self {
            Color::Blue => println!("цвет ноутбука Blue"),
            Color::Yellow => println!("цвет ноутбука Yellow"),
        }
    }
    
    }
    
    struct Dimensions {
        width: f32,
        height: f32,
        depth: f32,
    }
    
    impl Dimensions {
        fn print(&self) {
        println!("длина: {:?}", self.width);
        println!("ширина: {:?}", self.height);
        println!("высота: {:?}", self.depth);
    }
    
    }
    
    struct ProductData {
        color: Color,
        weight: f32,
        dimensions: Dimensions,
    }
    
    impl ProductData {
        fn new(color: Color, weight: f32, dimensions: Dimensions) -> Self {
        Self { color, weight, dimensions }
        }
    
        fn print(&self) {
        self.color.print();
        println!("и вес составляет: {:?}", self.weight);
        self.dimensions.print();
    }
    
    }


    fn main() {
        let box_dimensions1 = Dimensions {
            width: 23.2,
            height: 21.3,
            depth: 19.3,
        }; 
        
        let small_box = ProductData::new(Color::Blue, 7.7, box_dimensions1);
        
        small_box.print();
        
        let box_dimensions2 = Dimensions {
            width: 34.3,
            height: 31.2,
            depth: 24.3,
        }; 
        
        let big_box = ProductData::new(Color::Yellow, 9.9, box_dimensions2);
        
        big_box.print();

        }





// enum Color {
//     Blue,
//     Yellow,
// }

// impl Color {
//     fn print_color(&self) {
//         match self {
//             Color::Blue => println!("color happy blue"),
//             Color::Yellow => println!("color happy yellow"),
//         }
//     }
// }

// struct Dimensions {
//     width: f32,
//     height: f32,
//     depth: f32,
// }

// impl Dimensions {
//     fn print(&self) {
//         println!("ширина {:?}", self.width);
//         println!("высота {:?}", self.height);
//         println!("depth {:?}", self.depth);
//     }
// }

// struct BoxProperties {
//     color: Color,
//     weight: f32,
//     dimensions: Dimensions,
// }

// impl BoxProperties {
//     fn new(weight: f32, color: Color, dimensions: Dimensions) -> Self {
//         Self { color, weight, dimensions }
//     }

//     fn print(&self) {
//         self.color.print_color();
//         self.dimensions.print();
//         println!("weight: {:?}", self.weight);
//     }
// }

// fn main() {
//     let small_dimensions = Dimensions {
//         width: 1.4,
//         height: 2.2,
//         depth: 3.0,
//     };

//     let small_box = BoxProperties::new(21.3, Color::Blue, small_dimensions);

//     small_box.print();

// }


















// enum Color {
//     Blue,
//     Yellow,
// }

// impl Color {
//     fn print(&self) {
//         match self {
//             Color::Blue => println!("цвет настроения blue"),
//             Color::Yellow => println!("цвет настроения yellow"),
//         }
//     }
// }

// struct Dimensions {
//     width: f32,
//     height: f32,
//     depth: f32,
// }

// impl Dimensions {
//     fn print(&self) {
//         println!("width: {:?}", self.width);
//         println!("height: {:?}", self.height);
//         println!("depth: {:?}", self.depth);


//     }
// }

// struct ShippingBox {
//     color: Color,
//     weight: f32,
//     dimensions: Dimensions,
// }

// impl ShippingBox {
//     fn new(weight: f32, color: Color, dimensions: Dimensions) -> Self {
//          Self { weight, color, dimensions, }
//     }

//     fn print(&self) {
//         self.color.print();
//         self.dimensions.print();
//         println!("weight: {:?}", self.weight);
//     }
// }

// fn main() {
//    let small_dimensions = Dimensions {
//        width: 1.0,
//        height: 2.1,
//        depth: 2.0,
//    };
//    let small_box = ShippingBox::new(21.3, Color::Blue, small_dimensions);

//    small_box.print();

//    let big_dimensions = Dimensions {
//        width: 2.0,
//        height: 3.1,
//        depth: 3.0,
//    };
//    let big_box = ShippingBox::new(29.3, Color::Yellow, big_dimensions);

//    big_box.print();
// }

 // let my_box = ShippingBox {
    //     color: Color::Blue,
    //     weight: 5.5,
    //     dimensions: Dimensions {
    //         width: 11.1,
    //         height: 12.0,
    //         depth: 1.4,
    //     }
    // };
    // my_box.display_data();




/*
Владимир Тарковский просил передать
что он загуляет сегодня опять
а мог бы гулять и вчера, и сегодня 
да только вчера приходил к нему водный 
ангел, шептал что-то, долго курил,
и бил по щекам ему крыльями, и
когда он очнулся, то слышал, как звезды 
смыкают во тьме лучевые аорты
и время скользит между сомкнутых рук 





в объятиях с тем, кому выдал хирург 






заполнили окна пугливые сестры 
из пажитей тамошних, будто его 
несли женским взглядом 


с богом в окно

*/



// * Use a struct to encapsulate the box characteristics
// * Используйте структуру для инкапсуляции характеристик блока
// struct Box {
//     lenght: u32,
//     width: u32,
//     height: u32,
// }

// // * Use an enum for the box color
// // * Используйте перечисление для цвета поля

// enum Color {
//     Yellow,
//     Blue,
// }

// // * Implement functionality on the box struct to create a new box
// // * Реализуем функциональность структуры блока для создания нового блока

// impl Box {
//     fn new(&self) {
//         println!("длина (lenght): {:?}\nширина (width): {:?}\nвысота (height): {:?}", self.lenght, self.width, self.height);
//     }

    
// }

// impl Color {
//     fn print_color(&self) {
//         match self {
//             Color::Yellow => println!("цвет настроения yellow"),
//             Color::Blue => println!("цвет настроения blue"),
//         }
//     }
// }

// // * Implement functionality on the box struct to print the characteristics
// // * Реализуем функциональность в структуре блока для печати характеристик

// fn main() {
//     let box1 = Box {
//         lenght: 10,
//         width: 21,
//         height: 32,
//     };
//     box1.new();

//     let color = Color::Yellow;
//     color.print_color();


// }




// шпаргалка https://zerotomastery.io/cheatsheets/rust-cheat-sheet/








// struct Sweets {
//     candies: u32;
// }

// impl Sweets {
//     fn min_candies() -> Self {
//         Self { candies: 93 }
//     }

//     fn max_candies(&self) {
//         println!("девочек конфеточек сегодня: {:?}", self.candies);
//     }
// }

// fn main() {
//     let min_candies = Sweets::min_candies();
//     min_candies.max_candies();

//     let max_candies = Sweets { candies: 103 };

// }














// struct Sweets {
// 	candies: u32,
// }

// impl Sweets {
// 	fn min_candies() -> Self {
// 		Self { candies: 38 } 
// }

//     fn medium_candies() -> Self {
//         Self { candies: 50}
//     }

// 	fn max_candies(&self) {
// 		println!("конфеток: {:?} ", self.candies);
// }

// }

// fn main() {
// 	let min_candies = Sweets::min_candies();
// 	min_candies.max_candies();

//     let medium_candies = Sweets::medium_candies();
//     medium_candies.max_candies();

// 	let max_candies = Sweets { candies: 71 };
// 	max_candies.max_candies();
// }






















// struct Sweets {
//     candies: u32,
// }
// impl Sweets {
//     fn min_candies() -> Self {
//         Self { candies: 83 }
//     }

//     fn max_candies(&self) {
//         println!("max candies: {:?}", self.candies);
//     }
    
// }

// fn main() {
//     let min_candies = Sweets::min_candies();
//     min_candies.max_candies();

//     let max_candies = Sweets { candies: 101 };
//     max_candies.max_candies();
// }












// struct Temperature {
//     degrees: f32,
// }

// impl Temperature {
//     fn freezing() -> Self { // можно так fn freezing() -> Temperature {
//         Self { degrees: 32.3 } // и так Temperature { degrees: 32.3 }
//        // но альтернативное написание менее гибко, чем чистовик
//     // так мы добавили новую температуру - температуру замерзания 
//     }

//     fn show_temp(&self) {
//     println!("{:?} degrees", self.degrees); 
// }

// }
// fn main() {
//     let hot = Temperature { degrees: 32.1 };
//     hot.show_temp();

//     let cold = Temperature::freezing();
//     cold.show_temp(); 
    
    
// }
    
    // то же самое что Temperature::show_temp(&hot);
    
// то есть мы обращаемся к стуктуре Temperature и далее 
// к имплеменцированной через impl функции show_temp которая по сути
// стала методом который мы сами и создали - правильно говорю? и еще я умею по-русски печать десятью пальцами - и когда я пишу код то тоже не смотреть на клавиатуру но раньше когда я писал на латинице я это делал двумя пальцами - а сейчас я применил ту же систему слепой печати к английскому 



// а и получается, что show_temp(&self) то есть &self обозначает что когда мы вызываем эту функцию например командой hot.show_temp() и то есть в скобки где &self передастся значение указанное в переменной hot 

/*
Да, сэр, ваше понимание абсолютно верное! В Rust, использование `&self` в сигнатуре метода означает, что метод принимает неизменяемую ссылку на экземпляр, с которым он вызывается. Это позволяет методу читать данные экземпляра, не изменяя их.

Когда вы вызываете метод как `hot.show_temp()`, происходит следующее:

- `hot` — это экземпляр структуры `Temperature`.
- Когда метод `show_temp` вызывается с использованием синтаксиса точки `.`, Rust автоматически берет неизменяемую ссылку на `hot` и передает ее в метод как `&self`.
  
Это означает, что внутри метода `show_temp`, `self` теперь ссылается на `hot`. Следовательно, `self.degrees` будет равно `hot.degrees`, и метод сможет получить доступ к данным `degrees`, хранящимся в `hot`, для выполнения своих функций (в данном случае для вывода значения на экран).

Этот механизм ссылок является одной из ключевых особенностей Rust, обеспечивающих безопасность памяти и предотвращающих различные виды ошибок, такие как гонки данных или нарушение доступа к памяти. Он также способствует чистоте и читаемости кода, так как ясно показывает, какие методы могут изменять данные объекта, а какие — нет.

Если у вас есть дополнительные вопросы по этой теме или другим аспектам Rust, я с удовольствием помогу вам разобраться, сэр!

*/

// я упростил
// struct Bag {
//     vodka: i32
// }

// impl Bag {
//     fn add_vodka(&mut self, amount: i32) {
//         self.vodka += amount;
//         println!("Добавлено водки {:?} мл", amount);
//     }

//     fn remove_vodka(&mut self, amount: i32) {
//         self.vodka -= amount;
//         println!("Убавлено водки: {:?}", amount);
//     }
// }
//     fn main() {
//         let mut bag = Bag {
//             vodka: 743,
//         };

//         bag.add_vodka(100);
//         bag.remove_vodka(100);
//     }












// struct Bag {
//     icecream: u32,
//     vodka: u32,
// }

// impl Bag {
//     fn new_pank(&self) {
//         println!("мороженого осталось {:?} мл", self.icecream);
//         println!("водки осталось {:?} мл", self.vodka);
//     }
// }

// fn main() {
//     let bag = Bag {
//         icecream: 394,
//         vodka: 743,
//     };

//     bag.new_pank();
// }






// struct Temperature {
//     degrees: f32,
// }

// impl Temperature {
//     // вместо &self было (temp: Temperature) и это значит 
//     // что &self говорит о том что когда мы будем
// // вставлять в него что-то, то это что-то должно структурно
// // быть формата структуры выше 
//     fn show_temp(&self) {
//         println!("temperature: {:?} degrees", self.degrees);
//     }
// }

// fn main() {
//     let hot = Temperature { degrees: 99.9 };
//     hot.show_temp();

//     // было так Temperature::show_temp(hot);
// }


/* impl мы отождествляем или связываем что-то с чем-то
например структуру Temperature и функцию show_temp() 
и теперь когда мы напишем hot.show() то экземпляр структуры Temperature 
то есть эс это значение переменной hot которое будет взято функцией 
show_temp() для того чтобы передать в структуру конкретные значения 

также impl используется что какой-то тип данных соответствует 
определенному контракту или трейту. 

ниже код от ИИ с расшифровкой 

// Определение трейта Drivable. Трейт определяет набор методов, которые должны быть реализованы
// типами данных, которые хотят "выполнять" данный трейт. В данном случае определен метод drive.
trait Drivable {
    // Объявление метода drive, который должен быть реализован. Этот метод не изменяет данные,
    // поэтому используется &self, что означает неизменяемую ссылку на экземпляр, который вызывает метод.
    fn drive(&self);
}

// Определение структуры Car с одним полем model типа String.
struct Car {
    model: String,
}

// Реализация трейта Drivable для структуры Car. Этот блок кода реализует поведение,
// указанное в трейте Drivable, для экземпляров Car.
impl Drivable for Car {
    // Реализация метода drive из трейта Drivable. Когда этот метод вызывается на экземпляре Car,
    // в консоль будет выведено сообщение, которое включает модель автомобиля.
    fn drive(&self) {
        println!("{} is driving!", self.model);
    }
}

// Отдельная реализация блока impl для структуры Car, в котором определяются методы, специфичные для Car.
impl Car {
    // Метод start, который можно вызывать на экземплярах Car. Этот метод также не изменяет данные,
    // он просто выводит сообщение в консоль, используя модель автомобиля.
    fn start(&self) {
        println!("{} starts!", self.model);
    }
}

мой вопрос 
прекрасно объяснил -- благодарю - короче мы отождествляем что-то 
с чем-то через impl  а потом, если нам нужно, то мы объявляем 
переменную со структурой подачи конкретных данных соответственно 
той имплементации к методу которой мы обращаемся через точку 
my_car.drive(); 


*/