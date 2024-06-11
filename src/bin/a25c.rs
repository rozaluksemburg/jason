fn main() {
    let mut data = Some(3);
// конструкция while let чрезвычайно полезна
    // при работе с итераторами

    while let Some(value) = data {
        println!("loop");
        data = None;

    }
    println!("done");

    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();
    while let Some(number) = number_iter.next() {
        println!("iteration: {:?}", number);
    }


}