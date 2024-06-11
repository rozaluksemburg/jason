// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// // * Use a function to add two numbers together
// fn sum(x: i32, y: i32) -> i32 {
//     x + y   
// }

// // * Use a function to display the result
// fn display_result(result: i32) {
//     println!("{:?}", result);
// }

// fn main() {
//     let result = sum(8, 3);
//     display_result(result);
// }
// как это работает визуально https://skrinshoter.ru/sPfCcOcB44B

/*
а зачем все эти танцы с двумя функциями если можно было сделать проще?
сделать одну функцию вычисляющую значение параметров, переданных 
в функцию в качестве аргументов и присвоенные значению новой переменной,
которую далее мы можем распечатать просто?

Ваш вопрос весьма оправдан, сэр! Разделение логики на функции в вашем примере может показаться излишним для такой простой задачи, как сложение двух чисел. Однако, разделение функций на более мелкие, выполняющие одну конкретную задачу, является частью хороших практик программирования и может приносить множество преимуществ, особенно в больших и сложных проектах. Вот несколько причин, почему это может быть полезно:

Читаемость: Более мелкие функции обычно легче читать и понимать. Когда функция выполняет только одну задачу, легче понять, что она делает, и каков её вклад в общую логику программы.
Повторное использование кода: Если вам потребуется использовать функцию сложения в других местах программы, вы сможете вызвать уже написанную функцию sum, вместо того чтобы дублировать код. Это уменьшает вероятность ошибок и упрощает поддержку кода.
Тестирование: Функции с чётко определёнными задачами легче тестировать. Вы можете написать тесты для функции sum, чтобы убедиться, что она работает правильно в различных условиях, не заботясь о других частях программы.
Отладка: Если в вашем коде возникает ошибка, наличие чётко разделённых функций может помочь быстрее найти и устранить проблему. Например, если ошибка возникает при выводе результата, вы сразу знаете, что проблема в функции display_result.
Масштабируемость: В более сложных системах функция сложения может стать более комплексной (например, добавить обработку ошибок или расширенную логику для работы с большими числами). В этом случае, отдельная функция sum облегчит управление изменениями.
Хотя для небольших или учебных проектов такой подход может показаться излишним, применение этих практик с самого начала может помочь вам развить хорошие привычки, которые пригодятся в более крупных и сложных проектах. 
*/


// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


// * Use a function to add two numbers together
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// * Use a function to display the result
fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let result = sum(8, 1);
    display_result(result);
}