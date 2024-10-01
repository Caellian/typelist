use std::{marker::PhantomData, str::Chars};

pub struct Alphabet<'a> {
    pub letters: &'a str,
}

impl<'a> Default for Alphabet<'a> {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl<'a> Alphabet<'a> {
    pub const DEFAULT: Alphabet<'static> = Alphabet {
        letters: "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    };

    pub fn len(&self) -> usize {
        self.letters.len()
    }

    pub fn letters(&self) -> Chars<'a> {
        self.letters.chars()
    }

    pub fn encode(&self, mut value: usize) -> String {
        let base = self.len();
        let mut result = String::new();

        // Generate the alphabetic sequence like base-26 number
        loop {
            let remainder = value % base;
            result.push(self.letters().nth(remainder).unwrap());
            value /= base;
            if value == 0 {
                break;
            }
            value -= 1;
        }
        result.chars().rev().collect()
    }

    pub fn decode(&self, value: impl AsRef<str>) -> usize {
        let base = self.len();
        let mut index = 0;

        for c in value.as_ref().chars() {
            index *= base;
            index += self.letters.find(c).unwrap() + 1;
        }

        index - 1
    }
}

pub struct AlphabetGenerator<'a, T> {
    alphabet: Alphabet<'a>,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'a, T> Default for AlphabetGenerator<'a, T> {
    fn default() -> Self {
        Self::new(Alphabet::DEFAULT)
    }
}

impl<'a, T> AlphabetGenerator<'a, T> {
    pub fn new(alphabet: Alphabet<'a>) -> Self {
        Self {
            alphabet,
            index: 0,
            _phantom: PhantomData
        }
    }
}

impl<'a> Iterator for AlphabetGenerator<'a, String> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.alphabet.encode(self.index);
        self.index += 1;
        Some(value)
    }
}

macro_rules! impl_gen_parsed {
    ($($T: ty),+ $(,)?) => {
        $(
        impl<'a> Iterator for AlphabetGenerator<'a, $T> {
            type Item = $T;

            fn next(&mut self) -> Option<Self::Item> {
                let value = self.alphabet.encode(self.index);
                self.index += 1;
                Some(syn::parse_str(&value).expect(
                    concat!("unable to generate alphabet of type: ", stringify!($T))
                ))
            }
        }
        )+
    };
}
impl_gen_parsed![
    syn::Ident,
    syn::Type,
];
