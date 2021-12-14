use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("open.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("open.txt").unwrap_or_else(|error| {
                panic!("Ой что деется: {:#?}", error);
            })
        } else {
            panic!("Ой что деется: {:#?}", error);
        }
    });

    // ? let f = File::open("hello.txt").unwrap();
    //? === То же, что и код ниже ===
    //? "```unwrap```, является сокращённым методом, который реализован прямо как выражение ```match```"
    //? let f = File::open("hello.txt").expect("Failed to open hello.txt");
    //? === То же, что и код выше но ===
    //? "Использование expect вместо unwrap с предоставлением хорошего сообщения об ошибке <...> делает более простым отслеживание источника паники."
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) =>
    // println!("{:#?}", f);
    // };

    println!("{:#?}", f);
}
