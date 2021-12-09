#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_struct_1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect_struct_2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect_struct_3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect_struct_1.can_hold(&rect_struct_2));
    println!("Can rect1 hold rect3? {}", rect_struct_1.can_hold(&rect_struct_3));
    
    println!("\nrect_struct is {:?}\n", rect_struct);
    println!("rect_struct is {:#?}", rect_struct);

    println!(
        "The area of the rectangle_struct is {} square pixels.",
        area_struct(&rect_struct)
    );

    
    println!(
        "\nThe area  of the rectangle is {} square pixels.",
        rect_struct.area()
    );
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
