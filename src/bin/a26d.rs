// Topic: Inline Modules
//
// Summary:
// The existing program is complete, but all the code exists
// in a single module. This code can benefit from being organized
// into multiple modules.
//
// Requirements:
// * Organize the code into two modules based on their functionality:
//   - msg: string formatting functions
//   - math: math functions
// * Update the main function to use the functionality from the modules
//
// Notes:
// * After moving the functions into modules, try running
//   `cargo check --bin a26b` to get a listing of required code changes

// Тема: Встроенные модули
//
// Резюме:
// Существующая программа завершена, но весь код существует
// в одном модуле. Этот код можно выгодно организовать
// в несколько модулей.
//
// Требования:
// * Организуйте код в два модуля на основе их функциональности:
// - msg: функции форматирования строк
// - math: математические функции
// * Обновите главную функцию, чтобы она использовала функциональность модулей.
//
// Примечания:
// * После перемещения функций в модули попробуйте выполнить команду
// `cargo check --bin a26b`, чтобы получить список необходимых изменений кода

/*
я наблюдаю дождь в агонии холма
он медленно глядит глазами сентября 
на музыку внутри 

*/





fn main() {
    // Part 1: math functions
    use activity::math;

    let result = {
        let two_plus_two = math::add(2, 2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three)
    };

    // Ensure we have a correct result.
    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    {
         // Part 2: string functions
        use activity::msg::{trim, capitalize, exciting};

    let hello = {
        let msg = "hello ";
        let msg = trim(msg);
        capitalize(msg)
    };
    let world = {
        let msg = "world";
        exciting(msg)
    };
    let msg = format!("{}, {}", hello, world);

    // Ensure we have a correct result.
    assert_eq!(&msg, "Hello, world!");
    println!("{}", msg);
}

    }






   
    

