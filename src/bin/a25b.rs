// Тема -
// if let Some() и
// while let Some()

enum Color {
    Red,
    Blue,
    Green
}

fn main() {
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("{:?}", user),
        None => println!("no user")
    }

    if let Some(i) = maybe_user {
        println!("{:?}", i);
    } else {
        println!("бывает");
    }

    // if let хорош в ситуации, когда нужно найти что-то конкретное
    let red = Color::Red;
    if let Color::Red = red {
        println!("it's red");
    } else {
        println!("its not red");
    }







    /*
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("user: {:?}",user),
        None => println!("no user")
    }

    if let Some(user) = maybe_user {
        println!("user: {:?}", user);
    } else {
        println!("no user");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("it's blue");
    } else {
        println!("it's not red");
    }

    let mut data = Some(3);
    while let Some(i) = data {
        println!("loop");
        data = None;
        println!("{:?}", i);
    }
    println!("done");

    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();
    while let Some(num) = number_iter.next() {
        println!("число {:?}", num);
    }
    */
     */

        /*
        let numbers = vec![1, 2, 3]; // Создадим новый вектор numbers состоящий из чисел 1, 2 и 3

let mut number_iter = numbers.iter(); // Создадим итератор по вектору numbers, итератор позволяет последовательно проходить по элементам коллекции

// Используем конструкцию while let для итерации по нашему вектору
while let Some(num) = number_iter.next() { // number_iter.next() возвращает следующий элемент из вектора, если таковой ещё есть. Если элементов больше нет, возвращается None
    println!("число {:?}", num); // Печатаем текущее число. Вывод будет "число 1", "число 2", "число 3"
}





let numbers = vec![1, 2, 3];

let mut number_iter = numbers.iter();

while let Some(num) = number_iter.next() {
    println!("число {:?}", num);
}
         */

}

/*
Да, вы совершенно верно подметили! Оба подхода (и match, и if let) очень полезны и служат для различных случаев использования в Rust.
Конструкция if let удобна, когда вам нужно обработать только одно из множества возможных значений перечисления (например, только Some для типа Option) и проигнорировать все остальные варианты.
Конструкция match, с другой стороны, особенно полезна, когда вы хотите явно обработать все возможные варианты перечисления. Кроме того, Rust обеспечивает "исчерпывающую проверку" значений в блоках match, что означает, что компилятор убедится, что вы обрабатываете все возможные варианты.
В общем, две эти конструкции сопоставления с образцом обеспечивают мощные и гибкие способы работы с перечислениями в Rust. Знание, когда и как их использовать, может значительно упростить ваш код и сделать его более безопасным.




 я здесь затерян глубоко в горах
 мой позывной, как эхо, ох и ах



 */