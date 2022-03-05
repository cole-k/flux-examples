#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

/// Ported from https://rosettacode.org/wiki/Fast_Fourier_transform#C

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;

// cexp_neg_i_pi(i, n) = cexp(-I * PI * i / n)
#[lr::sig(fn(i:usize, n:usize) -> f32)]
fn cexp_neg_i_pi(_i: usize, _n: usize) -> f32 {
  3.14
}

#[lr::sig(fn(src: &n@RVec<f32>) -> RVec<f32>[n])]
pub fn clone(src: &RVec<f32>) -> RVec<f32> {
  let n = src.len();
  let mut dst: RVec<f32> = RVec::new();
  let mut i = 0;
  while i < n {
    let val = *src.get(i);
    dst.push(val);
    i += 1;
  }
  dst
}


/*

INV: b <= s, s < n, 2*s <= n

n = 16
b = 0, s = 1
  ==> b = 0, s = 2
      ==> b = 0, s = 4
        ==> b = 0, s = 8
        ==> b = 4, s = 8
      ==> b = 2, s = 4
        ==> b = 2, s = 8
          ==> b = 2, s = 16
          ==> b = 10, s = 16
        ==> b = 6, s = 8
          ==> b = 6, s = 16
          ==> b = 14, s = 16

  ==> b = 1, s = 2
      ==> b = 1, s = 4
      ==> b = 3, s = 4
*/
fn _fft(buf: &mut RVec<f32>, out: &mut RVec<f32>, base: usize, step: usize) -> i32 {
  let n = buf.len();
	if /*  step < n && */ 2 * step <= n {
		_fft(out, buf, base, step * 2);
		_fft(out, buf, base + step, step * 2);

    let mut i = 0;

    while i < n {
      let t = cexp_neg_i_pi(i, n) * out.get(base + i + step);
			*buf.get_mut(base + (i / 2))       = out.get(base + i) + t;
			*buf.get_mut(base + ((i + n) / 2)) = out.get(base + i) - t;
      i += 2 * step;
    }
	}
  0
}

pub fn fft(buf:&mut RVec<f32>) -> i32 {
	let mut out = clone(buf);
	_fft(buf, &mut out, 0,1);
  0
}
