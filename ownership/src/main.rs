fn main() {
// Литерал на стеке. Типы данных, размер которых известен во время компиляции, сохраняются полностью в стеке, поэтому такое копирование значений является быстрым. 
    let s1 = "hello";
    let s2 = s1;
    println!("s1 = {}, s2 = {}", s1, s2);
    
// Данные в куче. 
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // Передача переменной в функцию в качестве входного параметра будет перемещать или копировать её значение, точно также как это делает операция присвоения. 
    // println!("{}", s); // Если попытаться использовать s после вызова takes_ownership, Rust выдаст ошибку времени компиляции. 

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("{}", x);       
    
    // Есть возможность возвращать несколько значений используя кортеж

    let s4 = String::from("hello");

    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s5, len);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} 

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
