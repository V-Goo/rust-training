use std::fs;
use std::fs::File;
use std::io;
// use std::io::Read;
use std::io::ErrorKind;
use std::error::Error;
use std::net::IpAddr;


fn main() -> Result<(), Box<dyn Error>>{
    let home: IpAddr = "127..0.1".parse()?;
    // let home: IpAddr = "127..0.1".parse().expect("Я сказал IP адрес!!!");
    println!("{:#?}", home);
    let f = File::open("open.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("open.txt").unwrap_or_else(|error| {
                panic!("Ой что деется: {:#?}", error);
            })
        } else {
            panic!("Ой что деется: {:#?}", error);
        }
    });

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) =>
    // println!("{:#?}", f);
    // };
    
    //? === То же, что и код выше ===
    //? "```unwrap```, является сокращённым методом, который реализован прямо как выражение ```match```"
    // * let f = File::open("hello.txt").unwrap();

    //? === То же, что и код выше но ===
    //? "Использование expect вместо unwrap с предоставлением хорошего сообщения об ошибке <...> делает более простым отслеживание источника паники."
    //* let f = File::open("hello.txt").expect("Failed to open hello.txt");

    println!("{:#?}", f);
    // panic!("{:#?}",read_username_from_file());
    panic!("{:#?}",read_username_from_file_q())
    // read_username_from_file_q();
    // Ok(())
}

// Проброс ошибок. Передаём ошибку для обработки вызывающей функции
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
// Оператор "?", помещаемый после значения типа Result, работает почти таким же образом, как выражение match
// Ошибочные значения при выполнении методов с оператором "?" возвращаются через функцию from, определённую в типаже From стандартной библиотеки
// Данный типаж используется для конвертирования ошибок одного типа в ошибки другого типа. 
fn read_username_from_file_q() -> Result<String, io::Error> {
    // let mut s = String::new();
    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;
    // То же, но короче
    //  File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    // Всё это можно сделать одной строкой
    Ok(fs::read_to_string("hello.txt").expect("Кто здесь?"))
}
