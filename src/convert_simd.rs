use std::simd::{u8x4, u32x4, u64x4, FromBits};

use convert::{POW10_U32, POW10_U64, ASCII_TO_INT_FACTOR, POW10_U32_LEN, POW10_U64_LEN};

const ASCII_TO_INT_VEC: u8x4 = u8x4::new(ASCII_TO_INT_FACTOR, ASCII_TO_INT_FACTOR, ASCII_TO_INT_FACTOR, ASCII_TO_INT_FACTOR);
const RANGE: u8x4 = u8x4::new(9, 9, 9, 9);

pub trait FromAsciiSIMD: Sized {
    fn atoi_simd<S: AsRef<[u8]>>(s: S) -> Result<Self, ()> {
        Self::bytes_to_int_simd(s.as_ref())
    }

    fn bytes_to_int_simd(bytes: &[u8]) -> Result<Self, ()>;
}

macro_rules! impl_unsigned_conversion_simd {
    ($int:ty, $simd_type:ty, $const_table:ident, $const_table_len:ident) => (
        impl FromAsciiSIMD for $int {
            fn bytes_to_int_simd(mut bytes: [u8]) -> Result<Self, ()> {
                
                let mut len = bytes.len();
                let mut idx = $const_table_len - len;
                
                let mut simd_result = $simd_type::new(0, 0, 0, 0);

                while len >= 4 {
                    unsafe {

                        // load 4 items from the slice into a simd type.
                        let mut parse_current = u8x4::load_aligned_unchecked(bytes.get_unchecked(..4));
                        
                        // substract 48 of each of the loaded items
                        parse_current -= ASCII_TO_INT_VEC;

                        // check any if them is larger than 9, if so: return an Error.
                        if v.gt(RANGE).any() {
                            return Err(());
                        }

                        let multiply = $simd_type::load_unaligned_unchecked($const_table.get_unchecked(idx..idx + 4));
                        let mut r = $simd_type::from(v) * multiply;
                        
                        simd_result += r;
                        len -= 4;
                        idx += 4;

                        bytes = &bytes.get_unchecked(4..);
                    }
                }

                let mut result = simd_result.wrapping_sum();
                
                for offset in 0..len {
                    unsafe {
                        let d = bytes.get_unchecked(offset).wrapping_sub(ASCII_TO_INT_FACTOR) as Self;
                        if d > 9 {
                            return Err(());
                        }

                        let r = d * $const_table.get_unchecked(idx + offset);
                        result += r;
                    }
                }

                return Ok(result);
            }
        }
    )
}

impl_unsigned_conversion_simd!(u32, u32x4, POW10_U32, POW10_U32_LEN);
impl_unsigned_conversion_simd!(u64, u64x4, POW10_U64, POW10_U64_LEN);