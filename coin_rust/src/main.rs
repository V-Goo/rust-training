#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
let c = value_in_cent(Coin::Quarter(UsState::Alabama));
println!("Wow! {}", c);


let mut v = vec![1, 2, 3, 4, 5];

println!("The first element is: {:?}", v);
v.push(6); // Если поменять местами, будет ошибка
let first = &v[0];


println!("The first element is: {:?}", v);
println!("The first element is: {}", first);

let mut st = String::from("Hi");
let sr = String::from("Lo");
st = st + &sr;
println!("{}", st);

for c in "नमस्ते".chars() {
    println!("{}", c)
}
}
