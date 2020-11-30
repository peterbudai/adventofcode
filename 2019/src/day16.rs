use anyhow::Result;
use itertools::Itertools;

fn parse_input(data: &str) -> Vec<isize> {
    data.chars().filter_map(|c| c.to_digit(10).map(|d| d as isize)).collect()
}

fn format_output(input: &[isize]) -> String {
    input.iter().take(8).join("")
}

fn create_pattern(element: usize, len: usize) -> impl Iterator<Item=isize> {
    (0..=0).cycle().take(element+1)
        .chain((1..=1).cycle().take(element+1))
        .chain((0..=0).cycle().take(element+1))
        .chain((-1..=-1).cycle().take(element+1))
        .cycle().skip(1).take(len)
}

fn fft_element(element: usize, input: &[isize]) -> isize {
    input.iter().zip(create_pattern(element, input.len()))
        .skip(element)
        .map(|(i,p)| i*p)
        .sum::<isize>().abs() % 10
}

fn fft_input(input: &[isize]) -> Vec<isize> {
    (0..input.len()).map(|e| fft_element(e, input)).collect()
}

fn fft_phases(input: &[isize], phases: usize) -> Vec<isize> {
    let mut v = input.to_owned();
    for _ in 0..phases {
        v = fft_input(&v);
    }
    v
}

fn message_offset(input: &[isize]) -> usize {
    input[0..7].iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse().unwrap()
}

fn message_repeat(input: &[isize], repeat: usize) -> Vec<isize> {
    input.iter().cycle().take(input.len() * repeat).copied().collect()
}

fn fft_simple(input: &[isize]) -> Vec<isize> {
    let mut output = vec![0; input.len()];
    output[0] = input[input.len()-1];
    for (i, n) in input.iter().rev().enumerate().skip(1) {
        output[i] = (output[i-1] + n) % 10;
    }
    output.reverse();
    output
}

fn fft_simple_repeat(input: &[isize], repeat: usize, phases: usize) -> Vec<isize> {
    let offset = message_offset(input);

    let mut output = message_repeat(input, repeat).into_iter().skip(offset).collect_vec();
    for _ in 0..phases {
        output = fft_simple(&output);
    }
    output
}

pub fn solution(data: &str) -> Result<(String, String)> {
    let input = parse_input(data);
    Ok((
        format_output(&fft_phases(&input, 100)), 
        format_output(&fft_simple_repeat(&input, 10000, 100))
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_output() {
        assert_eq!(parse_input("12345678"), [1,2,3,4,5,6,7,8]);

        assert_eq!(format_output(&[1,2,3,4,5,6,7,8]), "12345678");
        assert_eq!(format_output(&[5,6,7,8,0,0,1,2,3,4]), "56780012");
    }

    #[test]
    fn pattern() {
        itertools::assert_equal(create_pattern(0, 0), vec![]);
        itertools::assert_equal(create_pattern(1, 0), vec![]);
        itertools::assert_equal(create_pattern(2, 0), vec![]);

        itertools::assert_equal(create_pattern(0, 3), vec![1,0,-1]);
        itertools::assert_equal(create_pattern(1, 2), vec![0,1]);
        itertools::assert_equal(create_pattern(2, 1), vec![0]);

        itertools::assert_equal(create_pattern(0, 8), vec![1,0,-1,0,1,0,-1,0]);
        itertools::assert_equal(create_pattern(1, 15), vec![0,1,1,0,0,-1,-1,0,0,1,1,0,0,-1,-1]);
        itertools::assert_equal(create_pattern(2, 15), vec![0,0,1,1,1,0,0,0,-1,-1,-1,0,0,0,1]);
    }

    #[test]
    fn fft_basic() {
        let input = [1,2,3,4,5,6,7,8];

        assert_eq!(fft_element(0, &input), 4);
        assert_eq!(fft_element(1, &input), 8);
        assert_eq!(fft_element(2, &input), 2);
        assert_eq!(fft_element(3, &input), 2);
        assert_eq!(fft_element(4, &input), 6);
        assert_eq!(fft_element(5, &input), 1);
        assert_eq!(fft_element(6, &input), 5);
        assert_eq!(fft_element(7, &input), 8);

        let input = fft_input(&input); 
        assert_eq!(input, [4,8,2,2,6,1,5,8]);
        let input = fft_input(&input); 
        assert_eq!(input, [3,4,0,4,0,4,3,8]);
        let input = fft_input(&input); 
        assert_eq!(input, [0,3,4,1,5,5,1,8]);
        let input = fft_input(&input); 
        assert_eq!(input, [0,1,0,2,9,4,9,8]);
    }

    #[test]
    fn fft_large() {
        assert_eq!(format_output(&fft_phases(&parse_input("80871224585914546619083218645595"), 100)), "24176176");
        assert_eq!(format_output(&fft_phases(&parse_input("19617804207202209144916044189917"), 100)), "73745418");
        assert_eq!(format_output(&fft_phases(&parse_input("69317163492948606335995924319873"), 100)), "52432133");
    }

    #[test]
    fn repeat() {
        assert_eq!(message_repeat(&[1,2,3,4], 0), []);
        assert_eq!(message_repeat(&[1,2,3], 1), [1,2,3]);
        assert_eq!(message_repeat(&[1,2], 3), [1,2,1,2,1,2]);
        assert_eq!(message_repeat(&[1], 10), [1,1,1,1,1,1,1,1,1,1]);
    }

    #[test]
    fn fft_message_decode() {
        let input = parse_input("03036732577212944063491565474664");
        assert_eq!(message_offset(&input), 303673);
        assert_eq!(format_output(&fft_simple_repeat(&input, 10000, 100)), "84462026");

        let input = parse_input("02935109699940807407585447034323");
        assert_eq!(message_offset(&input), 293510);
        assert_eq!(format_output(&fft_simple_repeat(&input, 10000, 100)), "78725270");

        let input = parse_input("03081770884921959731165446850517");
        assert_eq!(message_offset(&input), 308177);
        assert_eq!(format_output(&fft_simple_repeat(&input, 10000, 100)), "53553731");
    }
}