use anyhow::Result;

fn fuel_for_mass(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn fuel_for_mass_and_fuel(mass: usize) -> usize {
    let mut fuel_inc = fuel_for_mass(mass);
    let mut fuel_sum = fuel_inc;
    while fuel_inc > 0 {
        fuel_inc = fuel_for_mass(fuel_inc);
        fuel_sum += fuel_inc;
    }    
    fuel_sum
}    

fn sum_fuel(masses: &[usize], fuel_fn: fn (usize) -> usize) -> usize {
    masses.into_iter().fold(0, |acc, i| acc + fuel_fn(*i))
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let masses  = data.lines().map(|s| s.parse::<usize>()).collect::<Result<Vec<_>,_>>()?;
    Ok((sum_fuel(&masses, fuel_for_mass), sum_fuel(&masses, fuel_for_mass_and_fuel)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33583);
    }    
    
    #[test]
    fn for_mass_and_fuel() {
        assert_eq!(fuel_for_mass_and_fuel(14), 2);
        assert_eq!(fuel_for_mass_and_fuel(1969), 966);
        assert_eq!(fuel_for_mass_and_fuel(100756), 50346);
    }    
    
    #[test]
    fn sum() {
        assert_eq!(sum_fuel(&vec![12, 14, 1969, 100756], fuel_for_mass), 2+2+654+33583);
        assert_eq!(sum_fuel(&vec![14, 1969, 100756], fuel_for_mass_and_fuel), 2+966+50346);
    }
}
