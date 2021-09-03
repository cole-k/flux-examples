mod util;

mod ex1_fill;
mod ex2_min_index_loop;
mod ex3_min_index_iter;
mod ex4_kmp;

fn main() {
    println!("Hello, world!");
    ex1_fill::test(10);
    ex2_min_index_loop::test();
    ex3_min_index_iter::test();
    ex4_kmp::search("bro", "thequickbrownfox");
}
