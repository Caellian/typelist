use variadic_t::{TypeList, mapping::{Map3, HasMap3}};

fn main() {
    let elements: (&str, u32, i32) = ("a", 1u32, 16);

    let fibration: (usize, String, i32) = elements.map((
        |a: &str| a.len(),
        |b: u32| "hi ".repeat(b as usize),
        |c: i32| c * 100,
    ));

    println!("{:?}", fibration);
}
