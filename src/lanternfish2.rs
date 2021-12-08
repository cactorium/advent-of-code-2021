// it's pretty easy to model the laternfish as a vector
// with elements for each possible age from 0 to 8:

// a day can be simulated as multiplying this vector with this matrix:
// [[   0   1   0   0   0   0   0   0   0   0] [[ a0 ]]
// [    0   0   1   0   0   0   0   0   0   0] [[ a1 ]]
// [    0   0   0   1   0   0   0   0   0   0] [[ a2 ]]
// [    0   0   0   0   1   0   0   0   0   0] [[ a3 ]]
// [    0   0   0   0   0   1   0   0   0   0] [[ a4 ]]
// [    0   0   0   0   0   0   1   0   0   0] [[ a5 ]]
// [    1   0   0   0   0   0   0   1   0   0] [[ a6 ]]
// [    0   0   0   0   0   0   0   0   1   0]][[ a7 ]]
// [    1   0   0   0   0   0   0   0   0   0]][[ a8 ]]

// causes an cycle 0 lanternship to to turn into a cycle 7 and
//      cycle 8 laternship
// cycle 1 turns into cycle 0,
// cycle 2 turns into cycle 1, etc.

fn simulate_day(fish: &[u64; 9]) -> [u64; 9] {
    [
        fish[1], // 0
        fish[2], // 1
        fish[3], // 2
        fish[4], // 3
        fish[5], // 4
        fish[6], // 5
        fish[7] + fish[0], // 6
        fish[8], // 7
        fish[0] // 8
    ]
}


// simulating many days at once can be done easily thanks to the
// Cayley-Hamilton theorem, which allows any expression using a given matrix
// to be simplified into a nth degree polynomial, where n is the dimension of
// the square matrix
// this is done because the matrix solves its own characteristic equation,
// and thus any order polynomial expression can be divided by this characteristic
// equation, such that the remainder is of lower order, and the rest can be
// factored out:
// let C(x)=0 be the characteristic equation of the matrix M
// f(M) = G(M)C(M) + R(M) for some polynomial G(x), therefore
// f(M) = G(M)0 + R(M) = R(M)
// TODO maybe do this if the second half asks for a ridiculous number of days

fn main() {
    let mut fish = [0u64; 9];
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    for part in line.trim().split(",") {
        let cycle: usize = str::parse(part).unwrap();
        fish[cycle] += 1;
    }

    println!("{:?}", &fish);
    for _ in 0..256 {
        fish = simulate_day(&fish);
        println!("{:?} {}", &fish, fish.iter().sum::<u64>());
    }
}
