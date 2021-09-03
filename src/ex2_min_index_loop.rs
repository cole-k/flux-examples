
// reduce using loop
fn w_reduce<A, B, F>(acc:A, vec:&Vec<B>, f: F) -> A 
  where F: Fn(usize, A, &B) -> A 
{ 
    let mut res = acc;
    let sz = vec.len();
    let mut i = 0;
    while i < sz {
        res = f(i, res, &vec[i]);
        i = i + 1;

    }
    res
}

// min_index using reduce
fn min_index(vec:Vec<i32>) -> usize {
    let step = |i, acc, val:&i32| { 
        if *val < vec[acc]  {
            i
        } else {
            acc
        }
    };
    
    w_reduce(0, &vec, step)

}

pub fn test() {
    let v0:Vec<i32> = vec![11,30,2,12,41,10,15,32,1,99];
    println!("ex_2_min_index_loop = {:?}", min_index(v0));
}