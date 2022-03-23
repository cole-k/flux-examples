#![allow(unused_attributes)]
extern crate prusti_contracts;
use prusti_contracts::*;

#[path = "lib/matwrapper.rs"]
pub mod matwrapper;
use matwrapper::{MatWrapper, vecwrapper::VecWrapper};

// #[lr::sig(fn(x:i32{0 <= x}) -> i32{v: 0 < v})]
#[requires(0 <= x)]
#[ensures(0 < result)]
pub fn incr(x:i32) -> i32 {
  x + 1
}

// #[lr::sig(fn() -> i32)]
/*
pub fn test() -> i32 {
  let pi: f32 = 3.14;
  let mut m  = MatWrapper<i32>::new(5, 10, pi);

  let v1 = m.get(3, 7);
  *m.get_mut(4, 8) = v1 + v1;

  // let v2 = *m.get(3, 17);        //~ ERROR precondition might not hold
  // *m.get_mut(14, 8) = v2 + v2;   //~ ERROR precondition might not hold

  0
}*/

/* step 1 */

//#[lr::sig(fn (arr2: &RMat<f32>[m,n], m:usize{0 < m}, n: usize{ 0 < n}) -> bool)]
#[requires(0 < _m && 0 < n)]
#[requires(_m == arr2.rows() && n == arr2.cols())]
pub fn is_neg(arr2: &MatWrapper<i32>, _m:usize, n: usize) -> bool {
  let mut j = 1;
  while j < n - 1 {
    if arr2.get(0, j) < 0 {
      return true
    }
    j += 1;
  }
  false
}

/* step 2 */

//#[lr::sig(fn (m:usize{0 < m}, n:usize{0 < n}, arr2: &RMat<f32>[m, n]) -> bool)]
#[requires(0 < m && 0 < n)]
#[requires(m == arr2.rows() && n == arr2.cols())]
pub fn unb1(m:usize, n:usize, arr2: &MatWrapper<i32>) -> bool {
  let mut i = 0;
  let mut j = 1;

  // INV: 0 < i <= m, 0 <= j < n
  while j < n - 1 {
    if arr2.get(0, j) < 0 {
      i = i + 1;
      loop {
        if i < m {
          if arr2.get(i, j) < 0 {
            i = i + 1
          } else {
            i = 0;
            j = j + 1;
            break;
          }
        } else {
          return true
        }
      }
    } else {
      i = 0;
      j = j + 1;
    }
  }
  false
}


/* step 3 */

//#[lr::sig(fn (m:usize{0<m}, n:usize{2<n}, arr2: &RMat<f32>[m,n]) -> usize{v: 0<v && v+1<n})]
#[requires(0 < _m && 2 < n)]
#[requires(_m == arr2.rows() && n == arr2.cols())]
#[ensures(0 < result && result + 1 < n)]
pub fn enter_var(_m:usize, n:usize, arr2: &MatWrapper<i32>) -> usize {
  let mut c  = arr2.get(0, 1);
  let mut j  = 1;
  let mut j_ = 2;
  while j_ < n - 1 {
    body_invariant!(0 < j && j + 1 < n);
    // INV j+1 < n, j_ < n
    let c_ = arr2.get(0, j_);
	  if c_ < c {
      j = j_;
      c = c_;
    }
    j_ += 1
  }
  j
}

/* step 4 */

//#[lr::sig(fn(m:usize, n:usize, arr2: &RMat<f32>[m, n], j:usize{0 < j && j < n}, i0:usize{0 < i0 && i0 < m}, r0:f32) -> usize{v:0 < v && v < m})]
#[requires(0 < m && 0 < n
  && m == arr2.rows() && n == arr2.cols()
  && 0 < j && j < n
  && 0 < i0 && i0 < m)]
#[ensures(0 < result && result < m)]
pub fn depart_var(m:usize, n:usize, arr2: &MatWrapper<i32>, j:usize, i0:usize, r0:i32) -> usize {
  let mut i  = i0;
  let mut r  = r0;
  let mut i_ = i + 1;
  while i_ < m {
    body_invariant!(0 < i && i < m);

    let c_ = arr2.get(i_, j);
    if 0 < c_ {
        let r_ = arr2.get(i_, n-1) / c_;
        if r_ < r {
          i = i_;
          r = r_;
        }
        i_ += 1;
    } else {
      i_ += 1
    }
  }
  i
}

