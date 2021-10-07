// MIT License
//
// Copyright (c) 2021 Exercism
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Все упоминания `PhantomData` в этом файле можно убрать, они только для того,
// чтобы текущий код компилировался

use std::ops::Rem;

/// Правило для FizzBuzz: с помощью заданного предиката мы проверяем, должен ли
/// текущий элемент T быть заменен на слово? Если да, то на какое?
pub struct Matcher<T> {
    predicate: Box<dyn Fn(&T) -> bool>,
    substitution: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S: ToString>(_predicate: F, _substitute_with: S) -> Matcher<T>
    where
        F: Fn(&T) -> bool + 'static,
    {
        Self {
            predicate: Box::new(_predicate),
            substitution: _substitute_with.to_string(),
        }
    }
    pub fn apply(&self, val: &T) -> Option<String> {
        if (*self.predicate)(val) {
            Some(self.substitution.clone())
        } else {
            None
        }
    }
}

/// Набор правил Matcher, которые можно применить к итератору.
///
/// Более идиоматично использовать метод `.map()` для модификации итератора
/// вместо метода `Fizzy::apply()`, который этот итератор поглощает.
///
/// Зато можно попрактиковаться с более простым интерфейсом `apply`.
#[derive(Default)]
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // можете использовать `mut self` в сигнатуре, если вам нравится
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    /// Применяет набор Matchers к данному значению
    pub fn apply_elem(&self, elem: &T) -> String {
        let res = self
            .matchers
            .iter()
            .map(|matcher| matcher.apply(elem))
            .flatten()
            .collect::<Vec<_>>()
            .concat();
        if res.is_empty() {
            elem.to_string()
        } else {
            res
        }
    }
    /// Применяет набор Matchers к каждому элементу итератора
    pub fn apply<I>(&self, _iter: I) -> FizzyIterator<I>
    where
        I: Iterator<Item = T>,
    {
        FizzyIterator {
            fizzy: self,
            iter: _iter,
        }
    }
}

pub struct FizzyIterator<'a, I: Iterator> {
    fizzy: &'a Fizzy<I::Item>,
    iter: I,
}

impl<'a, I: Iterator> Iterator for FizzyIterator<'a, I>
where
    I::Item: ToString,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|elem| self.fizzy.apply_elem(&elem))
    }
}

/// Вспомогательная функция: возвращает `Fizzy` со стандартными правилами FizzBuzz
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: From<u8> + PartialEq + Rem<Output = T> + Copy + Clone + ToString,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| *n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| *n % 5.into() == 0.into(), "buzz"))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expect {
        () => {
            vec![
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz",
                "13", "14", "fizzbuzz", "16",
            ]
        };
    }

    #[test]
    fn test_simple() {
        let got = fizz_buzz::<i32>().apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_u8() {
        let got = fizz_buzz::<u8>().apply(1_u8..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_u64() {
        let got = fizz_buzz::<u64>().apply(1_u64..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];
        let expect = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];
        let got = fizz_buzz::<i32>()
            .apply(collatz_12.iter().cloned())
            .collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_custom() {
        let expect = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];
        let fizzer: Fizzy<i32> = Fizzy::new()
            .add_matcher(Matcher::new(|n: &i32| n % 5 == 0, "Buzz"))
            .add_matcher(Matcher::new(|n: &i32| n % 3 == 0, "Fizz"))
            .add_matcher(Matcher::new(|n: &i32| n % 7 == 0, "Bam"));
        let got = fizzer.apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_f64() {
        // a tiny bit more complicated because range isn't natively implemented on floats
        let got = fizz_buzz::<f64>()
            .apply(std::iter::successors(Some(1.0), |prev| Some(prev + 1.0)))
            .take(16)
            .collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_minimal_generic_bounds() {
        use std::fmt;
        use std::ops::{Add, Rem};

        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        struct Fizzable(u8);

        impl From<u8> for Fizzable {
            fn from(i: u8) -> Fizzable {
                Fizzable(i)
            }
        }

        impl fmt::Display for Fizzable {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let Fizzable(ref n) = self;
                write!(f, "{}", n)
            }
        }

        impl Add for Fizzable {
            type Output = Fizzable;
            fn add(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 + n2)
            }
        }

        impl Rem for Fizzable {
            type Output = Fizzable;
            fn rem(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 % n2)
            }
        }

        let got = fizz_buzz::<Fizzable>()
            .apply(std::iter::successors(Some(Fizzable(1)), |prev| {
                Some(*prev + 1.into())
            }))
            .take(16)
            .collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_map() {
        let fizzbuzz = fizz_buzz::<i32>();
        let got = (1..=16)
            .map(|x| fizzbuzz.apply_elem(&x))
            .collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }
}
