///! Helpful tools for working with ASCII strings.

// Working with single characters
// ------------------------------
pub trait CharExt {
    fn letter_to_num<T>(&self) -> T
    where
        T: std::convert::From<u8>;

    fn digit_to_num<T>(&self) -> T
    where
        T: std::convert::From<u8>;
}

impl CharExt for char {
    /// Converts an ASCII letter to it's numeric value.
    /// A = 0, B = 1, C = 2, ..., Z = 25
    /// a = 0, b = 1, c = 2, ..., z = 25
    ///
    /// # Panics
    ///
    /// If the character is not an ASCII letter.
    ///
    /// ```
    /// use aoc::stringstuff::CharExt;
    ///
    /// assert_eq!('A'.letter_to_num::<u8>(), 0u8);
    /// assert_eq!('a'.letter_to_num::<i32>(), 0i32);
    /// assert_eq!('M'.letter_to_num::<u64>(), 12u64);
    /// assert_eq!('z'.letter_to_num::<isize>(), 25isize);
    /// ```
    fn letter_to_num<T>(&self) -> T
    where
        T: std::convert::From<u8>,
    {
        let c = *self as u8;
        if c >= b'A' && c <= b'Z' {
            T::from(c - b'A')
        } else if c >= b'a' && c <= b'z' {
            T::from(c - b'a')
        } else {
            panic!("Not a letter: {}", self);
        }
    }

    /// Converts an ASCII digit to it's numeric value.
    /// 0 = 0, 1 = 1, 2 = 2, ..., 9 = 9
    ///
    /// # Panics
    ///
    /// If the character is not an ASCII digit.
    /// ```
    /// use aoc::stringstuff::CharExt;
    ///
    /// assert_eq!('0'.digit_to_num::<u8>(), 0u8);
    /// assert_eq!('1'.digit_to_num::<i32>(), 1i32);
    /// assert_eq!('5'.digit_to_num::<u64>(), 5u64);
    /// assert_eq!('9'.digit_to_num::<isize>(), 9isize);
    /// ```
    fn digit_to_num<T>(&self) -> T
    where
        T: std::convert::From<u8>,
    {
        let c = *self as u8;
        if c >= b'0' && c <= b'9' {
            T::from(c - b'0')
        } else {
            panic!("Not a digit: {}", self);
        }
    }
}

