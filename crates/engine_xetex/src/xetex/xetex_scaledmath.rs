#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use crate::xetex::xetex_core::scaled_t;
use std::ffi::c_char;

unsafe extern "C" {
    pub static mut arith_error: bool;
    pub static mut tex_remainder: scaled_t;
    pub static mut randoms: [i32; 55];
    pub static mut j_random: u8;
    pub static mut two_to_the: [i32; 31];
    pub static mut spec_log: [i32; 29];
    pub static mut help_ptr: u8;
    pub static mut help_line: [*const c_char; 6];

    pub fn error_here_with_diagnostic(s: *const c_char) -> *mut tectonic_bridge_core::Diagnostic;
    pub fn print_scaled(s: scaled_t);
    pub fn print_cstr(s: *const c_char);
    pub fn capture_to_diagnostic(diagnostic: *mut tectonic_bridge_core::Diagnostic);
    pub fn error();
}

#[inline]
fn odd(x: i32) -> bool {
    (x & 1) != 0
}

#[unsafe(export_name = "tex_round")]
pub unsafe extern "C" fn tex_round(r: f64) -> i32 {
    if r > i32::MAX as f64 {
        return i32::MAX;
    }
    if r < i32::MIN as f64 {
        return i32::MIN;
    }
    if r >= 0.0 {
        return (r + 0.5) as i32;
    }
    (r - 0.5) as i32
}

#[unsafe(export_name = "half")]
pub unsafe extern "C" fn half(x: i32) -> i32 {
    if odd(x) {
        return (x + 1) / 2;
    }
    x / 2
}

#[unsafe(export_name = "mult_and_add")]
pub unsafe extern "C" fn mult_and_add(mut n: i32, mut x: scaled_t, y: scaled_t, max_answer: scaled_t) -> scaled_t {
    unsafe {
        if n < 0 {
            x = -x;
            n = -n;
        }

        if n == 0 {
            y
        } else if x <= (max_answer - y) / n && (-x <= (max_answer + y) / n) {
            n * x + y
        } else {
            arith_error = true;
            0
        }
    }
}

#[unsafe(export_name = "x_over_n")]
pub unsafe extern "C" fn x_over_n(mut x: scaled_t, mut n: i32) -> scaled_t {
    unsafe {
        if n == 0 {
            arith_error = true;
            tex_remainder = x;
            0
        } else {
            if n < 0 {
                x = -x;
                n = -n;
                tex_remainder = -tex_remainder;
            }

            if x >= 0 {
                tex_remainder = x % n;
                x / n
            } else {
                tex_remainder = -((-x) % n);
                -((-x) / n)
            }
        }
    }
}

#[unsafe(export_name = "xn_over_d")]
pub unsafe extern "C" fn xn_over_d(mut x: scaled_t, n: i32, d: i32) -> scaled_t {
    unsafe {
        let positive = if x >= 0 {
            true
        } else {
            x = -x;
            false
        };

        let t = (x % 32768) * n;
        let mut u = (x / 32768) * n + (t / 32768);
        let v = (u % d) * 32768 + (t % 32768);

        if u / d >= 32768 {
            arith_error = true;
        } else {
            u = 32768 * (u / d) + (v / d);
        }

        if positive {
            tex_remainder = v % d;
            u
        } else {
            tex_remainder = -(v % d);
            -u
        }
    }
}

#[unsafe(export_name = "round_xn_over_d")]
pub unsafe extern "C" fn round_xn_over_d(mut x: scaled_t, n: i32, d: i32) -> scaled_t {
    unsafe {
        let positive = if x >= 0 {
            true
        } else {
            x = -x;
            false
        };

        let t = (x % 32768) * n;
        let mut u = (x / 32768) * n + (t / 32768);
        let mut v = (u % d) * 32768 + (t % 32768);
        if u / d >= 32768 {
            arith_error = true;
        } else {
            u = 32768 * (u / d) + (v / d);
        }
        v %= d;
        if 2 * v >= d {
            u += 1;
        }
        if positive { u } else { -u }
    }
}

unsafe fn make_frac(mut p: i32, mut q: i32) -> i32 {
    unsafe {
        let mut negative = if p >= 0 {
            false
        } else {
            p = -p;
            true
        };

        if q <= 0 {
            q = -q;
            negative = !negative;
        }

        let mut n = p / q;
        p %= q;

        if n >= 8 {
            arith_error = true;
            if negative { -0x7FFFFFFF } else { 0x7FFFFFFF }
        } else {
            n = (n - 1) * 0x10000000;
            let mut f: i32 = 1;

            loop {
                let be_careful = p - q;
                p = be_careful + p;
                if p >= 0 {
                    f = f + f + 1;
                } else {
                    f = f + f;
                    p += q;
                }
                if f >= 0x10000000 {
                    break;
                }
            }

            let be_careful = p - q;
            if be_careful + p >= 0 {
                f += 1;
            }

            if negative { -(f + n) } else { f + n }
        }
    }
}

