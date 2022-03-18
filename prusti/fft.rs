extern crate prusti_contracts;
use prusti_contracts::*;

#[derive(PartialEq, Copy, Clone)]
pub struct F32Wrapper {
  f: f32,
}

impl F32Wrapper {
  //pub const ZERO: F32Wrapper = F32Wrapper { f: 0.0 };

  #[trusted]
  pub fn new(val: f32) -> F32Wrapper {
    return Self { f: val }
  }

  #[trusted]
  pub fn float_of_int(val: usize) -> F32Wrapper {
    return Self { f: val as f32 }
  }

  #[trusted]
  pub fn zero() -> F32Wrapper {
    return Self { f: 0.0 }
  }

  #[trusted]
  pub fn one_half() -> F32Wrapper {
    return Self { f: 0.5 }
  }

  #[trusted]
  pub fn pi() -> F32Wrapper {
    return Self { f: 3.14159265358979323846 }
  }

  #[trusted]
  pub fn two_pi() -> F32Wrapper {
    return Self { f: 2.0 * 3.14159265358979323846 }
  }
  
  #[trusted]
  pub fn add(&self, other: F32Wrapper) -> F32Wrapper {
    F32Wrapper { f: self.f + other.f }
  }

  #[trusted]
  pub fn sub(&self, other: F32Wrapper) -> F32Wrapper {
    F32Wrapper { f: self.f - other.f }
  }

  #[trusted]
  pub fn mul(&self, other: F32Wrapper) -> F32Wrapper {
    F32Wrapper { f: self.f * other.f }
  }

  #[trusted]
  pub fn div(&self, other: F32Wrapper) -> F32Wrapper {
    F32Wrapper { f: self.f / other.f }
  }

  #[trusted]
  pub fn lt(&self, other: F32Wrapper) -> bool {
    self.f < other.f
  }

  #[trusted]
  pub fn sin(&self) -> F32Wrapper {
    F32Wrapper { f: f32::sin(self.f) }
  }

  #[trusted]
  pub fn cos(&self) -> F32Wrapper {
    F32Wrapper { f: f32::cos(self.f) }
  }

  #[trusted]
  pub fn abs(&self) -> F32Wrapper {
    F32Wrapper { f: f32::abs(self.f) }
  }
}
/*
#[extern_spec]
impl PartialEq<F32Wrapper> {
    #[pure]
    #[trusted]
    fn eq(&self, other: &F32Wrapper) -> bool;
}*/

pub struct VecWrapperF32 {
  v: Vec<F32Wrapper>,
}

/// Need this because Prusti doesn't seem to be able to handle polymorphic types
impl VecWrapperF32 {
  #[trusted]
  #[ensures(result.len() == 0)]
  pub fn new() -> Self {
      Self { v: Vec::new() }
  }

  #[trusted]
  #[ensures(result.len() == n)]
  pub fn from_elem_n(value: F32Wrapper, n: usize) -> Self {
    let mut v = Vec::new();
    for _ in 0..n {
      v.push(value);
    }

    Self { v }
  }

  /// A ghost function for getting the length of the vector
  #[trusted]
  #[pure]
  pub fn len(&self) -> usize {
      self.v.len()
  }

  /// A ghost function for specifying values stored in the vector.
  #[trusted]
  #[pure]
  #[requires(index < self.len())]
  pub fn lookup(&self, index: usize) -> F32Wrapper {
      self.v[index]
  }

  /// A ghost function for specifying values stored in the vector.
  #[trusted]
  #[requires(index < self.len())]
  #[ensures(self.len() == old(self.len()))]
  pub fn store(&mut self, index: usize, value: F32Wrapper) {
      self.v[index] = value;
  }

  #[trusted]
  #[ensures(self.len() == old(self.len()) + 1)]
  pub fn push(&mut self, value: F32Wrapper) {
      self.v.push(value);
  }
}

//#[lr::sig(fn(src: &n@RVec<f32>{0 <= n}) -> RVec<f32>[n])]
/*#[ensures(result.len() == src.len())]
pub fn clone(src: &VecWrapperF32) -> VecWrapperF32 {
  let n = src.len();
  let mut dst: VecWrapperF32 = VecWrapperF32::new();
  let mut i = 0;
  while i < n {
    body_invariant!(dst.len() == i);
    body_invariant!(i < src.len());
    let val = src.lookup(i);
    dst.push(val);
    i += 1;
  }
  dst
}*/

