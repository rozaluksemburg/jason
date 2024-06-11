// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print


fn return_turple() -> (i32, i32, i32) {
    (7, 2, 5)
}

fn main() {
    
    let (x, y, z) = return_turple();

    if y > 5 {
        println!("y-value > 5");
    } else if y < 5 {
        println!("y-value < 5");
    } else {
        println!("y-value = 5");
    }

    if x > 5 {
        println!("x-value > 5");
    } else if x < 5 {
        println!("x-value < 5");
    } else {
        println!("x-value = 5");
    }

    if z > 5 {
        println!("z-value > 5");
    } else if z < 5 {
        println!("z-value < 5");
    } else {
        println!("z-value = 5");
    }

}








// fn xyz() -> (i32, i32, i32) {
//     (4, 8, 7)
// }

// fn main() {
//     let (x, y, z) = xyz();

//     if y > 5 {
//         println!("y-value > 5");
//     }




// }











/*
был воздух выстлан ржавчиной холмов
а я всё пел про эту, про любовь 

*/





/* отличная программа, демонстрирующая уровень доступа 

use std::io;

#[derive(Debug)]  // Этот атрибут должен быть здесь, чтобы включить поддержку форматирования Debug для AccessLevel
enum AccessLevel {
    User,
    Admin,
}

struct Human {
    name: String,
    access_level: AccessLevel,
}

fn main() {
    println!("Пожалуйста, введите ваше имя для инициализации уровня доступа");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("упс");
    let name = name.trim().to_string(); 

    let user_access = Human {
        name: name.clone(),
        access_level: if name == "Ionesko" {
            AccessLevel::Admin
        } else {
            AccessLevel::User
        }
    };  

    match user_access.access_level {
        AccessLevel::Admin => println!("Добро пожаловать, Сэр"),
        AccessLevel::User => println!("try again {:?}", user_access.name), 

    }

    println!("имя: {}\nуровень доступа: {:?}", user_access.name, user_access.access_level);   


}

конец программы с уровнем доступа 
*/

// fn main() {
//     let coord = (2, 3);
//     println!("первое значение: {:?}\nвторое значение: {:?}", coord.0, coord.1);

//     // пример деструктурирование кортежа 
//     let (x, y) = coord;
//     println!("{:?}\n{:?}", x, y); 

//     let user_info = ("Emma", 20);
//     println!("{:?}\n{:?}", user_info.0, user_info.1);

//     // но вместо того, что выше, мы можем ввести деструктурирование кортежа
//     // делается это для того что 2 значения если в переменной,
//     // то чтобы потом можно было это вспомнить таким образом
//     let (name, age) = ("Aigul", 30);

//     let favorites = ("yellow", 14, "Che", "coffee");

//     let state = favorites.2;
//     let drink = favorites.3;

// // в кортежах лучше не сохранять больше 3 пунктов 






// }





// use std::io;

// enum AccessLevel {
//     User,
//     Admin,
// }

// struct User {
//     name: String,
//     access_level: AccessLevel,
// }

// fn main() {

//     let mut scaner_access = String::new();

//     io::stdin().read_line(&mut scaner_access).expect("баран"); 

//     let user1 = User {
//         name: String::from("Jack"),
//         access_level: AccessLevel::User,
//     };

//     let user2 = User {
//         name: String::from("Ionesko"),
//         access_level: AccessLevel::Admin, 
//     };

//     let user1 = User {
//         name: name.to_string(),
//         access_level: if AccessLevel::Admin {
//             println!("Доброй пожаловать, Сэр!")
//         } else {
//             println!("баран"); 
//         }

//     };

//     println!("имя: {:?}\nуровень доступа: {:?}", user1.name, user1.access_level);


//     match user1.access_level {
//         AccessLevel::User => println!("{:?} is a user", user1.name),
//         AccessLevel::Admin => println!("{:?} is a admin", user1.name),
//     }

//     match user2.access_level {
//         AccessLevel::User => println!("{:?} is a user", user2.name),
//         AccessLevel::Admin => println!("{:?} is a admin", user2.name),
//     }

