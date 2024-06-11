use itertools::Itertools;

fn main() {
    let numbers = vec![1, 2, 3, 4, 6];
    let mut plus_one = vec![];
    for num in &numbers {
        plus_one.push(num + 1)
    }
    println!("{:?}", plus_one);
/*
    let plus_one: Vec<_> = numbers
    .iter() // итерирует извлекает поочереди из вектора numbers цифры
    .map(|num| + 1)
    .collect();
*/

    let plus_filter: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .filter(|num| *num < 4)
        .collect();
    println!("{:?}",plus_filter);


    let last = numbers
        .iter()
        .last();
    println!("{:?}", last);
// выводит последнюю цифру из вектора

    let min = numbers
        .iter()
        .min();
    println!("{:?}", min);
    // возвращает минимальное число из вектора numbers

    let max = numbers
        .iter()
        .max();
    println!("{:?}", max);
    // возвращает последнее число вектора numbers

    let sum: i32 = numbers.iter().sum();
    println!("{:?}", sum);
    // возвращает сумму всех итерируемых элементов в векторе numbers


    let average = sum as f64 / numbers.len() as f64;
    println!("{:?}", average);
    // то есть выведется отношение суммы чисел в векторе к количеству элементов

    let filtered: Vec<_> = numbers
        .iter()
        .filter(|num| *num % 2 == 0)
        .collect();
    println!("{:?}", filtered);

    let take: Vec<&i32> = numbers
        .iter()
        .take(3)
        .collect();
    println!("{:?}", take);
    // метод take() здесь выведет все числа до 3 включительно,
    // а все остальное 4 5 там - отметет

    let skip: Vec<&i32> = numbers
        .iter()
        .skip(3)
        .collect();
    println!("{:?}", skip);
    // а так указываем в skip(номер элемента, который мы не желаем отображать)

    let sorted: Vec<_> = numbers
        .iter()
        .cloned()
        .sorted()
        .collect();
    println!("{:?}", sorted);







}

/*
сов

сем отбился котик мой

а мне осталось над собой
разжечь костер, чтоб стал он мной

я ей подарил незабудку
но вышла она за турка
потом вышел я за турка
и стали мы жены турка
почище чем жены Лота
у должности демиурга
нет функций автопилота

я пылинки сдувал с ее белого лба






и стали мы штопать турки

потом наша корпорация
из турок и жен последних





у нас тут не любят в дурке
один говорит мне турок
как будто бы я придурок
как он но о том не знаю


я, мама, завтра с ночевой


 */