//#[lr::sig(fn(px: &mut n@VecWrapperF32, py: &mut VecWrapperF32{v:v == n}) -> i32 where 2 <= n)]
#[requires(px.len() >= 2)]
#[requires(px.len() == py.len())]
#[ensures(px.len() == old(px.len()))]
#[ensures(py.len() == old(py.len()))]
pub fn fft(px: &mut VecWrapperF32, py: &mut VecWrapperF32) {
  loop_a(px, py);
  loop_b(px, py);
  loop_c(px, py);
}

// #[lr::sig(fn(px: &mut n@RVec<f32>, py: &mut RVec<f32>[n]) -> i32)]
#[requires(px.len() == py.len())]
#[ensures(px.len() == old(px.len()))]
#[ensures(py.len() == old(py.len()))]
fn loop_a(px: &mut VecWrapperF32, py: &mut VecWrapperF32) -> i32 {
  let n = px.len() - 1;
  let px_len = px.len();
  let py_len = py.len();
  let mut n2 = n;
  let mut n4: usize = n / 4;
  // This is necessary to keep prusti happy
  let three = F32Wrapper::float_of_int(3);

  while 2 < n2 {
    body_invariant!(n < px.len() && n < py.len());
    body_invariant!(px.len() == px_len);
    body_invariant!(py.len() == py_len);
    let e = F32Wrapper::two_pi().div(F32Wrapper::float_of_int(n2));
    let e3 = e.mul(three);
    let mut a = F32Wrapper::zero();
    let mut a3 = F32Wrapper::zero();
    let mut j = 1;
    while j <= n4 {
      body_invariant!(n < px.len() && n < py.len());
      body_invariant!(py.len() == py_len);
      body_invariant!(px.len() == px_len);

      assert!(px.len() == px_len);
      let cc1 = a.cos();
      let ss1 = a.sin();
      let cc3 = a3.cos();
      let ss3 = a3.sin();
      a = a.add(e);
      a3 = a3.add(e3);

      let mut is = j;
      let mut id = 2 * n2;

      while is < n {
        body_invariant!(n < px.len() && n < py.len());
        body_invariant!(is < px.len());
        body_invariant!(py.len() == py_len);
        body_invariant!(px.len() == px_len);

        // INV 0 <= is, 0 <= n2 <= id
        let mut i0 = is;
        let mut i1 = i0 + n4;
        let mut i2 = i1 + n4;
        let mut i3 = i2 + n4;

        while i3 <= n {
          // INV 0 <= i0 <= i1 <= i2 <= i3, 0 <= id
          body_invariant!(n < px.len() && n < py.len());
          body_invariant!(py.len() == py_len);
          body_invariant!(px.len() == px_len);
          body_invariant!(i0 <= i1 && i1 <= i2 && i2 <= i3);
          body_invariant!(i3 < px.len());

          let r1 = px.lookup(i0).sub(px.lookup(i2));
          let tmp = px.lookup(i0).add(px.lookup(i2));
          px.store(i0, tmp);

          let r2 = px.lookup(i1).sub(px.lookup(i3));
          let tmp = px.lookup(i1).add(px.lookup(i3));
          px.store(i1, tmp);
          let s1 = py.lookup(i0).sub(py.lookup(i2));
          let tmp = px.lookup(i0).add(px.lookup(i2));
          py.store(i0, tmp);
          let s2 = py.lookup(i1).sub(py.lookup(i3));
          let tmp = px.lookup(i1).add(px.lookup(i3));
          py.store(i1, tmp);

          let s3 = r1.sub(s2);
          let r1 = r1.add(s2);
          let s2 = r2.sub(s1);
          let r2 = r2.add(s1);
          px.store(i2, r1.mul(cc1).sub(s2.mul(ss1)));
          py.store(i2, (F32Wrapper::zero().sub(s2)).mul(cc1).sub(r1.mul(ss1)));
          px.store(i3, s3.mul(cc3).add(r2.mul(ss3)));
          py.store(i3, r2.mul(cc3).sub(s3.mul(ss3)));

          i0 = i0 + id;
          i1 = i1 + id;
          i2 = i2 + id;
          i3 = i3 + id;
        }
        // end loop1

        is = 2 * id - n2 + j;
        id = 4 * id;
      }
      // end loop2
    j += 1
    }
    n2 = n2/2;
    n4 = n4/2;
  }
  0
}

