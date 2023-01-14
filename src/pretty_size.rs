use core::num::NonZeroUsize;

pub fn pretty_size(bytes: usize) -> String {
    macro_rules! invoke {
        ($unit:expr, $unit_str:literal) => {
            {
                // println!(".[{rest_1}][{rest_2}][{rest_3}]");

                //*/
                // 0 2 4 6
                // ___n.12
                let buffer = [0u8; 4 + 1 + 1 + 1 + $unit_str.len()];
                operate(prepare(bytes, $unit), buffer, *$unit_str)
            }
        }
    }

    fn convert_to_numeric_char(a: usize) -> u8 {
        match a {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'3',
            4 => b'4',
            5 => b'5',
            6 => b'6',
            7 => b'7',
            8 => b'8',
            9 => b'9',
            _ => unreachable!("{}", a)
        }
    }
    const GIB: usize = 1024 * 1024 * 1024;
    const MIB: usize = 1024 * 1024;
    const KIB: usize = 1024;

    #[inline]
    fn prepare(bytes: usize, unit: usize) -> (usize, usize, usize) {
        let x = bytes;
        let n = x / unit;
        let rest = x - (n * unit);
        let div_10 = unit / 10;
        let rest_1 = rest / div_10;
        let rest = rest - (rest_1 * div_10);
        let div_100 = unit / 100;
        let rest_2 = if rest != 0 {
            rest / div_100
        } else {
            0
        };
        let rest = rest - (rest_2 * div_100);
        let div_1000 = unit / 1000;
        let _rest_3 = if rest != 0 {
            rest / div_1000
        } else {
            0
        };
        // round
        /*
        let rest_2 = if (rest_3 >= 5) {
            rest_2 + 1
        } else {
            rest_2
        };
        */

        let (rest_2, rest_1) = if rest_2 >= 10 {
            ((rest_2 & 15) % 10, rest_1 + (rest_2 / 10))
        } else {
            (rest_2, rest_1)
        };
        let (rest_1, n) = if rest_1 >= 10 {
            ((rest_1 & 15) % 10, n + (rest_1 / 10))
        } else {
            (rest_1, n)
        };
        assert!(n <= 1024, "{n} <= 1024 (where: {x})");
        (n, rest_1, rest_2)
    }

    fn write_numeric_char(n: usize, pow: usize, bytes: &mut [u8], byte_index: usize, r: &mut usize, head: &mut Option<NonZeroUsize>) {
        if n >= pow {
            bytes[byte_index] = convert_to_numeric_char(*r / pow);
            *r -= *r / pow * pow;
            head.get_or_insert_with(|| (unsafe { NonZeroUsize::new_unchecked(byte_index + 1) }));
        }
    }

    #[inline]
    fn operate<const N: usize, const M: usize>((n, rest_1, rest_2): (usize, usize, usize), mut bytes: [u8; N], unit_bytes: [u8; M]) -> String {
        let mut r = n;
        let mut head: Option<NonZeroUsize> = None;

        // println!("3 {r}");
        write_numeric_char(n, 1000, &mut bytes, 0, &mut r, &mut head);
        write_numeric_char(n, 100, &mut bytes, 1, &mut r, &mut head);
        write_numeric_char(n, 10, &mut bytes, 2, &mut r, &mut head);
        write_numeric_char(n, 1, &mut bytes, 3, &mut r, &mut head);
        bytes[4] = b'.';
        bytes[5] = convert_to_numeric_char(rest_1);
        bytes[6] = convert_to_numeric_char(rest_2);
        if M != 0 {
            bytes[7..(7 + M)].copy_from_slice(&unit_bytes);
        };

        // SAFETY: we're just initialized `head` earlier, or a moment ago with fallback value.
        let head = unsafe { head.unwrap_unchecked() }.get() - 1;
        // SAFETY:
        // - b'0'..=b'9' is valid UTF-8 codepoint.
        // - `$unit_str` is `&str`, and wrote their byte sequence - it's also valid UTF-8 codepoint sequence.
        // - concatenating two valid UTF-8 sequences produces one valid UTF-8 sequence.
        unsafe { std::str::from_utf8_unchecked(&bytes[head..]).to_string() }
    }

    if bytes > 100 * GIB {
        unreachable!("domain constraints");
    }

    if bytes >= GIB {
        invoke!(GIB, b"GiB")
    } else if bytes >= MIB {
        invoke!(MIB, b"MiB")
    } else if bytes >= KIB {
        invoke!(KIB, b"KiB")
    } else {
        let mut buf = [0u8; 5];
        let mut rest = bytes;
        let mut head_index: Option<NonZeroUsize> = None;
        write_numeric_char(bytes, 1000, &mut buf, 0, &mut rest, &mut head_index);
        write_numeric_char(bytes, 100, &mut buf, 1, &mut rest, &mut head_index);
        write_numeric_char(bytes, 10, &mut buf, 2, &mut rest, &mut head_index);
        buf[3] = convert_to_numeric_char(rest % 10);
        // SAFETY: 4 != 0.
        head_index.get_or_insert_with(|| (unsafe { NonZeroUsize::new_unchecked(3 + 1) }));
        buf[4] = b'B';
        // SAFETY: we've just initialized head_index with some value.
        let head = unsafe { head_index.unwrap_unchecked() }.get() - 1;
        // SAFETY: convert_to_numeric_char yields only b'0'..b'9', which is valid codepoint for UTF-8 strings.
        unsafe { std::str::from_utf8_unchecked(&buf[head..]).to_string() }
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use crate::pretty_size::pretty_size;

    static TEST_BYTES: [usize; 19] =
        [0, 1, 9, 10, 99, 100, 999, 1000, 1023, 1024, 1536, 999_999, 1_000_000, 1_023_999, 1_024_000, 1024 * 1024, 999_999_999, 1_000_000_000, 1024 * 1024 * 1024];

    fn reference(bytes_base_of_two: usize) -> String {
        const KIB: f64 = 1_024.0f64;
        const MIB: f64 = KIB * 1_024.0f64;
        const GIB: f64 = MIB * 1_024.0f64;

        let bytes = bytes_base_of_two as f64;
        if bytes >= GIB {
            format!("{a:.2}GiB", a = bytes / GIB)
        } else if bytes >= MIB {
            format!("{a:.2}MiB", a = bytes / MIB)
        } else if bytes >= KIB {
            format!("{a:.2}KiB", a = bytes / KIB)
        } else {
            format!("{bytes}B")
        }
    }

    #[test]
    fn test() {
        for q in TEST_BYTES {
            println!("{q}");
            assert_eq!(reference(q), pretty_size(q))
        }
    }

    #[test]
    fn main() {
        for b in TEST_BYTES {
            // assert_eq!(reference(b), int_op(b));
            assert_eq!(reference(b), pretty_size(b));
        }

        println!("f: {:?}", bench_f());
//         println!("i: {:?}", bench_i());
        println!("i2: {:?}", bench_i2());
        bench_ix();
        println!("{}", reference(999999));
    }

    fn bench_f() -> std::time::Duration {
        let time = Instant::now();
        for i in 0..1048576 {
            reference(i);
        }

        time.elapsed()
    }

    fn bench_i2() -> std::time::Duration {
        let time = Instant::now();
        for q in 0..1048576 {
            pretty_size(q);
        }

        time.elapsed()
    }

    fn bench_ix() {
        for i in TEST_BYTES {
            let time = Instant::now();
            let f = reference(i);
            let time_f = time.elapsed();
            let time = Instant::now();
            let i = pretty_size(i);
            let time_i = time.elapsed();
            assert_eq!(f, i);
            println!("for {i}: float = {time_f:?} | int = {time_i:?}")
        }
    }
}