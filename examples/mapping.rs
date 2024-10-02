use variadic_t::{variant::Tuple3, TypeList};

fn main() {
    let example: (u32, i16, u64) = (0u32, 1i16, 2u64);

    let coalesce: Vec<String> = example
        .fold_map(|variant| match variant {
            Tuple3::Variant0(it) => it.to_string(),
            Tuple3::Variant1(it) => it.to_string(),
            Tuple3::Variant2(it) => it.to_string(),
        })
        .collect();
    println!("{:?}", coalesce);
}
