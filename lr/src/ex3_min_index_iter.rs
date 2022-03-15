fn reduce<A, B, F>(acc:A, vec:&Vec<B>, f: F) -> A 
  where F: Fn(usize, A, &B) -> A 
{ 
    let mut res = acc;
    for (i, val) in vec.iter().enumerate() {
        res = f(i, res, val);
    }  
    res
}

fn min_index(vec:Vec<i32>) -> usize {
    let step = |i, acc, val:&i32| { 
        if *val < vec[acc]  {
            i
        } else {
            acc
        }
    };

    // iterator version
    reduce(0, &vec, step)
}


pub fn test() {
    let v0:Vec<i32> = vec![11,30,2,12,41,10,15,32,1,99];
    println!("ex_3_min_index_iter = {:?}", min_index(v0));
}