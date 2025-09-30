use clap::Parser;

const ABOUT_MESSAGE: &str = "Compute the chance of expected offspring with dominant allele from a random pairing in the population.";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// Count for individuals with homozygous dominant genotype.
    dom_count: usize,
    /// Count for individuals with heterozygous genotype.
    het_count: usize,
    /// Count for individuals with homozygous recessive genotype.
    rec_count: usize,
}

fn comb_two(n: usize) -> usize {
    // n*n - n is an identity from using r = 2, keeping us from hitting int overflow
    // n*n - n will always be even
    // >> bit right shift of one place is a quicker divide for even numbers
    (n * n - n) >> 1
}

fn dominant_phenotype(k: usize, m: usize, n: usize) -> f64 {
    // Where kc (KK) represents the combinations of K to pair with themselves
    // KK, KM, KT weighed 1 (full chance to pass on)
    // MM weighed 0.75
    // MT weighed 0.5
    // TT weighed 0
    let total_com = comb_two(k + m + n) as f64;
    let kc = comb_two(k);
    let mc = comb_two(m) as f64;
    //let nc = combination(n, 2) as f64;

    ((kc + k * m + k * n) as f64 + (mc * 3. / 4.) + ((m * n) as f64 / 2.)) / total_com
}

fn main() {
    let args = Cli::parse();
    let (k, m, n) = (args.dom_count, args.het_count, args.rec_count);

    println!("{:.5}", dominant_phenotype(k, m, n));
}
