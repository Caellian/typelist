use variadic_t::TypeList;

fn main() {
    let example: (&str, i32, i32, [&str; 3]) = ("a", 1, 2, ["hello", " ", "world"]);

    // This only works because `example` is fully resolved.
    #[allow(clippy::let_unit_value)]
    let example = if example.len() > 0 {
        let (greeting, example): ([&str; 3], (&str, i32, i32)) = example.pop_back();
    
        for item in greeting {
            println!("{}", item)
        }

        let (first, rest): (&str, (i32, i32)) = example.pop_front();
        let (second, rest): (i32, (i32,)) = rest.pop_front();
        let (third, rest): (i32, ()) = rest.pop_front();
        // let (fourth, rest) = rest.pop_front(); // compile error: pop_front is not defined.

        rest
    } else {
        unreachable!()
    };

    let example: (&str,) = example.push_front("better");
    let example: (&str, &str) = example.push_front("harder");
    let example: (&str, &str, &str) = example.push_back("faster");
    let example: (&str, &str, &str, &str) = example.push_back("stronger");

    println!("{:?}", example); // harder, better, faster, stronger

    let (value, _) = example.remove::<2>();
    assert_eq!(value, "faster");
    // Isn't that cool even if completely useless?

    /*
        Why? Because we didn't infer _anything_. All that type information was present
        from the beginning and we basically just grouped variables under a identifier.

        Something like this can't be done:
        fn do_something<T>(t: T) where T: TypeList {
            if t.len() > 0 {
                let (greeting, example) = example.pop_back(); // compiler error: missing NonEmpty bound
            
                for item in greeting {
                    println!("{}", item)
                }
            }
        }

        Making `pop_back` panic instead of not being implemented would allow less
        strict `TypeList` specification, but value returned by `pop_back` from
        generic `TypeList` is not constrained which makes it completely opaque
        and useless.

        For it to be usable, there needs to exist a way of restricting TypeList
        parameters. This _can be done_ on a per-use basis, but this means that each
        crate has to implement its own `TypeList` for every kind of type list they
        want to use.
    */
}