//#[lr::sig(fn (px: &mut n@RVec<f32>, py: &mut RVec<f32>[n]) -> i32)]
#[requires(px.len() == py.len())]
#[ensures(px.len() == old(px.len()))]
#[ensures(py.len() == old(py.len()))]
fn loop_b(px: &mut VecWrapperF32, py: &mut VecWrapperF32) -> i32 {
  let n = px.len() - 1;
  let px_len = px.len();
  let py_len = py.len();

  let mut is = 1;
  let mut id = 4;

  while is < n {
    body_invariant!(n < px.len() && n < py.len());
    body_invariant!(py.len() == py_len);
    body_invariant!(px.len() == px_len);

    // INV: 0 <= is, 4 <= id
    let mut i0 = is;
    let mut i1 = is + 1;
    while i1 <= n {
      body_invariant!(n < px.len() && n < py.len());
      body_invariant!(i0 <= i1);
      body_invariant!(py.len() == py_len);
      body_invariant!(px.len() == px_len);

      // INV: 0 <= i0 <= i1, 0 <= id
      let r1 = px.lookup(i0);
      let tmp = r1.add(px.lookup(i1));
      px.store(i0, tmp);
      let tmp = r1.sub(px.lookup(i1));
      px.store(i1, tmp);

      let r1 = py.lookup(i0);
      let tmp = r1.add(px.lookup(i1));
      py.store(i0, tmp);
      let tmp = r1.sub(px.lookup(i1));
      py.store(i1, tmp);

      i0 = i0 + id;
      i1 = i1 + id;
    }
    is = 2 * id - 1;
    id = 4 * id;
  }
  0
}


//#[lr::sig(fn (px: &mut n@RVec<f32>, py: &mut RVec<f32>[n]) -> i32 where 2 <= n)]
#[requires(px.len() >= 2)]
#[requires(px.len() == py.len())]
#[ensures(px.len() == old(px.len()))]
#[ensures(py.len() == old(py.len()))]
fn loop_c(px: &mut VecWrapperF32, py: &mut VecWrapperF32) -> i32 {
  let n = px.len() - 1;
  let mut i = 1;
  let mut j = 1;
  let px_len = px.len();
  let py_len = py.len();

  while i < n {
    body_invariant!(n < px.len() && n < py.len());
    body_invariant!(px.len() == px_len);
    body_invariant!(py.len() == py_len);
    // INV: 0 <= i, 0 <= j <= n
    if i < j {
      body_invariant!(j <= n);
      body_invariant!(n < px.len() && n < py.len());
      body_invariant!(px.len() == px_len);
      body_invariant!(py.len() == py_len);

      let xt = px.lookup(j);
      let tmp = px.lookup(i);
      px.store(j, tmp);
      px.store(i, xt);

      let xt = px.lookup(j);
      let tmp = px.lookup(i);
      px.store(j, tmp);
      px.store(i, xt);
    }
    i += 1;
    //j = loop_c1(j, n/2);
    // let mut k = n / 2;
    // while k < j {
    //
    //   INV:   0 <= k + k <= n
    //   QUAL:  2k <= n
    //
    //   j = j - k;
    //   k = k / 2;
    // }
    // j = j + k;
  }
  0
}

#[trusted]
#[ensures(result <= k)]
pub fn div_by_2(k: usize) -> usize {
  k / 2
}

//#[lr::sig(fn (j:usize{0<=j}, k: usize{0<=k}) -> usize{v:0<=v && v<=k+k})]
#[ensures(result <= k+k)]
pub fn loop_c1(j:usize, k: usize) -> usize {
  if j <= k {
    j + k
  } else {
    loop_c1(j-k, div_by_2(k))
  }
}