// }




/*

векторы, шкалики, винтики, нолики 
хрумкают чипсами два алкоголика 

это в конце станет всё так легко 
точно, известно, понятно, бесплотно,

если нет душ, значит, мы беспилотны
ясно одно 

 



ты идёшь, как Бродский по Сан-Марко,
вдоль пространства озера и парка,

в шляпе из густого перламутра
и в руках предмет похож на тумблер 

скоро ты присядешь на скамейку,
лебедей покормишь и на счастье
бросишь одногранную монетку, 
а потом уедешь в город Еткуль, 
где так много места, чтоб остаться,

прикоснуться к дереву сквозь шёпот 
воздуха в ночи молекулярной 
где по небу ходит антилопа 
шеститонным ямбом и автобус 
проезжает полукруг полярный 

и за полукругом в дождь на площадь
выходя, как Бродский - на Сан-Марко,
я скольжу во тьме к тебе наощупь 
ты скользишь во мне к себе на площадь:

раз в руке у нас одна фиалка 






но ведь это не в твоей натуре





и ты знаешь, это же так просто,



но ведь это не в твоей натуре


а потом расплакаться, проститься




чтобы долго-долго слушать Бога 




но ведь это не в твоей натуре 








где так много воздух, чтоб услышать 





или в том, что мрамор превратился 
наконец-то в кожу, кости, мясо,
взгляд, переходящий плавно в числа,
а за ними свет 


как взгляд Матисса 
(будто смотрят в небо водолазы 




)


на фактуру музыки 




а вокруг поэзия и числа 
пальцы над глазами пианиста 
облачностью 


кажется, не помню, Ашкенази?





а в глазах холмы в холмах холмятся 







дверь, куда ведущая - понятно 



и за стенкой женское контральто



речь у бога как всегда не складна 



а мне, знаешь, это непонятно 


или в том, что я в тебя влюбился 
вышел в дождь и там остановился 
пока шел он на меня 



а вокруг поэзия и числа 
солнце в спицах велосипедиста
а еще в глазах 







и везде все можно и бесплатно 

а потом ты всматриваясь утром




но ведь это же и так понятно 











облако почти что капиллярно 

ты идешь туда, сюда, оттуда 




музыка, сверкающая, словно 
жемчуг на ключицах птичьих, смертных 
я тебя люблю - я ем крыжовник 




размечая космос уголовный 
я стою во тьме, той тьме подобный 


музыка, сверкающая, словно 
жемчуг на ключицах или между 
строк читая космос уголовный 
я стою во тьме, той тьме подобный 
утешат 



и я здесь, проект твой семидневный 


и Ты здесь, такой не трансцендентный 

трансцендентный 

перед Тем 




кто дождь правобережный 



в центре ослепительности 


пред Тобой чей лик 




Любовью и Надеждой 






месят воздух словно футуриста 



, впрочем, бескорыстно


...

остальное вроде бы понятно 









облака и свет дрожит на линзе 



и умчишься в Рим, ну, или в Еткуль 


по пути из Рима, скажем, в Еткуль 







облачности два куска и сито 






свежесть разомкнутой шёпотом глины 




музыка, музыка, море, война,
радость и прочие наши утехи 
что ты там ищешь в пустом человеке 





и безотчётны 





*/







// fn main() {

//     enum Access {
//         Full,
//     }

//     fn one_two_three() -> (i32, i32, i32) {
//         (1, 2, 3,)
//     }

//     let numbers = one_two_three();
//     let (x, y, z) = one_two_three();

//     println!("{:?}, {:?}", x, numbers.0);
//     println!("{:?}, {:?}", y, numbers.1);
//     println!("{:?}, {:?}", z, numbers.2);

//     let (employee, access) = ("Jake", Access::Full);
// // а эта строка для примера, что мы можем объединять в кортеж разные типы данных,
// // в данном случае в кортеж обернули строковую переменную Jake и обозначение уровня доступа Access::Full 





// }
