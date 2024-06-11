// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase

// библиотеку можно открыть в терминале командой rustup doc --open

// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let kiss = "Ionesko";
    println!("{:?}", kiss.to_lowercase());
    println!("{:?}", kiss.to_uppercase());
    

}











/*
fn main() {
    let numbers = vec![1, 2, 3];
    match numbers.is_empty() { // метод is_empty() проверяет переменную пуста ли она
        // если numbers пуста, значит, это правда true и выводим следующее сообщение
        true => println!("no numbers"),
        false => println!("has numbers")
    }
}
*/


/*
    /// три косых черты говорят что так мы заносим этот блок в документацию
    enum Color {
        Red,
        Blue
    }

    /// кусочек начала ящика
    struct Mail {
        /// адрес
        address: String,
    }

    /// некая сложительная функция
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

*/





/*



ты обучался плеску у воды
по крайней мере, так оно могло
казаться,



let mut v = Vec::new();
assert!(v.is_empty());

v.push(1);
assert!(!v.is_empty());

и

let a = [1, 2, 3];
assert!(!a.is_empty());

let b: &[i32] = &[];
assert!(b.is_empty());



 */