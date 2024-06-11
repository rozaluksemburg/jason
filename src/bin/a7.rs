// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
// Также, если вам больше не нужны предупреждения о неиспользуемых 
// присваиваниях, вы можете отключить этот конкретный вид предупреждений 
// на уровне файла или модуля, добавив 
// #![allow(unused_assignments)] в начало файла.
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


enum Direction {
    Green,
    Blue,
}

fn print_color(color: Direction) {
    match color {
        Direction::Green => println!("green"),
        Direction::Blue => println!("blue"),
    }
}

fn main() {
print_color(Direction::Green); 
print_color(Direction::Blue); 

}






// enum Direction {
//     Left,
//     Right,
//     // Up, если был бы этот пункт, то компилятор выдал бы ошибку,
//     // потому как Up нет в переборе match всех вариантов enum
// }

// fn main() {
//     let go = Direction::Right;
//     // присваиваем конкретный пункт из перечисления enum переменной go
//     // и далее её перебираем
//     match go {
//         Direction::Left => println!("go left"),
//         Direction::Right => println!("go right"),
//         // Direction::Up => println!("go up"),
//     }
// }













// fn main() {
// enum Direction {
//     Green,
//     Yellow,
//     Blue
// }

// fn print_color(color: Direction) {
//     match color {
//         Direction::Green => println!("Green"),
//         Direction::Yellow => println!("Yellow"),
//         Direction::Blue => println!("blue"),
//     }
// }

// let color = Direction::Blue;
// print_color(color);



// }






// fn main() {
//     enum Direction { // это значит, что переменная Direction может принимать ОДНО из четырёх значений
//         Up,
//         Down,
//         Left,
//         Right
//     }
// // перечисления 
//     fn which_way(go: Direction) {
//         match go {
//             Direction::Up => println!("up"),
//             Direction::Down => println!("down"),
//             Direction::Left => println!("Left"),
//             Direction::Right => println!("Right"),
//         }
//     }

//     let mut go = Direction::Up;
//     which_way(go);

//     go = Direction::Down;
//     which_way(go);

//     go = Direction::Left;
//     which_way(go);

//     go = Direction::Right;
//     which_way(go); 


// }
