use anyhow::Result;

fn digits(n: u32) -> [u32; 6] {
    [n / 100000, n % 100000 / 10000, n % 10000 / 1000, n % 1000 / 100, n % 100 / 10, n % 10]
}

#[cfg(test)]
#[test]
fn test_digits() {
    assert_eq!(digits(122345), [1,2,2,3,4,5]);
    assert_eq!(digits(111123), [1,1,1,1,2,3]);
    assert_eq!(digits(135679), [1,3,5,6,7,9]);
    assert_eq!(digits(111111), [1,1,1,1,1,1]);
    assert_eq!(digits(223450), [2,2,3,4,5,0]);
    assert_eq!(digits(123789), [1,2,3,7,8,9]);
}

fn not_decrease(digits: &[u32; 6]) -> bool {
    digits[0] <= digits[1] &&  digits[1] <= digits[2] &&  digits[2] <= digits[3] && digits[3] <= digits[4] && digits[4] <= digits[5]
}

#[cfg(test)]
#[test]
fn test_not_decrease() {
    assert!(not_decrease(&digits(122345)));
    assert!(not_decrease(&digits(135679)));
    assert!(not_decrease(&digits(111111)));
    assert!(!not_decrease(&digits(121123)));
    assert!(!not_decrease(&digits(223450)));
    assert!(!not_decrease(&digits(987321)));
}

fn has_double(digits: &[u32; 6]) -> bool {
    digits[0] == digits[1] || digits[1] == digits[2] || digits[2] == digits[3] || digits[3] == digits[4] || digits[4] == digits[5]
}

#[cfg(test)]
#[test]
fn test_has_double() {
    assert!(has_double(&digits(122345)));
    assert!(!has_double(&digits(135679)));
    assert!(has_double(&digits(111111)));
    assert!(has_double(&digits(121123)));
    assert!(has_double(&digits(223450)));
    assert!(!has_double(&digits(987321)));
}

fn has_strict_double(digits: &[u32; 6]) -> bool {
    (digits[0] == digits[1] && digits[1] != digits[2]) || 
    (digits[0] != digits[1] && digits[1] == digits[2] && digits[2] != digits[3]) || 
    (digits[1] != digits[2] && digits[2] == digits[3] && digits[3] != digits[4]) || 
    (digits[2] != digits[3] && digits[3] == digits[4] && digits[4] != digits[5]) || 
    (digits[3] != digits[4] && digits[4] == digits[5])
}

#[cfg(test)]
#[test]
fn test_has_strict_double() {
    assert!(has_strict_double(&digits(122345)));
    assert!(!has_strict_double(&digits(135679)));
    assert!(!has_strict_double(&digits(111111)));
    assert!(has_strict_double(&digits(121123)));
    assert!(has_strict_double(&digits(223450)));
    assert!(!has_strict_double(&digits(987321)));
    assert!(has_strict_double(&digits(112233)));
    assert!(!has_strict_double(&digits(123444)));
    assert!(has_strict_double(&digits(111122)));
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let limits = data.split('-').map(|s| s.parse::<u32>()).collect::<Result<Vec<_>, _>>()?;
    
    Ok((limits[0]..limits[1]+1)
        .into_iter()
        .map(|n| digits(n))
        .filter(|d| not_decrease(d))
        .fold((0usize, 0usize), 
            |(c1, c2), d| (
                if has_double(&d) { c1 + 1 } else { c1 },
                if has_strict_double(&d) { c2 + 1 } else { c2 }
            )
        )
    )
}
