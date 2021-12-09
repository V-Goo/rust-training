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

    // в новой интерпретации, функция имеет в качестве параметра ссылку на объект вместо того, чтобы забирать его во владение

    let s6 = String::from("hello");

    let len = calculate_length_link(&s6);

    println!("The length of '{}' is {}.", s1, len);

    
    
    let mut s7 = String::from("hello");

    // let r2 = &mut s7;
    // {
    //     // r1 goes out of scope here, so we can make a new reference with no problems.
    //     let r1 = &mut s7;
    // } 
    // println!("{}, {}", r1, r2)

    // область видимости ссылочной переменной начинается от места, где она создана и продолжается до места где она последний раз использована. Например, данный код будет скомпилирован, потому что последнее использование неизменяемой ссылки происходит перед тем, как появляется изменяемая ссылка
    let r2 = &mut s7;
    println!("{}", r2);
    let r1 = &mut s7;
    println!("{}", r1,);

    // если у вас есть ссылка на какие-то данные, компилятор обеспечит что эти данные не выйдут из области видимости прежде, чем из области видимости исчезнет ссылка.

// *  Slises
    let f0 = String::from("Hello blin!");

    let f = first_word_no_slise(&f0);
    println!("{}", f);
    
    let f1 = first_word(&f0);
    println!("{}", f1);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word1 = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word2 = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word3 = first_word(my_string_literal);
    println!("{} {} {}", word1, word2, word3)
}

//TODO Functions

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_no_slise(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn calculate_length_link(s: &String) -> usize {
    s.len()
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
