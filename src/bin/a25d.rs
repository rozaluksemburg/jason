
use std::collections::HashMap;
// прописывая так - это принадлежит только главной функции
// для модулей нужно прописывать используемые компоненты в самом модуле, как это сделано ниже


mod greet {
    use std::collections::HashMap;
    pub fn hello() {
        println!("hello");
    }

    pub fn goodbay() {
        println!("goodbay");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}


fn main() {
    use greet::*;
    // обозначает, что нам доступны все функции модуля greet
    hello();
    use math::*;
    let multi = multiply(3, 5);
    println!("{:?}", multi);
}