//#[lr::sig(fn (np:usize) -> f32 where 2 <= np)]
#[requires(2 <= np)]
pub fn fft_test(np:usize) -> F32Wrapper {
  let enp = F32Wrapper::float_of_int(np);
  let n2 = np / 2;
  let npm = n2 - 1;
  let mut pxr = VecWrapperF32::from_elem_n(F32Wrapper::zero(), np+1);
  let mut pxi = VecWrapperF32::from_elem_n(F32Wrapper::zero(), np+1);
  assert!(pxi.len() >= npm + 2);
  let t = F32Wrapper::pi().div(enp);
  pxr.store(1, enp.sub(F32Wrapper::float_of_int(1)).mul(F32Wrapper::one_half()));
  pxi.store(1, F32Wrapper::zero());
  pxr.store(n2+1, F32Wrapper::zero().sub(F32Wrapper::one_half()));
  pxi.store(n2+1, F32Wrapper::zero());
  let mut i = 1;
  while i <= npm {
    body_invariant!(pxr.len() == np + 1);
    body_invariant!(pxi.len() == np + 1);
    body_invariant!(i + 1 < np + 1);
    let j = np - i;
    pxr.store(i+1, F32Wrapper::zero().sub(F32Wrapper::one_half()));
    pxr.store(j+1, F32Wrapper::zero().sub(F32Wrapper::one_half()));
    let z = t.mul(F32Wrapper::float_of_int(i));
    let y = F32Wrapper::one_half().mul(z.cos().div(z.sin()));
    pxi.store(i+1, F32Wrapper::zero().sub(y));
    pxi.store(j+1, y);
    i += 1;
  }

  fft(&mut pxr, &mut pxi);

  let mut zr = F32Wrapper::zero();
  let mut zi = F32Wrapper::zero();
  let mut _kr = 0;
  let mut _ki = 0;
  let mut i = 0;
  while i < np {
    body_invariant!(pxr.len() == np + 1);
    body_invariant!(pxi.len() == np + 1);
    body_invariant!(i + 1 < np + 1);
    let a = pxr.lookup(i+1).sub(F32Wrapper::float_of_int(i)).abs();
    if zr.lt(a) {
      zr = a;
      _kr = i;
    }
    let a = pxi.lookup(i+1).abs();
    if zi.lt(a) {
      zi = a;
      _ki = i;
    }
    i += 1;
  }
  if zr.abs().lt(zi.abs()) { zi } else { zr }
}

//#[lr::sig(fn() -> i32)]
pub fn doit() {
  let mut i = 4;
  let mut np = 16;
  while i <= 16 {
    body_invariant!(2 <= np);
    fft_test(np);
    i  = i + 1;
    np = np * 2;
  }
}

pub fn main() {}

