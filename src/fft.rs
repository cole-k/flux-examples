#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;


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

fn pi() -> f32 {
  3.14159265358979323846
}

fn two_pi() -> f32 {
  2.0 * pi()
}

fn float_of_int(_n:usize) -> f32 {
  1.0
}

fn cos(_x:f32) -> f32 {
  1.0
}

fn sin(_x:f32) -> f32 {
  1.0
}

pub fn fft(px: &mut RVec<f32>, py: &mut RVec<f32>) -> i32 {
  loop_a(px, py);
  loop_b(px, py);
  loop_c(px, py);
  0
}

fn loop_a(px: &mut RVec<f32>, py: &mut RVec<f32>) -> i32 {
  let n = px.len() - 1;
  let mut n2 = n;
  let mut n4 = n / 4;

  while 2 < n2 {
    let e = two_pi() / float_of_int(n2);
    let e3 = 3.0 * e;
    let mut a: f32 = 0.0;
    let mut a3: f32 = 0.0;
    let mut j = 1;
    while j <= n4 {
      let cc1 = cos(a);
      let ss1 = sin(a);
      let cc3 = cos(a3);
      let ss3 = sin(a3);
      a = a + e;
      a3 = a3 + e3;

      let mut is = j;
      let mut id = 2 * n2;
      while is < n {
        let mut i0 = is;
        let mut i1 = i0 + n4;
        let mut i2 = i1 + n4;
        let mut i3 = i2 + n4;
        while i3 <= n {
          let r1 = px.get(i0) - px.get(i2);
          *px.get_mut(i0) = px.get(i0) + px.get(i2);
          let r2 = px.get(i1) - px.get(i3);
          *px.get_mut(i1) = px.get(i1) + px.get(i3);
          let s1 = py.get(i0) - py.get(i2);
          *py.get_mut(i0) = py.get(i0) + py.get(i2);
          let s2 = py.get(i1) - py.get(i3);
          *py.get_mut(i1) = py.get(i1) + py.get(i3);

          let s3 = r1 - s2;
          let r1 = r1 + s2;
          let s2 = r2 - s1;
          let r2 = r2 + s1;
          *px.get_mut(i2) = r1 * cc1 - s2 * ss1;
          *py.get_mut(i2) = (0. - s2) * cc1 - r1 * ss1;
          *px.get_mut(i3) = s3 * cc3 + r2 * ss3;
          *py.get_mut(i3) = r2 * cc3 - s3 * ss3;
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

fn loop_b(px: &mut RVec<f32>, py: &mut RVec<f32>) -> i32 {
  let n = px.len() - 1;
  // loop_b2(px, py, 1, 4)
  let mut is = 1;
  let mut id = 4;
  while is < n {
    // loop_b1(px, py, is, is + 1, id);
    let mut i0 = is;
    let mut i1 = is + 1;
    while i1 <= n {
      let r1 = *px.get(i0);
      *px.get_mut(i0) = r1 + px.get(i1);
      *px.get_mut(i1) = r1 - px.get(i1);

      let r1 = *py.get(i0);
      *py.get_mut(i0) = r1 + py.get(i1);
      *py.get_mut(i1) = r1 - py.get(i1);

      i0 = i0 + id;
      i1 = i1 + id;
    }
    is = 2 * id - 1;
    id = 4 * id;
  }
  0
}

fn loop_c(px: &mut RVec<f32>, py: &mut RVec<f32>) -> i32 {
  let n = px.len() - 1;
  let mut i = 1;
  let mut j = 1;
  while i < n {
    if i < j {
      let xt = *px.get(j);
      *px.get_mut(j) = *px.get(i);
      *px.get_mut(i) = xt;

      let xt = *py.get(j);
      *py.get_mut(j) = *py.get(i);
      *py.get_mut(i) = xt;
    }
    i += 1;
    j = loop_c1(j, n/2);
  }
  0
}

fn loop_c1(j:usize, k: usize) -> usize {
  if k >= j {
    j + k
  } else {
    loop_c1(j-k, k/ 2)
  }
}

/*
fn loop_b1(px: &mut RVec<f32>, py: &mut RVec<f32>,
          i0: usize, i1: usize, id: usize
        ) -> i32 {
  let n = px.len() - 1;
  if i1 <= n {
    let r1 = *px.get(i0);
    *px.get_mut(i0) = r1 + px.get(i1);
    *px.get_mut(i1) = r1 - px.get(i1);

    let r1 = *py.get(i0);
    *py.get_mut(i0) = r1 + py.get(i1);
    *py.get_mut(i1) = r1 - py.get(i1);
    loop_b1(px, py, i0 + id, i1 + id, id);
  }
  0
}

fn loop_b2(px: &mut RVec<f32>, py: &mut RVec<f32>,
           is: usize, id: usize) -> i32 {
  let n = px.len() - 1;
  if is < n {
    // loop_b1(px, py, is, is + 1, id);
    let mut i0 = is;
    let mut i1 = is + 1;
    while i1 <= n {
      let r1 = *px.get(i0);
      *px.get_mut(i0) = r1 + px.get(i1);
      *px.get_mut(i1) = r1 - px.get(i1);

      let r1 = *py.get(i0);
      *py.get_mut(i0) = r1 + py.get(i1);
      *py.get_mut(i1) = r1 - py.get(i1);

      i0 = i0 + id;
      i1 = i1 + id;
    }
    loop_b2(px, py, 2 * id - 1, 4 * id);
  }
  0
}

fn loop_a2(px: &mut RVec<f32>, py: &mut RVec<f32>,
         n2: usize, n4: usize, j:usize,
         cc1: f32, ss1: f32, cc3: f32, ss3:f32,
         is: usize, id: usize,
        ) -> i32 {

  let n = px.len();
  if is < n {
    let i1 = is + n4;
    let i2 = i1 + n4;
    let i3 = i2 + n4;
    loop_a1(px, py, is, id, n4, cc1, ss1, cc3, ss3);
    loop_a2(px, py, n2, n4, j, cc1, ss1, cc3, ss3, 2 * id - n2 + j, 4 * id);
  }
  0
}

fn loop_a1(px: &mut RVec<f32>, py: &mut RVec<f32>,
         is: usize, id: usize, n4: usize,
         cc1: f32, ss1: f32, cc3: f32, ss3:f32
        ) -> i32 {

  let n = px.len();

  let mut i0 = is;
  let mut i1 = i0 + n4;
  let mut i2 = i1 + n4;
  let mut i3 = i2 + n4;

  while i3 <= n {

    let r1 = px.get(i0) - px.get(i2);
    *px.get_mut(i0) = px.get(i0) + px.get(i2);

    let r2 = px.get(i1) - px.get(i3);
    *px.get_mut(i1) = px.get(i1) + px.get(i3);

    let s1 = py.get(i0) - py.get(i2);
    *py.get_mut(i0) = py.get(i0) + py.get(i2);

    let s2 = py.get(i1) - py.get(i3);
    *py.get_mut(i1) = py.get(i1) + py.get(i3);

    let s3 = r1 - s2;
    let r1 = r1 + s2;
    let s2 = r2 - s1;
    let r2 = r2 + s1;

    *px.get_mut(i2) = r1 * cc1 - s2 * ss1;
    *py.get_mut(i2) = (0. - s2) * cc1 - r1 * ss1;
    *px.get_mut(i3) = s3 * cc3 + r2 * ss3;
    *py.get_mut(i3) = r2 * cc3 - s3 * ss3;

    i0 = i0 + id;
    i1 = i1 + id;
    i2 = i2 + id;
    i3 = i3 + id;
  }
  0
}
*/

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