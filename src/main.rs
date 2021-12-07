mod util;

mod ex1_fill;
mod ex2_min_index_loop;
mod ex3_min_index_iter;
mod ex4_kmp;




enum List(K) {
    Cons(i32{K} , Box<List>),
    Nil,
}



enum List /* <L, K> */ {
    Cons(i32{K} , Box<List /* @L (K)*/>),
    Nil,
}

fn foo(){

    let mut list = List::Nil;
    
    // list: @L1#  K           L1 |-> a                 a:{a = Nil}

    for i in 0..5 {

        // list: @L1#, i: @I         L1# |-> a, I |-> b           b:{len v = 0}

        list = List::Cons(i, Box::new(list));
       
        // list: @L2, i: @I         L1 |-> ???,  L2 |-> ???

        println!("{}", i)
    }

    let mut cursor = &list;

    while let List::Cons(vec, rest) = &cursor {
        assert!(0 <= *vec);
        cursor = rest;
    }

}


fn main() {
    println!("Hello, world!");
    ex1_fill::test(10);
    ex2_min_index_loop::test();
    ex3_min_index_iter::test();
    ex4_kmp::search("bro", "thequickbrownfox");
}