//#[warn(unconditional_recursion)]
//#[lr::assume]
//#[lr::sig(fn() -> usize{v:false})]
#[trusted]
#[ensures(false)]
pub fn die() -> usize {
  panic!("die")
}

//cd #[lr::sig(fn (m:usize{0 < m}, n:usize{0 < n}, arr2: &RMat<f32>[m, n], j: usize{0 < j && j < n}) -> usize{v:0 < v && v < m})]
#[requires(0 < m && 0 < _n
  && m == arr2.rows() && _n == arr2.cols()
  && 0 < j && j < _n)]
#[ensures(0 < result && result < m)]
pub fn init_ratio_i(m:usize, _n:usize, arr2: &MatWrapper<i32>, j: usize) -> usize {
  let mut i = 1;
  while i < m {
    //body_invariant!(m == arr2.rows() && _n == arr2.cols());
    body_invariant!(0 < i && i < m && j < _n);
    let c = arr2.get(i, j);
    if 0 < c {
      return i
    }
    i += 1;
  }

  die() // abort ("init_ratio: negative coefficients!")
}
/*
(*let rec init_ratio_left arr2 m n j i =
  if i < m then
    let c = Bigarray.Array2.get arr2 i j in
      if c > 0.0 then i
      else init_ratio_left arr2 m n j (i+1)
  else assert false
in

let rec init_ratio_right arr2 m n j i =
  if i < m then
    let c = Bigarray.Array2.get arr2 i j in
      if c > 0.0 then (Bigarray.Array2.get arr2 i (n-1)) /. c
      else init_ratio_right arr2 m n j (i+1)
  else assert false
in*)

let rec init_ratio arr2 m n j i =
  if i < m then
    let c = Bigarray.Array2.get arr2 i j in
      if c > 0.0 then (i, (Bigarray.Array2.get arr2 i (n-1)) /. c)
      else init_ratio arr2 m n j (i+1)
  else assert false
in

(* step 5 *)

let rec norm_aux arr2 n i c j =
  if j < n then
    let _none = Bigarray.Array2.set arr2 i j ((Bigarray.Array2.get arr2 i j) /. c) in
      norm_aux arr2 n i c (j+1)
  else ()
in

let norm arr2 n i j =
  let c = Bigarray.Array2.get arr2 i j in
    norm_aux arr2 n i c 1
in

let rec row_op_aux1 arr2 n i i' c j =
  if j < n then
    let cj =  Bigarray.Array2.get arr2 i j in
    let cj' =  Bigarray.Array2.get arr2 i' j in
    let _none = Bigarray.Array2.set arr2 i' j (cj' -. cj *. c) in
      row_op_aux1 arr2 n i i' c (j+1)
  else ()
in

let row_op_aux2 arr2 n i i' j =
  let c' = Bigarray.Array2.get arr2 i' j in
    row_op_aux1 arr2 n i i' c' 1
in

let rec row_op_aux3 arr2 m n i j i' =
  if i' < m then
    if i' <> i then
      let _none = row_op_aux2 arr2 n i i' j in
	row_op_aux3 arr2 m n i j (i'+1)
    else row_op_aux3 arr2 m n i j (i'+1)
  else ()
in

let row_op arr2 m n i j =
    let _none = norm arr2 n i j in
      row_op_aux3 arr2 m n i j 0
in

let rec simplex arr2 m n =
  if is_neg arr2 n then
    if unb1 arr2 m n 0 1 then assert false
    else
      let j = enter_var arr2 n 1 (Bigarray.Array2.get arr2 0 1) 2 in
      (*let i = init_ratio_left arr2 m n j 1 in
      let r = init_ratio_right arr2 m n j 1 in*)
      let zz = init_ratio arr2 m n j 1 in
      let i = fst zz in
      let r = snd zz in
      let i = depart_var arr2 m n j i r (i+1) in
      let _none = row_op arr2 m n i j in
	simplex arr2 m n
  else ()
in

let main a =
  let m = Bigarray.Array2.dim1 a in
  let n = Bigarray.Array2.dim2 a in
    if m > 1 then begin
      if n > 2 then simplex a m n
      else assert false
    end
    else assert false
in
  Random.self_init();
  let arr = Bigarray.Array2.create Bigarray.float64 Bigarray.c_layout
    (Random.int 20 + 1)
    (Random.int 30 + 1) in
    main arr;;
*/





