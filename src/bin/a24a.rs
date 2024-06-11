

fn main() {
    let numbers = vec![1, 2, 3, 4, 5]; 
    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    
    
    let plus_one: Vec<_> = numbers
    .iter() // итерирует извлекает поочереди из вектора numbers цифры
    .filter(|num| num <= 3) // bool то есть или true или false возвращает
    // и здесь мы можем задавать условие если num (одно из значений numbers, которую мы перебираем при помощи метода iter() выше)
    // скажем можно сделать, чтобы значения при итерации  принимались только 
    // меньше или равно 3 то есть в итоге plus_one будет равна только 1 2 а 3 4 5 не входят в это условие, поэтому и не просваивается переменной plus_one 
    .collect();

    let numbers = vec![1, 2, 3, 4, 5];

    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .collect();

    let new_numbers: Option<&i32> = numbers
        .iter()
        .find(|num| *num == 3 );
    //скажем если бы написали .find(|num| num == 40)
    // то результатом find вернулось бы option в значении None 
    //потому что в векторе numbers, который мы перебираем, нет 40
    
    let count = numbers
    .iter()
    .count();
    //возвращает количество элементов в векторе numbers

    let last = numbers
    .iter()
    .last();
    
    }










// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];
//     let plus_one: Vec<_> = numbers.iter().map(|&num| num + 1).collect();
// }

/*
так пахнут осенью господние луга 
так небо в дождь скользит сквозь берега 

*/