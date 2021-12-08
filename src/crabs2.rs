// so the fuel used to align the crabs to a given position
// can be modeled as a correlation
// of the crab density function with an absolute value function,
// which can be done in O(x^2) naively or in O(x log x) time
// using some form of FFT, where x is the range of positions
// of crabs

// the correlation is equivalent because the cost of fuel for a given crab
// only depends on the relative position of the crab from the chosen
// alignment position,
//
// fuel_i(x) = cost(x - x_i)
//
// and therefore the sum of all the fuel costs can be written as
// fuel(x) = sum_i cost(x - x_i) ,
// which can be parameterized over the number of crabs in each position:
// fuel(x) = sum_j count(x_i == j) * cost(x - x_i)
// giving the correlation mentioned above

// the correlation can be calculated using the DFT, multiplying
// componentwise (with the complex conjugate in the case of
// correlation instead of convolution), and then taking the
// inverse DFT

/*
// straight up copied from Wikipedia
fn twiddle(&(ref a, ref b): &(f64, f64), k: usize, n: usize) -> (f64, f64) {
    let phase = 2*std::consts::PI/(n as f64) * (k as f64);
    let real = phase.cos();
    let imag = phase.sin();
    (*a * real - *b * imag, *a * imag + *b * real)
}

fn difft2(x: &[(f64, f64)], n: usize, stride: usize) -> Vec<(f64, f64)> {
    if n == 1 {
        return;
    }
    let mut output = Vec::new();
    output.append(difft2(&x[0..n], n/2, 2*stride));
    output.append(difft2(&x[s..n], n/2, 2*stride));
    for k in 0..(n/2) {
        let p = output[k*s];
        let q = twiddle(&output[k*s + n/2], k, n);
        output[k*s][0] = p[0] + q[0];
        output[k*s][1] = p[1] + q[1];
        output[k*s + n/2][0] = p[0] - q[0];
        output[k*s + n/2][1] = p[1] - q[1];
    }
    output
}
*/

// after looking at the input, let's just go with the dumb O(n^2) option

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    let crabs: Vec<_> = buffer
        .trim()
        .split(",")
        .map(|s| str::parse::<isize>(s).unwrap())
        .collect();

    let max_x = crabs.iter().map(|v| *v).fold(0, std::cmp::max);
    let min_x = crabs.iter().map(|v| *v).fold(max_x, std::cmp::min);

    let cost: Vec<_> = (min_x..(max_x+1))
        .map(|x| crabs.iter().map(|cx| {
            let dx = (cx - x).abs();
            (dx * (dx + 1))/2
        }).sum())
        .collect();

    let max_cost = cost.iter().map(|v| *v).fold(0, std::cmp::max);
    let min_cost = cost.iter().map(|v| *v).fold(max_cost, std::cmp::min);
    println!("{}", min_cost);
}
