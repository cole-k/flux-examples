
pub fn test(val:i32) {
    // create a vector
    let mut vec:Vec<i32> = Vec::new();
    let mut k = val;

    println!("ex_1_fill_and_check!");

    // fill it with values >= val
    while crate::util::toss() {
        vec.push(k);
        k = k + 1
    }

    // assert values >= val using iter
    for v in vec.iter() {
       assert_eq!(val <= *v, true);
    }

    // assert values >= val using plain pop 
    while !vec.is_empty() {
        if let Some(v) = vec.pop() {
            assert_eq!(val <= v, true)
        }
    }
}



