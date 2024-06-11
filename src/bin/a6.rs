// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop



fn main() {
    let mut io = 5;
    while io >= 1 {
        println!("{:?}", io);
        io = io - 1;
    } // то есть пока while io >= 1,
    // выполняются операции в {} и дальше компилятор 
    // возвращается вверх и так до того момента пока io переменная
    // не станет меньше одного, тогда если io будет 0
    // то это не будет удовлетворять условию что io равна или больше 1
    // потому что 0 больше одного вроде как))
    println!{"done!"};
}










// fn main() {
//     let mut i = 0;
//     // пока while переменная i меньше или равна 3, то выполняется условие в фигурных скобках
//     while i <= 3 {
//         println!("{:?}", i);
//         i = i + 1;
//     }
//     //когда i станет равна 4 то программа не выполнит печати и прибавления
//     // { println!("{:?}", i); i = i + 1; } 
//     // потому что i не меньше и не равна 3 теперь - ну 4 не меньше и не равно 3 

    


// }
