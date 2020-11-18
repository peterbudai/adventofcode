use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use num::Integer;

type Coord3 = (isize, isize, isize);

fn subsignum((x1, y1, z1): &Coord3, (x2, y2, z2): &Coord3) -> Coord3 {
    ((x1 - x2).signum(), (y1 - y2).signum(), (z1 -z2).signum())
}

fn add((x1, y1, z1): &mut Coord3, (x2, y2, z2): &Coord3) {
    *x1 += *x2;
    *y1 += *y2;
    *z1 += *z2;
}

fn sumabs((x, y, z): &Coord3) -> isize {
    x.abs() + y.abs() + z.abs()
}

fn parse_coord(line: &str) -> Result<Coord3> {
    Regex::new(r"<\s*x=\s*(-?\d+)\s*,\s*y=\s*(-?\d+)\s*,\s*z=\s*(-?\d+)\s*>")?
        .captures(line).ok_or(anyhow!("Invalid input"))?
        .iter().skip(1).filter_map(|o| o.map(|m| m.as_str().parse::<isize>().unwrap()))
        .collect_tuple::<Coord3>().ok_or(anyhow!("Invalid input"))
}

fn parse_input(data: &str) -> Result<(Vec<Coord3>, Vec<Coord3>)> {
    let p = data.lines().map(|line| parse_coord(line)).collect::<Result<Vec<_>, _>>()?;
    let v = vec![(0, 0, 0); p.len()];
    Ok((p, v))
}

fn energy(positions: &[Coord3], velocities: &[Coord3]) -> usize {
    positions.iter().map(|p| sumabs(p))
        .zip(
            velocities.iter().map(|v| sumabs(v))
        )
        .map(|(pot, kin)| (pot * kin) as usize)
        .sum()
}

fn step(positions: &mut [Coord3], velocities: &mut [Coord3]) {
    for (i, p0) in positions.iter().enumerate() {
        for p1 in positions.iter() {
            add(&mut velocities[i], &subsignum(p1, p0));
        }
    }

    for (i, p) in positions.iter_mut().enumerate() {
        add(p, &velocities[i]);
    }
}

fn separate_axis(positions: &[Coord3], velocities: &[Coord3], axis: u8) -> (Vec<isize>, Vec<isize>) 
{
    let axisfn: fn(&Coord3)->isize = match axis {
        0 => |(x, _, _)| *x,
        1 => |(_, y, _)| *y,
        2 => |(_, _, z)| *z,
        _ => unreachable!()
    };
    (positions.iter().map(|p| axisfn(p)).collect_vec(), velocities.iter().map(|p| axisfn(p)).collect_vec())
}

fn step_axis(positions: &mut [isize], velocities: &mut [isize]) {
    for (i, v) in velocities.iter_mut().enumerate() {
        let p0 = positions[i];
        *v += positions.iter().map(|p| (p - p0).signum()).sum::<isize>();
    }
    for (i, p) in positions.iter_mut().enumerate() {
        *p += velocities[i];
    }
}

fn find_repeat_in_axis(positions: &[isize], velocities: &[isize]) -> usize {
    let mut v = velocities.to_owned();
    let mut p = positions.to_owned();

    let mut steps = 0;
    loop {
        step_axis(&mut p, &mut v);
        steps += 1;

        if &v[..] == velocities {
            break;
        }
    }
    steps * 2
}

