use blog_type::Post;

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}
// impl Message {
// 	fn new() {
// 		Message {
// 			// Quit,
// 			Move: { x: 3, y: 5},
// 			Write: ("Text"),
// 			ChangeColor: (0, 0, 0),
// 		}
// 	}
// }

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    // assert_eq!("", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());

		let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

		let mut  msg = Message::Quit;
		msg = Message::Write(String::from("Text"));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    } 
}