/* ORIGINAL DML Code below

(*
** by: Dave Edelblute, edelblut@cod.nosc.mil, 05 Jan 1993
** Modified: R. Mayer to work with hist benchmark routines.
** Translated from C to de Caml: Hongwei Xi, 07 Nov 1998
*)
let{n:int | n >= 2} fft px py n = (* n must be a power of 2! *)
  let rec{n2:nat} loop n2 n4 =
    if le_int n2 2 then () else (* the case n2 = 2 is treated below *)
    let e = two_pi /. (float_of_int n2) in let e3 = 3.0 *. e in
    let a = ref 0.0 and a3 = ref 0.0 in
    for j = 1 to n4 do
      let cc1 = cos !a and ss1 = sin !a and cc3 = cos !a3 and ss3 = sin !a3 in
      let none_ = a := !a +. e and none_ = a3 := !a3 +. e3 in
      let rec loop1 i0 i1 i2 i3 id =
        if gt_int i3 n then () else (* out_of_bounds *)
        let r1 = px..(i0) -. px..(i2)
        and none_ = Array.set px i0 (px..(i0) +. px..(i2))
        and r2 = px..(i1) -. px..(i3)
        and none_ = Array.set px i1 (px..(i1) +. px..(i3))
        and s1 = py..(i0) -. py..(i2)
        and none_ = Array.set py i0 (py..(i0) +. py..(i2))
        and s2 = py..(i1) -. py..(i3)
        and none_ = Array.set py i1 (py..(i1) +. py..(i3) in)
        let s3 = r1 -. s2 and r1 = r1 +. s2
        and s2 = r2 -. s1 and r2 = r2 +. s1 in
        let none_ = px..(i2) <- r1 *. cc1 -. s2 *. ss1
        and none_ = py..(i2) <- (-. s2) *. cc1 -. r1 *. ss1
        and none_ = px..(i3) <- s3 *. cc3 +. r2 *. ss3
        and none_ = py..(i3) <- r2 *. cc3 -. s3 *. ss3 in
        loop1 (i0 + id) (i1 + id) (i2 + id) (i3 + id) id
      withtype {i0:nat}{i1:int}{i2:int}{i3:int | i0 <= i1 <= i2 <= i3}{id:nat}
               int(i0) -> int(i1) -> int(i2) -> int(i3) -> int(id) -> unit in
      let rec loop2 is id =
        if is >= n then () else begin
          let i1 = is + n4 in
          let i2 = i1 + n4 in
          let i3 = i2 + n4 in
          loop1 is i1 i2 i3 id;
          loop2 (2 * id - n2 + j) (4 * id)
        end
      withtype {is:nat}{id:nat | id >= n2} int(is) -> int(id) -> unit in
      loop2 j (2 * n2)
    done;
    loop (n2 / 2) (n4 / 2)
    withtype int(n2) -> int -> unit in
    loop n (n / 4);


    // HEREHEREHERE [ONTO LoopB]
    let rec loop1 i0 i1 id =
      if gt_int i1 n then () else
      let r1 = px..(i0) in
      let none_ = px..(i0) <- r1 +. px..(i1)
      and none_ = px..(i1) <- r1 -. px..(i1) in
      let r1 = py..(i0) in
      let none_ = py..(i0) <- r1 +. py..(i1)
      and none_ = py..(i1) <- r1 -. py..(i1) in
      loop1 (i0 + id) (i1 + id) id
    withtype {i0:nat}{i1:int | i0 <= i1} int(i0) -> int(i1) -> {id:nat} int(id) -> unit in
    let rec loop2 is id =
      if is >= n then () else begin
        loop1 is (is + 1) id;
        loop2 (2 * id - 1) (4 * id)
      end
    withtype {is:nat}{id:nat | id >= 4} int(is) -> int(id) -> unit in
    loop2 1 4;

    // loop_c1
    let rec loop1 j k =
      if ge_int k j then j + k else loop1 (j - k) (k / 2)
    withtype
      loop_c1 {j:nat}{k:nat | k <= n / 2} int(j) -> int(k) -> [i:nat | i <= n] int(i) in
    let rec loop2 i j =
      if i >= n then () else begin
        if ge_int i j then () else begin
          let xt = px..(j) in px..(j) <- px..(i); Array.get px i <- xt;
          let xt = Array.get py j in Array.get py j <- Array.get py i; Array.get py i <- xt;
        end;
        loop2 (i + 1) (loop1 j (n / 2))
      end
    withtype {i:nat} int(i) -> {j:nat | j <= n} int(j) -> unit in
    loop2 1 1; n
withtype float vect(n+1) -> float vect(n+1) -> int(n) -> int(n)
;;
let fabs r = if r >. 0.0 then r else (-. r)
;;
let ffttest np =
  let none_ = print_int np and none_ = print_string "... " in
  (* A *)
  let enp = float_of_int np and n2 = np / 2 in
  let npm = n2 - 1
  and pxr = make_vect (np+1) 0.0
  and pxi = make_vect (np+1) 0.0
  and t = pi /. enp in
  let none_ = Array.get pxr 1 <- (enp -. 1.0) *. 0.5
  and none_ = Array.get pxi 1 <- 0.0
  and none_ = pxr..(n2+1) <- (-. 0.5)
  and none_ = pxi..(n2+1) <- 0.0 in
  for i = 1 to npm do
    let j = np - i in
    let none_ = pxr..(i+1) <- (-. 0.5) and none_ = pxr..(j+1) <- (-. 0.5) in
    let z = t *. (float_of_int i) in
    let y = 0.5 *. cos(z) /. sin(z) in
    pxi..(i+1) <-  (-. y); pxi..(j+1) <- y
  done;
  fft pxr pxi np;

  (* B *)
  let rec loop i zr zi kr ki =
    if ge_int i np then (zr, zi) else
    let a = fabs(pxr..(i+1) -. (float_of_int i)) in
    let (zr, kr) = if zr <. a then (a, i) else (zr, kr) in
    let a = fabs(pxi..(i+1)) in
    let (zi, ki) = if zi <. a then (a, i) else (zi, ki) in
    loop (i+1) zr zi kr ki
  withtype {i:nat} int(i)  -> float -> float -> int -> int -> float * float in
  let (zr, zi) = loop 0 0.0 0.0 0 0 in
  let zm = if fabs zr <. fabs zi then zi else zr
  in print_float zm; print_newline ()
withtype {np:int | np >= 2} int(np) -> unit
;;
let rec loop_np i np =
  if i > 16 then () else begin ffttest np; loop_np (i + 1) (np * 2) end
withtype int -> {np:int | np >= 2} int(np) -> unit
;;
let doit () = loop_np 4 16;;
*)

*/