fn find_repeat(positions: &[Coord3], velocities: &[Coord3]) -> usize {
    (0..3).into_iter().map(|axis| {
        let (p, v) = separate_axis(&positions, &velocities, axis);
        find_repeat_in_axis(&p, &v)
    }).fold1(|a, s| a.lcm(&s)).unwrap()
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let (mut positions, mut velocities) = parse_input(data)?;

    let repeat = find_repeat(&positions, &velocities);
    let energy = {
        for _ in 0..1000 {
            step(&mut positions, &mut velocities);
        }
        energy(&positions, &velocities)
    };

    Ok((energy, repeat))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    fn parse_test_vector(data: &str) -> (Vec<Coord3>, Vec<Coord3>) {
        data.lines().map(|line|
            Regex::new(r"pos=\s*(<[^>]+>)\s*,\s*vel=\s*(<[^>]+>)").unwrap()
            .captures(line).unwrap()
            .iter().skip(1).filter_map(|o| o.map(|m|
                parse_coord(m.as_str()).unwrap()
            )).collect_tuple::<(Coord3, Coord3)>().unwrap()
        ).unzip()
    }

    fn input1() -> (Vec<Coord3>, Vec<Coord3>) {
        parse_input(indoc!(
            "<x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>"
        )).unwrap()
    }

    fn input2() -> (Vec<Coord3>, Vec<Coord3>) {
        parse_input(indoc!(
            "<x=-8, y=-10, z=0>
             <x=5, y=5, z=10>
             <x=2, y=-7, z=3>
             <x=9, y=-8, z=-3>"
        )).unwrap()
    }

    #[test]
    fn parse() {
        assert_eq!(parse_coord("<x=-1, y=0, z=2>").unwrap(), (-1, 0, 2));
        assert_eq!(parse_coord("<x=2,y=-10,z=-7>").unwrap(), (2, -10, -7));
        assert_eq!(parse_coord("<x= 4,y= -8,z= 8>").unwrap(), (4, -8, 8));
        assert_eq!(parse_coord("<x=3 , y=5 , z=-1>").unwrap(), (3, 5, -1));
    }

    #[test]
    fn steps1() {
        let (mut p, mut v) = input1();

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>
             pos=<x= 2, y=-10, z=-7>, vel=<x= 0, y= 0, z= 0>
             pos=<x= 4, y= -8, z= 8>, vel=<x= 0, y= 0, z= 0>
             pos=<x= 3, y=  5, z=-1>, vel=<x= 0, y= 0, z= 0>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
             pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
             pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
             pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>
             pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>
             pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>
             pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
       
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 5, y=-6, z=-1>, vel=<x= 0, y=-3, z= 0>
             pos=<x= 0, y= 0, z= 6>, vel=<x=-1, y= 2, z= 4>
             pos=<x= 2, y= 1, z=-5>, vel=<x= 1, y= 5, z=-4>
             pos=<x= 1, y=-8, z= 2>, vel=<x= 0, y=-4, z= 0>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
        
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 2, y=-8, z= 0>, vel=<x=-3, y=-2, z= 1>
             pos=<x= 2, y= 1, z= 7>, vel=<x= 2, y= 1, z= 1>
             pos=<x= 2, y= 3, z=-6>, vel=<x= 0, y= 2, z=-1>
             pos=<x= 2, y=-9, z= 1>, vel=<x= 1, y=-1, z=-1>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
        
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=-1, y=-9, z= 2>, vel=<x=-3, y=-1, z= 2>
             pos=<x= 4, y= 1, z= 5>, vel=<x= 2, y= 0, z=-2>
             pos=<x= 2, y= 2, z=-4>, vel=<x= 0, y=-1, z= 2>
             pos=<x= 3, y=-7, z=-1>, vel=<x= 1, y= 2, z=-2>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
        
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=-1, y=-7, z= 3>, vel=<x= 0, y= 2, z= 1>
             pos=<x= 3, y= 0, z= 0>, vel=<x=-1, y=-1, z=-5>
             pos=<x= 3, y=-2, z= 1>, vel=<x= 1, y=-4, z= 5>
             pos=<x= 3, y=-4, z=-2>, vel=<x= 0, y= 3, z=-1>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
        
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 2, y=-2, z= 1>, vel=<x= 3, y= 5, z=-2>
             pos=<x= 1, y=-4, z=-4>, vel=<x=-2, y=-4, z=-4>
             pos=<x= 3, y=-7, z= 5>, vel=<x= 0, y=-5, z= 4>
             pos=<x= 2, y= 0, z= 0>, vel=<x=-1, y= 4, z= 2>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
        
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 5, y= 2, z=-2>, vel=<x= 3, y= 4, z=-3>
             pos=<x= 2, y=-7, z=-5>, vel=<x= 1, y=-3, z=-1>
             pos=<x= 0, y=-9, z= 6>, vel=<x=-3, y=-2, z= 1>
             pos=<x= 1, y= 1, z= 3>, vel=<x=-1, y= 1, z= 3>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
        
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 5, y= 3, z=-4>, vel=<x= 0, y= 1, z=-2>
             pos=<x= 2, y=-9, z=-3>, vel=<x= 0, y=-2, z= 2>
             pos=<x= 0, y=-8, z= 4>, vel=<x= 0, y= 1, z=-2>
             pos=<x= 1, y= 1, z= 5>, vel=<x= 0, y= 0, z= 2>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
        
        step(&mut p, &mut v);

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
             pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
             pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
             pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
    }

    #[test]
    fn steps2() {
        let (mut p, mut v) = input2();

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= -8, y=-10, z=  0>, vel=<x=  0, y=  0, z=  0>
             pos=<x=  5, y=  5, z= 10>, vel=<x=  0, y=  0, z=  0>
             pos=<x=  2, y= -7, z=  3>, vel=<x=  0, y=  0, z=  0>
             pos=<x=  9, y= -8, z= -3>, vel=<x=  0, y=  0, z=  0>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= -9, y=-10, z=  1>, vel=<x= -2, y= -2, z= -1>
             pos=<x=  4, y= 10, z=  9>, vel=<x= -3, y=  7, z= -2>
             pos=<x=  8, y=-10, z= -3>, vel=<x=  5, y= -1, z= -2>
             pos=<x=  5, y=-10, z=  3>, vel=<x=  0, y= -4, z=  5>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=-10, y=  3, z= -4>, vel=<x= -5, y=  2, z=  0>
             pos=<x=  5, y=-25, z=  6>, vel=<x=  1, y=  1, z= -4>
             pos=<x= 13, y=  1, z=  1>, vel=<x=  5, y= -2, z=  2>
             pos=<x=  0, y=  1, z=  7>, vel=<x= -1, y= -1, z=  2>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 15, y= -6, z= -9>, vel=<x= -5, y=  4, z=  0>
             pos=<x= -4, y=-11, z=  3>, vel=<x= -3, y=-10, z=  0>
             pos=<x=  0, y= -1, z= 11>, vel=<x=  7, y=  4, z=  3>
             pos=<x= -3, y= -2, z=  5>, vel=<x=  1, y=  2, z= -3>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 14, y=-12, z= -4>, vel=<x= 11, y=  3, z=  0>
             pos=<x= -1, y= 18, z=  8>, vel=<x= -5, y=  2, z=  3>
             pos=<x= -5, y=-14, z=  8>, vel=<x=  1, y= -2, z=  0>
             pos=<x=  0, y=-12, z= -2>, vel=<x= -7, y= -3, z= -3>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=-23, y=  4, z=  1>, vel=<x= -7, y= -1, z=  2>
             pos=<x= 20, y=-31, z= 13>, vel=<x=  5, y=  3, z=  4>
             pos=<x= -4, y=  6, z=  1>, vel=<x= -1, y=  1, z= -3>
             pos=<x= 15, y=  1, z= -5>, vel=<x=  3, y= -3, z= -3>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 36, y=-10, z=  6>, vel=<x=  5, y=  0, z=  3>
             pos=<x=-18, y= 10, z=  9>, vel=<x= -3, y= -7, z=  5>
             pos=<x=  8, y=-12, z= -3>, vel=<x= -2, y=  1, z= -7>
             pos=<x=-18, y= -8, z= -2>, vel=<x=  0, y=  6, z= -1>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=-33, y= -6, z=  5>, vel=<x= -5, y= -4, z=  7>
             pos=<x= 13, y= -9, z=  2>, vel=<x= -2, y= 11, z=  3>
             pos=<x= 11, y= -8, z=  2>, vel=<x=  8, y= -6, z= -7>
             pos=<x= 17, y=  3, z=  1>, vel=<x= -1, y= -1, z= -3>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x= 30, y= -8, z=  3>, vel=<x=  3, y=  3, z=  0>
             pos=<x= -2, y= -4, z=  0>, vel=<x=  4, y=-13, z=  2>
             pos=<x=-18, y= -7, z= 15>, vel=<x= -8, y=  2, z= -2>
             pos=<x= -2, y= -1, z= -8>, vel=<x=  1, y=  8, z=  0>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=-25, y= -1, z=  4>, vel=<x=  1, y= -3, z=  4>
             pos=<x=  2, y= -9, z=  0>, vel=<x= -3, y= 13, z= -1>
             pos=<x= 32, y= -8, z= 14>, vel=<x=  5, y= -4, z=  6>
             pos=<x= -1, y= -2, z= -8>, vel=<x= -3, y= -6, z= -9>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        let (pt, vt) = parse_test_vector(indoc!(
            "pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
             pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
             pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
             pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>"
        ));
        assert_eq!(p, pt);
        assert_eq!(v, vt);
    }

    #[test]
    fn energy1() {
        let (mut p, mut v) = input1();

        for _ in 0..10 {
            step(&mut p, &mut v);
        }

        assert_eq!(energy(&p, &v), 179);
    }        

    #[test]
    fn energy2() {
        let (mut p, mut v) = input2();

        for _ in 0..100 {
            step(&mut p, &mut v);
        }

        assert_eq!(energy(&p, &v), 1940);
    }

    #[test]
    fn per_axis() {
        let (mut p, mut v) = input1();

        let (mut px, mut vx) = separate_axis(&p, &v, 0);
        let (mut py, mut vy) = separate_axis(&p, &v, 1);
        let (mut pz, mut vz) = separate_axis(&p, &v, 2);

        step_axis(&mut px, &mut vx);
        step_axis(&mut py, &mut vy);
        step_axis(&mut pz, &mut vz);
        step(&mut p, &mut v);

        assert_eq!(separate_axis(&p, &v, 0), (px, vx));
        assert_eq!(separate_axis(&p, &v, 1), (py, vy));
        assert_eq!(separate_axis(&p, &v, 2), (pz, vz));
    }

    #[test]
    fn repeat_per_axis() {

        let test_axis = |(p, v): (Vec<Coord3>, Vec<Coord3>), axis| {
            let (pa, va) = separate_axis(&p, &v, axis);
            let sa = find_repeat_in_axis(&pa, &va);
    
            let mut pt = p.clone();
            let mut vt = v.clone();
            for _ in 0..sa {
                step(&mut pt, &mut vt);
            }
            assert_eq!(separate_axis(&pt, &vt, axis), (pa, va));
        };

        test_axis(input1(), 0);
        test_axis(input1(), 1);
        test_axis(input1(), 2);

        test_axis(input2(), 0);
        test_axis(input2(), 1);
        test_axis(input2(), 2);
    }

    #[test]
    fn repeat() {
        let (p, v) = input1();
        assert_eq!(find_repeat(&p, &v), 2772);

        let (p, v) = input2();
        assert_eq!(find_repeat(&p, &v), 4686774924);
    }
}
