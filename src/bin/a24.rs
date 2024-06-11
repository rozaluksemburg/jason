// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {

    let data: Vec<_> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|num| num * 3)
        .filter(|num| num > &10)
        .collect();

    for num in data {
        println!("{:?}", num);
    }



    let data1 = vec![1, 2, 3, 4, 5];

    let three_filter = data1
        .iter()
        .map(|num| num * 3)
        .filter(|num| *num > 10);

    for num in three_filter {
        println!("{:?}", num);
    }






}

/*

небо лишь отладочная версия
черновой прелюдии зрачка
у меня с тобой одна депрессия
а вторая корчит дурака

ты скользишь ступнями по ступенчатым
бритвам и чем выше, тем острей
разрезает зрением фасеточным
стрекоза танцующих людей






не Оккамы






 */