unsafe fn take_frac(mut q: i32, mut f: i32) -> i32 {
    unsafe {
        let mut negative = if f >= 0 {
            false
        } else {
            f = -f;
            true
        };

        if q < 0 {
            q = -q;
            negative = !negative;
        }

        let mut n: i32;
        if f < 0x10000000 {
            n = 0;
        } else {
            n = f / 0x10000000;
            f %= 0x10000000;

            if q <= 0x7FFFFFFF / n {
                n *= q;
            } else {
                arith_error = true;
                n = 0x7FFFFFFF;
            }
        }

        f += 0x10000000;
        let mut p: i32 = 0x08000000;

        if q < 0x40000000 {
            loop {
                if odd(f) {
                    p = (p + q) / 2;
                } else {
                    p /= 2;
                }
                f /= 2;
                if f == 1 {
                    break;
                }
            }
        } else {
            loop {
                if odd(f) {
                    p = p + (q - p) / 2;
                } else {
                    p /= 2;
                }
                f /= 2;
                if f == 1 {
                    break;
                }
            }
        }

        let be_careful = n - 0x7FFFFFFF;
        if be_careful + p > 0 {
            arith_error = true;
            n = 0x7FFFFFFF - p;
        }

        if negative { -(n + p) } else { n + p }
    }
}

unsafe fn m_log(mut x: i32) -> i32 {
    unsafe {
        if x <= 0 {
            error_here_with_diagnostic(c"Logarithm of ".as_ptr());
            print_scaled(x);
            print_cstr(c" has been replaced by 0".as_ptr());
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            help_line[1] = c"Since I don't take logs of non-positive numbers,".as_ptr();
            help_line[0] = c"I'm zeroing this one. Proceed, with fingers crossed.".as_ptr();
            error();
            0
        } else {
            let mut y: i32 = 1302456860;
            let mut z: i32 = 6581195;

            while x < 0x40000000 {
                x += x;
                y -= 93032639;
                z -= 48782;
            }

            y += z / 65536;
            let mut k: usize = 2;

            while x > 0x40000004 {
                let mut z = ((x - 1) / two_to_the[k]) + 1;

                while x < 0x40000000 + z {
                    z = (z + 1) / 2;
                    k += 1;
                }

                y += spec_log[k];
                x -= z;
            }

            y / 8
        }
    }
}

unsafe fn ab_vs_cd(mut a: i32, mut b: i32, mut c: i32, mut d: i32) -> i32 {
    if a < 0 {
        a = -a;
        b = -b;
    }

    if c < 0 {
        c = -c;
        d = -d;
    }

    if d <= 0 {
        if b >= 0 {
            if (a == 0 || b == 0) && (c == 0 || d == 0) {
                return 0;
            } else {
                return 1;
            }
        }

        if d == 0 {
            if a == 0 {
                return 0;
            } else {
                return -1;
            }
        }

        let q = a;
        a = c;
        c = q;
        let q = -b;
        b = -d;
        d = q;
    } else if b <= 0 {
        if b < 0 {
            if a > 0 {
                return -1;
            }
        }

        if c == 0 {
            return 0;
        } else {
            return -1;
        }
    }

    loop {
        let mut q = a / d;
        let mut r = c / b;

        if q != r {
            if q > r {
                return 1;
            } else {
                return -1;
            }
        }

        q = a % d;
        r = c % b;

        if r == 0 {
            if q == 0 {
                return 0;
            } else {
                return 1;
            }
        }

        if q == 0 {
            return -1;
        }

        a = b;
        b = q;
        c = d;
        d = r;
    }
}

unsafe fn new_randoms() {
    unsafe {
        for k in 0..24 {
            let mut x = randoms[k] - randoms[k + 31];
            if x < 0 {
                x += 0x10000000;
            }
            randoms[k] = x;
        }

        for k in 24..55 {
            let mut x = randoms[k] - randoms[k - 24];
            if x < 0 {
                x += 0x10000000;
            }
            randoms[k] = x;
        }

        j_random = 54;
    }
}

#[unsafe(export_name = "init_randoms")]
pub unsafe extern "C" fn init_randoms(seed: i32) {
    unsafe {
        let mut j = seed.abs();

        while j >= 0x10000000 {
            j /= 2;
        }

        let mut k: i32 = 1;

        for i in 0..55usize {
            let jj = k;
            k = j - k;
            j = jj;
            if k < 0 {
                k += 0x10000000;
            }
            randoms[(i * 21) % 55] = j;
        }

        new_randoms();
        new_randoms();
        new_randoms();
    }
}

#[unsafe(export_name = "unif_rand")]
pub unsafe extern "C" fn unif_rand(x: i32) -> i32 {
    unsafe {
        if j_random == 0 {
            new_randoms();
        } else {
            j_random -= 1;
        }

        let y = take_frac(x.abs(), randoms[j_random as usize]);
        if y == x.abs() {
            0
        } else if x > 0 {
            y
        } else {
            -y
        }
    }
}

#[unsafe(export_name = "norm_rand")]
pub unsafe extern "C" fn norm_rand() -> i32 {
    unsafe {
        loop {
            let (x, u) = loop {
                if j_random == 0 {
                    new_randoms();
                } else {
                    j_random -= 1;
                }

                let cand_x = take_frac(112429, randoms[j_random as usize] - 0x08000000);

                if j_random == 0 {
                    new_randoms();
                } else {
                    j_random -= 1;
                }

                let cand_u = randoms[j_random as usize];
                if cand_x.abs() < cand_u {
                    break (cand_x, cand_u);
                }
            };

            let x = make_frac(x, u);
            let l = 139548960 - m_log(u);
            if ab_vs_cd(1024, l, x, x) >= 0 {
                return x;
            }
        }
    }
}
