struct Test
{
    x : i32,
    y : i32,
}

impl Test
{
    fn new() -> Test
    {
        print!("Init Test");
        Test{x: 0, y: 0}
    }

    fn get(&self) -> i32
    {
        0
    }
}

fn main() {
    let test = Test::new();
    
    println!("Hello, world! {}", test.x);
}
