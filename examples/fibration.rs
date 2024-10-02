use variadic_t::{TypeList, mapping::{Map3, HasMap3}, variant::Tuple3};

fn main() {
    let input = 0;

    let type_select = |value: usize| {
        Tuple3::Variant0(value as u32)
    };
    let fibration: Tuple3<u32, f32, usize> = type_select(input);

    // println!("{:?}", fibration);
}