/*
(*
(* An implementation of the simplex method in DML *)

datatype 'a array2D with (nat,nat) =
  {m:nat,n:nat} A(m,n) of ('a array(n)) array(m) * int(m) * int(n)

fun('a) nRows (A (_, m, _)) = m
withtype {m:nat,n:nat} <> => 'a array2D(m,n) -> int(m)

fun('a) nCols (A (_, _, n)) = n
withtype {m:nat,n:nat} <> => 'a array2D(m,n) -> int(n)

(* step 1 *)

fun is_neg_aux (arr2, n, j) =
    if j < n - 1 then
	if sub2 (arr2, 0, j) <. 0.0 then true
	else is_neg_aux (arr2, n, j+1)
    else false
withtype {m:pos,n:pos,j:nat | j <= n} <n-j> =>
         (float array(n)) array(m) * int(n) * int(j) -> bool

fun is_neg (arr2, n) = is_neg_aux (arr2, n, 1)
withtype {m:pos,n:pos} <> => (float array(n)) array(m) * int(n) -> bool

(* step 2 *)

fun unb1 (arr2, m, n, i, j) =
    if j < n-1 then
	if sub2 (arr2, 0, j) <. 0.0 then unb2 (arr2, m, n, i+1, j)
	else unb1 (arr2, m, n, 0, j+1)
    else false
withtype {m:pos,n:pos,i:nat,j:nat | i < m, j <= n} <n-j, m-i> =>
         (float array(n)) array(m) * int (m) * int(n) * int(i) * int(j) -> bool

and unb2 (arr2, m, n, i, j) =
    if i < m then
	if sub2 (arr2, i, j) <. 0.0 then unb2 (arr2, m, n, i+1, j)
	else unb1 (arr2, m, n, 0, j+1)
    else true
withtype {m:pos,n:pos,i:nat,j:nat | i <= m, j < n} <n-j,m-i> =>
         (float array(n)) array(m) * int (m) * int(n) * int(i) * int(j) -> bool


(* step 3 *)

fun enter_var (arr2, n, j, c, j') =
    if j' < n-1 then
	let
	    val c' = sub2 (arr2, 0, j')
	in
	    if c' <. c then enter_var (arr2, n, j', c', j'+1)
	    else enter_var (arr2, n, j, c, j'+1)
	end
    else j
withtype {m:pos,n:pos,j:pos,j':pos | j+1 < n, j' < n} <n-j'> =>
         (float array(n)) array(m) * int(n) * int(j) * float * int(j') ->
	 [j:pos | j+1 < n] int(j)

(* step 4 *)

fun depart_var (arr2, m, n, j, i, r, i') =
    if i' < m then
	let
	    val c' = sub2 (arr2, i', j)
	in
	    if c' >. 0.0 then
		let
		    val r' = sub2(arr2, i', n-1) /. c'
		in
		    if r' <. r then depart_var(arr2, m, n, j, i', r', i'+1)
		    else depart_var (arr2, m, n, j, i, r, i'+1)
		end
	    else depart_var (arr2, m, n, j, i, r, i'+1)
	end
    else i
withtype {m:pos,n:pos,i:pos,i':pos,j:pos | i < m, i' <= m, j < n} <m-i'> =>
         (float array(n)) array(m) * int(m) * int(n) * int(j) * int(i) * float * int(i') ->
	 [i:pos | i < m] int(i)

fun init_ratio (arr2, m, n, j, i) =
  if i < m then
      let
	  val c = sub2 (arr2, i, j)
      in
	  if c >. 0.0 then (i, sub2 (arr2, i, n-1) /. c)
	  else init_ratio (arr2, m, n, j, i+1)
      end
  else abort ("init_ratio: negative coefficients!")
withtype {m:pos,n:pos,j:pos,i:pos | j < n, i <= m} <m-i> =>
         (float array(n)) array(m) * int(m) * int(n) * int(j) * int(i) ->
         [i:pos | i < m] int(i) * float

(* step 5 *)

fun norm_aux (arr2, n, i, c, j) =
  if j < n then
    let
        val _ = update2 (arr2, i, j, sub2 (arr2, i, j) /. c)
    in
        norm_aux (arr2, n, i, c, j+1)
    end
  else ()
withtype {m:pos,n:pos,i:pos,j:pos | i < m, j <= n} <n-j> =>
         (float array(n)) array(m) * int(n) * int(i) * float * int(j) -> unit

fun norm (arr2, n, i, j) =
  let
      val c = sub2 (arr2, i, j)
  in
      norm_aux (arr2, n, i, c, 1)
  end
withtype {m:pos,n:pos,i:pos,j:pos | i < m, j < n} <> =>
         (float array(n)) array(m) * int(n) * int(i) * int(j) -> unit

fun row_op_aux1 (arr2, n, i, i', c, j) =
  if j < n then
      let
	  val cj =  sub2 (arr2, i, j)
	  val cj' =  sub2 (arr2, i', j)
	  val _ = update2 (arr2, i', j, cj' -. cj *. c)
      in
	  row_op_aux1 (arr2, n, i, i', c, j+1)
      end
  else ()
withtype {m:pos,n:pos,i:pos,i':nat, j:pos | i < m, i' < m, j <= n} <n-j> =>
         (float array(n)) array(m) * int(n) * int(i) * int(i') * float * int(j) -> unit

fun row_op_aux2 (arr2, n, i, i', j) =
  let
      val c' = sub2 (arr2, i', j)
  in
      row_op_aux1 (arr2, n, i, i', c', 1)
  end
withtype {m:pos,n:pos,i:pos,i':nat, j:pos | i < m, i' < m, j < n} <> =>
         (float array(n)) array(m) * int(n) * int(i) * int(i') * int(j) -> unit

fun row_op_aux3 (arr2, m, n, i, j, i') =
  if i' < m then
     if i' <> i then
	 let
	     val _ = row_op_aux2(arr2, n, i, i', j)
	 in
	     row_op_aux3 (arr2, m, n, i, j, i'+1)
         end
     else row_op_aux3 (arr2, m, n, i, j, i'+1)
  else ()
withtype {m:pos,n:pos,i:pos,j:pos,i':nat | i < m, j < n, i' <= m} <m-i'> =>
         (float array(n)) array(m) * int(m) * int(n) * int(i) * int(j) * int(i') -> unit

fun row_op (arr2, m, n, i, j) =
    let
	val _ = norm (arr2, n, i, j)
    in
	row_op_aux3 (arr2, m, n, i, j, 0)
    end
withtype {m:pos,n:pos,i:pos,j:pos| i < m, j < n} <> =>
         (float array(n)) array(m) * int(m) * int(n) * int(i) * int(j) -> unit

fun simplex (arr2, m, n) =
    if is_neg (arr2, n) then
	if unb1 (arr2, m, n, 0, 1) then abort ("simplex: unbound solution!")
	else
	    let
		val j = enter_var (arr2, n, 1, sub2 (arr2, 0, 1), 2)
		val (i, r) = init_ratio (arr2, m, n, j, 1)
		val i = depart_var  (arr2, m, n, j, i, r, i+1)
		val _ = row_op (arr2, m, n, i, j)
	    in
		simplex (arr2, m, n)
	    end
    else ()
withtype {m:int,n:int | m > 1, n > 2}
         (float array(n)) array(m) * int(m) * int(n) -> unit

fun main (A (arr2, m, n)) =
  if m > 1 then
      if n > 2 then simplex (arr2, m, n)
      else abort ("too few columns")
  else abort ("too few rows")
withtype float array2D -> unit
*)
 */

 pub fn main() {}