
#[derive(Debug)]
struct Gold {
    player_name: String,
    possition: String,
    age: u8
}

fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);
    // метод some() проверяет есть ли в переменной a значение
    // метод выдает либо true либо false
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    
    let a_is_none = a.is_none();
    // если есть значение значит a_is_none = false
    // если нет значения то a_is_none = true
    dbg!(a_is_none);

    let a_mapped = a.map(|num| num + 1);
    // замечательно в map то что она добавит 1 если переменная a не пуста 
// то есть если в переменной есть значение то map сработает, если нет - то просто пропустится
// здесь интересно то что метод file
        dbg!(a_mapped);

    let a_filtered = a.filter(|num| num == &1);
// а так мы проверяем равна ли переменная a == &1
// если да то a_filtered станет указанным значением & то есть 1
// то есть если переменная a - равна 1 то и a_filtered равен 1
        dbg!(a_filtered);

        let a_or_else= a.or_else(|| Some(5));
    // если у a нет значения или значение None 
    // то a_or_else станет Some(5) а 
    // то есть используем or_else(|| ) если a не возвращает значения но нам нужно
    // чтобы во что бы то ни стало возвращалось какое-то значение не пустое 
    dbg!(a_or_else);

    let unwrapped = a.unwrap_or_else(|| 0);
    // метод для option unwrap_or_else не принимает метод or_else в качестве аргумента
    dbg!(unwrapped);
    // unwrap_or_else присваивает переменной значение a если оно есть
    // если значение отсутствует то переменной присваивается то что мы укажем после ||
    // в данном случае это 0
    
    let b: Option<Gold> = None;
    let unwrap = b.unwrap_or_else(|| Gold {
        player_name: "Charly".to_owned(),
        possition: "Forward".to_owned(),
        age: 34
    });

   

    dbg!(unwrap);











    println!("{:?}", a_mapped);
}