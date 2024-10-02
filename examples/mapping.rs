use variadic_t::{variant::Tuple3, TypeList, mapping::HasMap3};

fn main() {
    let example: (u32, i16, u64) = (0u32, 1i16, 2u64);

    let coalesce: Vec<String> = example
        .fold_map(|variant| match variant {
            Tuple3::Variant0(it) => it.to_string(),
            Tuple3::Variant1(it) => it.to_string(),
            Tuple3::Variant2(it) => it.to_string(),
        })
        .collect();


    let project: (usize, String, u64) = example.map((
        |a: u32| a as usize,
        |b: i16| "hi ".repeat(b as usize),
        |c: u64| c * 100,
    ));
    
    println!("{:?}", coalesce);
}
