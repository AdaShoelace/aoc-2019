pub fn first(input: Vec<String>) -> f64 {
    input
        .iter()
        .map(|line| line.parse::<f64>().unwrap())
        .map(fuel)
        .sum::<f64>()
}

pub fn second(input: Vec<String>) -> f64 {
    input
        .into_iter()
        .map(|line| line.parse::<f64>().unwrap())
        .map(|line| fuel_rec(fuel(line)))
        .sum::<f64>()
}

fn fuel(mass: f64) -> f64 {
    let ret = (mass / 3.).floor() - 2.;
    if ret < 0. { return 0. }
    ret
}

fn fuel_rec(mut input: f64) -> f64 {
    match input {
        0f64 => input,
        _ => input + fuel_rec(fuel(input))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_rec() {
        assert_eq!(fuel_rec(fuel(1969f64)), 966f64);
    }

}
