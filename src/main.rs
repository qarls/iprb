use clap::Parser;

const ABOUT_MESSAGE: &str = "Compute the Hamming distance between s and t.";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// Count for individuals with homozygous dominant genotype.
    dom_count: usize,
    /// Count for individuals with heterozygous genotype.
    het_count: usize,
    /// Count for individuals with homozygous recesivve genotype.
    rec_count: usize,
}

fn factorial(num: usize) -> u128 {
    if num <= 1 {
        1
    } else {
        let mut result: u128 = 1;
        for i in 2..=num {
            result *= i as u128;
        }
        result
    }
}

fn combination(n: usize, r: usize) -> usize {
    if n < r {
        panic!("Combination detected {n} < {r}!");
    } else {
        (factorial(n) / factorial(r) / factorial(n - r))
            .try_into()
            .unwrap()
    }
}

fn dominant_phenotype(k: usize, m: usize, n: usize) -> f64 {
    // Where kc (KK) represents the combinations of K to pair with themselves
    // KK, KM, KT weighed 1 (full chance to pass on)
    // MM weighed 0.75
    // MT weighed 0.5
    // TT weighed 0
    let total_com = combination(k + m + n, 2) as f64;
    let kc = combination(k, 2);
    let mc = combination(m, 2) as f64;
    //let nc = combination(n, 2) as f64;

    ((kc + k * m + k * n) as f64 + (mc * 3. / 4.) + ((m * n) as f64 / 2.)) / total_com
}

fn main() {
    let args = Cli::parse();
    let (k, m, n) = (args.dom_count, args.het_count, args.rec_count);

    println!("{:.5}", dominant_phenotype(k, m, n));
}
