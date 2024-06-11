// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut w = 1;
    loop {
        println!("{:?}", w);
        
        if w == 5 {
            break;
        }
        w = w + 1;
    }
    println!("+"); 
}














// fn main() {
//     let mut i = 3;
//     loop {
//         println!("{:?}", i);
//         i = i - 1;
//         if i == 0 { 
//             break;
//         }
        
//     }
//     println!("fa");
// }


// loop бесконечный цикл пока не наткнется на препятствие 
// а цикл while автоматически проверит некоторые условия 
// и как только у цикла while будет выполнено условие, то цикл прекратится 
// https://skrinshoter.ru/sPd1Km5R9yw

// let mut a = 0;
// loop { // этот блок будет выполнятся, пока a не станет равна 5 чтобы компилятор вышел из программы
//     if a == 5 {
//         break;
//     }
//     println!("{:?}", a);
//     a = a + 1;
// }

// // это выведет 0 1 2 3 4 5

// // https://skrinshoter.ru/sPdhviiXQ7I

// let mut a = 0;
// while a != 5 { //пока переменная a не равна 5
//     println!("{:?}", a); // то мы печатаем ее текущее значение 
//     a = a + 1; //
// }

/*
ну то есть while пока переводится то есть 
пока a не равна 5

то есть loop отличает от while тем что 

то есть loop это однофункциональный цикл без данных 
а в while мы уже можем вставлять данные 



*/

// let mut counter = 0;
 
// loop {
//     println!("we loop forever");
//     counter += 1;
//     if counter == 5 {
//         break; // выход из цикла после 5 итераций 
//     }
// }

// /* цикл while используется для повторения блока кода,
// пока условие остается истинным

// */

// let mut number = 3;

// println!("я считаю до нуля");

// while number != 0 { // пока number не равен 0, цикл ниже выполняется
    
//     println!("{number:?}");
//     number -= 1;
// }
// /*как только number становится равным 0,
// то происходит выход из программы вследствие того,
// что while выводит сообщения внутри фигурных скобок 
// только если выполняется условия, то если number не равен 0
// или если бы был равен 0 то есть мы бы так сделали,
// то это выглядело бы так  

// let number = 0;

// while number == 0 {
//     println!("ты выиграл"); 
// } 

// я запустил этот цикл и он был бесконечен 


// */
// println!("раз и ты выучил rust");

// //или так
// let sky = 0;

// while sky == 0 {
//     println!("ты выиграл"); 
//     break;
// } 


// } 