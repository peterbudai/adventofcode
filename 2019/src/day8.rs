use anyhow::Result;
use itertools::Itertools;

pub fn solution(data: &str) -> Result<(usize, String)> {
    let layer_size = 25 * 6;

    let check_layer = data.chars()
        .chunks(layer_size).into_iter()
        .map(|l| l.fold(
            vec![0; 4], 
            |mut s, p| { 
                s[p.to_digit(10).unwrap_or(3) as usize] += 1; 
                s 
            }
        ))
        .min_by_key(|ls| ls[0]).unwrap();
    
    let image = (0..layer_size).map(|i| data.chars().skip(i).step_by(layer_size).fold('2', |p, c| if p == '2' { c } else { p }))
        .map(|p| match p { '1' => "##", _ => "  " }).chunks(25).into_iter().map(|l| l.collect::<String>()).fold(String::new(), |s, l| s + "\n" + &l);
    
    Ok((check_layer[1] * check_layer[2], image))
}
