fn main() {
    let mut res = Vec::new();
    for i in 10..99 {
        for j in (i + 1)..99 {
            let v = i as f64 / j as f64;
            let (q_i, r_i, q_j, r_j) = (i / 10, i % 10, j / 10, j % 10);
            if q_i == q_j {
                if v == r_i as f64 / r_j as f64 {
                    res.push(((i, j), (r_i, r_j)));
                }
            }
            if q_i == r_j {
                if v == r_i as f64 / q_j as f64 {
                    res.push(((i, j), (r_i, q_j)));
                }
            }
            if r_i == q_j {
                if v == q_i as f64 / r_j as f64 {
                    res.push(((i, j), (q_i, r_j)));
                }
            }
            if r_i == r_j && r_i != 0 {
                if v == q_i as f64 / q_j as f64 {
                    res.push(((i, j), (q_i, q_j)));
                }
            }
        }
    }
    let mut n_prod = 1;
    let mut d_prod = 1;
    for (_, (n, d)) in res {
        n_prod *= n;
        d_prod *= d;
    }
    let mut q = 2;
    while q <= n_prod {
        if n_prod % q == 0 && d_prod % q == 0 {
            n_prod /= q;
            d_prod /= q;
        } else {
            q += 1
        }
    }
    println!("Res: {d_prod}");
}
