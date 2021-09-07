// rust port of https://github.com/ucsd-progsys/liquidhaskell/blob/develop/tests/pos/kmpVec.hs

fn kmp_table<T:Eq>(p:&Vec<T>) -> Vec<usize> {
    let     m = p.len();
    let mut t = crate::util::replicate(m, 0);
    let mut i = 1;
    let mut j = 0;
    while i <= m - 1 {
        if p[i] == p[j] {
            t[i] = j + 1;
            i    = i + 1;
            j    = j + 1;
        } else if j == 0 {
            t[i] = 0;
            i    = i + 1;
        } else {
            j = t[j-1];
        }
    };
    t
}

fn kmp_search(pattern:&str, target:&str) -> i32  {
    let target: Vec<char> = target.chars().collect();
    let mut t_i: usize = 0;
    let mut p_i: usize = 0;
    let target_len = target.len();
    let mut result_idx = 0i32;

    let pat:Vec<char> = pattern.chars().collect();
    let t = kmp_table(&pat);
    let pattern_len = pat.len();

    while (t_i <= target_len - 1) && (p_i <= pattern_len - 1) {
        if target[t_i] == pat[p_i] {
            if result_idx == 0 {
                result_idx = t_i as i32;
            }
            t_i = t_i + 1;
            p_i = p_i + 1;
            if p_i >= pattern_len{
                return result_idx
            }
        } else {
            if p_i == 0 {
                p_i = 0;
            } else {
                p_i = t[p_i - 1];
            }
            t_i = t_i + 1;
            result_idx = 0;
        }
    }
    -1
}

pub fn search(pat: &str, str: &str) -> i32 {
    let res = kmp_search(pat, str);
    println!("kmp_search: pat = {}, str = {}, res = {:?}", pat, str, res);
    res
}

