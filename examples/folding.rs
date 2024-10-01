use variadic_t::{TypeList, variant::Tuple3};

fn main() {
    let example: (u32, i16, &str) = (0u32, 1i16, "A");
    for item /* Tuple3<u32, i16, &str> */ in example.fold() {
        match item {
            Tuple3::Variant0(a) => {
                println!("{}", a);
            },
            Tuple3::Variant1(b) => {
                println!("{}", b);
            },
            Tuple3::Variant2(c) => {
                println!("{}", c);
            },
        }
    }
}
