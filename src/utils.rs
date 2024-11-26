
use crate::constants::CIPHERTEXT_LENGTH;

pub fn crypto_sort_uint32(array: &mut [u32]) {
    let n = array.len();
    for j in 0..n {
        array[j] ^= 0x80000000;
    }
    let array_i32: &mut [i32] = unsafe { std::mem::transmute(&mut *array) };
    crypto_sort_int32(array_i32);

    for j in 0..n {
        array[j] ^= 0x80000000;
    }
}

#[inline]
fn minmax(a: &mut i32, b: &mut i32) {
    let ab = *b ^ *a;
    let mut c = (*b as i64 - *a as i64) as i32;
    c ^= ab & (c ^ *b);
    c >>= 31;
    c &= ab;
    *a ^= c;
    *b ^= c;
}

/* assume 2 <= n <= 0x40000000 */
pub fn crypto_sort_int32(x: &mut [i32]) {

    let n = x.len() as i64;
    let mut top = 1;
    while top < n - top {
        top += top;
    }

    let mut p = top;
    while p >= 1 {
        let mut i = 0;
        while i + 2 * p <= n {
            for j in i..i + p {
                let (left, right) = x.split_at_mut((j + p) as usize);
                minmax(&mut left[j as usize], &mut right[0]);
            }
            i += 2 * p;
        }
        for j in i..n - p {
            let (left, right) = x.split_at_mut((j + p) as usize);
            minmax(&mut left[j as usize], &mut right[0]);
        }

        let mut i = 0;
        let mut j = 0;
        let mut q = top;
        while q > p {
            if j != i {
                loop {
                    if j == n - q {
                        break;
                    }
                    let mut a = x[(j + p) as usize];
                    let mut r = q;
                    while r > p {
                        minmax(&mut a, &mut x[(j + r) as usize]);
                        r >>= 1;
                    }
                    x[(j + p) as usize] = a;
                    j += 1;
                    if j == i + p {
                        i += 2 * p;
                        break;
                    }
                }
            }
            while i + p <= n - q {
                for j in i..i + p {
                    let mut a = x[(j + p) as usize];
                    let mut r = q;
                    while r > p {
                        minmax(&mut a, &mut x[(j + r) as usize]);
                        r >>= 1;
                    }
                    x[(j + p) as usize] = a;
                }
                i += 2 * p;
            }
            // now i + p > n - q
            j = i;
            while j < n - q {
                let mut a = x[(j + p) as usize];
                let mut r = q;
                while r > p {
                    minmax(&mut a, &mut x[(j + r) as usize]);
                    r >>= 1;
                }
                x[(j + p) as usize] = a;
                j += 1;
            }
            q >>= 1;
        }
        p >>= 1;
    }
}

pub fn crypto_verify_1039(x: &[u8], y: &[u8]) -> i32 {
    let mut differentbits: u32 = 0;

    for i in 0..CIPHERTEXT_LENGTH {
        differentbits |= (x[i] ^ y[i]) as u32;
    }

    (1 & ((differentbits.wrapping_sub(1)) >> 8)) as i32 - 1
}