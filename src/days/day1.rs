use super::Day;

pub struct Day1;

impl Day for Day1 {
    fn first(&self, input: Vec<String>) -> u64 {
        input
            .iter()
            .map(|line| line.parse::<f64>().unwrap())
            .map(fuel)
            .sum::<f64>() as u64
    }

    fn second(&self, input: Vec<String>) -> u64 {
        input
            .into_iter()
            .map(|line| line.parse::<f64>().unwrap())
            .map(|line| fuel_rec(fuel(line)))
            .sum::<f64>() as u64
    }
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
