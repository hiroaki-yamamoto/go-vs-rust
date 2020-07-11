use ::std::env;
use ::rug::Integer;

fn fib(n: i32) -> Integer {
    if n < 1 {
        return Integer::from(0);
    }
    let mut mat: Vec<Vec<Integer>> = vec![
        vec![Integer::from(1), Integer::from(1)],
        vec![Integer::from(1), Integer::from(0)],
    ];
    let factor: Vec<Vec<Integer>> = vec![
        vec![Integer::from(1), Integer::from(1)],
        vec![Integer::from(1), Integer::from(0)],
    ];
    for _ in 1..n {
        let mut tmp = vec![
            vec![Integer::new(), Integer::new()],
            vec![Integer::new(), Integer::new()],
        ];
        tmp[0][0] = (
            mat[0][0].clone() * &factor[0][0]
        ) + (
            mat[0][1].clone() * &factor[1][0]
        );
        tmp[0][1] = (
            mat[0][0].clone() * &factor[0][1]
        ) + (
            mat[0][1].clone() * &factor[1][1]
        );
        tmp[1][0] = (
            mat[1][0].clone() * &factor[0][0]
        ) + (
            mat[1][1].clone() * &factor[1][0]
        );
        tmp[1][1] = (
            mat[1][0].clone() * &factor[0][1]
        ) + (
            mat[1][1].clone() * &factor[1][1]
        );
        mat = tmp;
    }
    return mat[0][1].clone();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args[1].parse().unwrap();
    let res = fib(n);
    println!("{}", res.to_string_radix(10));
}
