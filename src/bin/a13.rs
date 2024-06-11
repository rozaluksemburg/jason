// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
// просить ии переписывать задачи понятно
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector



fn main() {

    let pussy_collection = vec![10, 20, 30, 40];

    for bust4 in &pussy_collection {
        match bust4 {
            30 => println!("thirty"),
            _ => println!("{:?}", bust4),
        }
    }

    println!("количество элементов: {:?}", pussy_collection.len());
   


    }



// if *bust4 == 30 {
//     println!("thirty");
// } else {
//     println!("{:?}", bust4);
// }
// }

 // let qiantity = pussy_collection.len();
    // println!("{:?}", qiantity);

// struct Test {
//     score: i32,
// }

// fn main() {
//     let two = vec![
//         Test { score: 90 },
//         Test { score: 89 },
//         Test { score: 93 },
//         Test { score: 978 },
//     ];

//     for one in two {
//         println!("Оценка теста: {:?}", one.score);
//     }

// }









// fn main() {
//     let mut my_numbers = vec![1, 2, 3];

// // метод push добавляет вложенную в скобки цифру 
// // в конец вектора my_numbers то есть было 1 2 3
// // а стало 1 2 3 1
//     my_numbers.push(1);
//     my_numbers.push(2);
//     my_numbers.push(3);
// // метод pop удаляет последнюю цифру в нашем случае 3
// // из списка 1 2 3 1 2 3 - последнюю pop удаляет так 
// // что остается 1 2 4 1 2
//     my_numbers.pop();

// // метод len() просто выводит количество элементов в векторе    
//     let lenght = my_numbers.len();
//     println!("{:?}", lenght);

//     let two = my_numbers[1];
//     // 

//     let two_girl = vec![1, 2, 3];

//     for one_woman in two_girl {
//         println!("{:?}", one_woman); // вызывает 3 раза
//     }



// }
