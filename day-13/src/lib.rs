use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

pub fn compare_packets(a: &str, b: &str) -> Ordering {
    let mut chars_a = a.chars().peekable();
    let mut chars_b = b.chars().peekable();
    loop {
        let a_value = next_value(&mut chars_a);
        let b_value = next_value(&mut chars_b);
        let ordering = a_value.cmp(&b_value);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    fn next_value(chars: &mut Peekable<Chars>) -> (i32, i32) {
        let mut integer = None;
        let mut level = 0;
        loop {
            let c = chars.peek().unwrap();
            match c {
                ']' => match integer {
                    Some(i) => {
                        return (i, level);
                    }
                    _ => {
                        chars.next();
                        level -= 1;
                        return (-1, level);
                    }
                },
                '[' => {
                    chars.next();
                    level += 1
                }
                c if c.is_ascii_digit() => {
                    let integer = integer.get_or_insert(0);
                    *integer *= 10;
                    *integer += (*c as u32 - '0' as u32) as i32;
                    chars.next();
                }
                _ => {
                    chars.next();
                    if let Some(i) = integer {
                        return (i, level);
                    }
                }
            }
        }
    }
}
