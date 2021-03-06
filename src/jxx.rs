use std::fmt;

pub fn jxx_fix_offset(offset: u16) -> i16 {
    if offset & 0b10_0000_0000 > 0 {
        (offset | 0xfc00) as i16
    } else {
        offset as i16
    }
}

/// All jxx instructions implement this trait to provide a common interface
/// and polymorphism
pub trait Jxx {
    fn mnemonic(&self) -> &str;
    fn offset(&self) -> i16;
    fn size(&self) -> usize;
}

macro_rules! jxx {
    ($t:ident, $n:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $t {
            offset: i16,
        }

        impl $t {
            pub fn new(offset: i16) -> $t {
                $t { offset }
            }
        }

        impl Jxx for $t {
            fn mnemonic(&self) -> &str {
                $n
            }

            fn offset(&self) -> i16 {
                self.offset
            }

            fn size(&self) -> usize {
                2
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // LowerHex will treat hex numbers as unsigned so rather than
                // -0x6 we get 0xfffa. This is expected functionality and
                // unlikely to change. This is a working hack for now but we
                // should probably implement a better fix that is more
                // efficient https://github.com/rust-lang/rust/issues/42860
                if self.offset < 0 {
                    write!(f, "{} #-{:#x}", $n, -self.offset)
                } else {
                    write!(f, "{} #{:#x}", $n, self.offset)
                }
            }
        }
    };
}

jxx!(Jnz, "jnz");
jxx!(Jz, "jz");
jxx!(Jlo, "jlo");
jxx!(Jc, "jc");
jxx!(Jn, "jn");
jxx!(Jge, "jge");
jxx!(Jl, "jl");
jxx!(Jmp, "jmp");
