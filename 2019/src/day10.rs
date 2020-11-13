use anyhow::{ensure, Result};
use approx::abs_diff_eq;
use itertools::Itertools;
use std::cmp::Ordering;
use std::f64::consts::{PI, FRAC_PI_2};
use crate::util::Coord;

fn coord_to_num(coord: &Coord) -> Result<usize> {
    ensure!(coord.0 >= 0 && coord.1 >= 0, "Only works with unsigned coordinates");
    Ok(coord.0 as usize * 100 + coord.1 as usize)
}

fn parse_map(data: &str) -> Vec<Coord> {
    data.lines().enumerate()
        .map(|(y, l)| l.chars().enumerate()
            .map(move |(x, c)| (x as isize, y as isize, c == '#'))
        )
        .flatten()
        .filter(|(_, _, a)| *a)
        .map(|(x, y, _)| (x, y))
        .collect()
}

type Distance = (f64, f64);

fn distance((x1, y1): &Coord, (x2, y2): &Coord) -> Distance {
    let dx = *x2 as f64 - *x1 as f64;
    let dy = *y1 as f64 - *y2 as f64;

    ((dx.powi(2) + dy.powi(2)).sqrt(), dy.atan2(dx))
}

fn direct_sight(coords: &[Coord], from: &Coord) -> Vec<(Coord, Distance)> {
    coords.iter()
        .filter(|(x,y)| !abs_diff_eq!(*x, from.0) || !abs_diff_eq!(*y, from.1))
        .map(|c| (*c, distance(from, c)))
        .fold(Vec::<(Coord, Distance)>::new(), |mut v, (c1, (d1, a1))| {
            if let Some((i, (_, (d2, _)))) = v.iter().find_position(|(_, (_, a2))| abs_diff_eq!(a1, a2)) {
                if *d2 > d1 {
                    v[i].0 = c1;
                    v[i].1.0 = d1;
                }
            } else {
                v.push((c1, (d1, a1)));
            }
            v
        })
}

fn most_direct_sight(coords: &[Coord]) -> (Coord, usize) {
    coords.iter()
        .map(|c| (*c, direct_sight(coords, c).len()))
        .max_by_key(|(_, n)| *n).unwrap()
}

fn vaporize_targets(coords: &[Coord], from: &Coord) -> Vec<Coord> {
    let mut targets = direct_sight(coords, from)
        .into_iter()
        .map(|(c, (_, a))| (c, (a + PI + FRAC_PI_2) % (2f64 * PI)))
        .sorted_by(|(_, a1), (_, a2)| if a1 > a2 { Ordering::Less } else if a1 < a2 { Ordering::Greater } else { Ordering::Equal })
        .collect::<Vec<(Coord, f64)>>();

    if abs_diff_eq!(targets.last().unwrap().1, 0f64) {
        targets.rotate_right(1);
    }
    targets.into_iter().map(|(c, _)| c).collect()
}

fn vaporize_order(coords: &mut Vec<Coord>, from: &Coord) -> Vec<Coord> {
    let mut order = Vec::<Coord>::new();

    coords.retain(|c| c != from);
    while !coords.is_empty() {
        let mut targets = vaporize_targets(coords, from);
        coords.retain(|c| !targets.contains(c));
        order.append(&mut targets);
    }
    order
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let mut asteroids = parse_map(data);
    let (station_coords, visible_asteroids) = most_direct_sight(&asteroids);
    let vaporized_coords = vaporize_order(&mut asteroids, &station_coords);
    Ok((visible_asteroids, coord_to_num(&vaporized_coords[199])?))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4, SQRT_2};

    #[test]
    fn coord_num() {
        assert_eq!(coord_to_num(&(0, 0)).unwrap(), 0);
        assert_eq!(coord_to_num(&(0, 1)).unwrap(), 1);
        assert_eq!(coord_to_num(&(8, 2)).unwrap(), 802);
        assert_eq!(coord_to_num(&(8, 0)).unwrap(), 800);

        assert!(coord_to_num(&(-1, 0)).is_err());
        assert!(coord_to_num(&(0, -1)).is_err());
        assert!(coord_to_num(&(-1, -1)).is_err());
    }

    #[test]
    fn parse() {
        let map = parse_map(indoc!(".#..#
                                          .....
                                          #####
                                          ....#
                                          ...##"));
        assert_eq!(map.len(), 10);
        assert!(map.contains(&(1, 0)));
        assert!(map.contains(&(4, 0)));
        assert!(map.contains(&(0, 2)));
        assert!(map.contains(&(1, 2)));
        assert!(map.contains(&(2, 2)));
        assert!(map.contains(&(3, 2)));
        assert!(map.contains(&(4, 2)));
        assert!(map.contains(&(4, 3)));
        assert!(map.contains(&(3, 4)));
        assert!(map.contains(&(4, 4)));
    }

    #[test]
    fn coord_distance() {
        let (d, a) = distance(&(0, 0), &(1, 0));
        assert_abs_diff_eq!(d, 1f64);
        assert_abs_diff_eq!(a, 0f64);
    
        let (d, a) = distance(&(0, 0), &(0, 2));
        assert_abs_diff_eq!(d, 2f64);
        assert_abs_diff_eq!(a, -FRAC_PI_2);
    
        let (d, a) = distance(&(2, 0), &(0, 0));
        assert_abs_diff_eq!(d, 2f64);
        assert_abs_diff_eq!(a, PI);
    
        let (d, a) = distance(&(0, 1), &(0, 0));
        assert_abs_diff_eq!(d, 1f64);
        assert_abs_diff_eq!(a, FRAC_PI_2);
    
        let (d, a) = distance(&(0, 0), &(1, 1));
        assert_abs_diff_eq!(d, SQRT_2);
        assert_abs_diff_eq!(a, -FRAC_PI_4);
    
        let (d, a) = distance(&(0, 1), &(1, 0));
        assert_abs_diff_eq!(d, SQRT_2);
        assert_abs_diff_eq!(a, FRAC_PI_4);
    }

    #[test]
    fn direct() {
        let map = parse_map(indoc!(
            ".#..#
             .....
             #####
             ....#
             ...##"));
    
        assert_eq!(direct_sight(&map, &(1, 0)).len(), 7);
        assert_eq!(direct_sight(&map, &(4, 0)).len(), 7);
        assert_eq!(direct_sight(&map, &(0, 2)).len(), 6);
        assert_eq!(direct_sight(&map, &(1, 2)).len(), 7);
        assert_eq!(direct_sight(&map, &(2, 2)).len(), 7);
        assert_eq!(direct_sight(&map, &(3, 2)).len(), 7);
        assert_eq!(direct_sight(&map, &(4, 2)).len(), 5);
        assert_eq!(direct_sight(&map, &(4, 3)).len(), 7);
        assert_eq!(direct_sight(&map, &(3, 4)).len(), 8);
        assert_eq!(direct_sight(&map, &(4, 4)).len(), 7);
    }


    #[test]
    fn most_direct_count() {
        let mds = most_direct_sight(parse_map(indoc!(
            ".#..#
             .....
             #####
             ....#
             ...##"
        )).as_slice());
        assert_eq!(mds, ((3,4), 8));
    
        let mds = most_direct_sight(parse_map(indoc!(
            "......#.#.
             #..#.#....
             ..#######.
             .#.#.###..
             .#..#.....
             ..#....#.#
             #..#....#.
             .##.#..###
             ##...#..#.
             .#....####"
        )).as_slice());
        assert_eq!(mds, ((5,8), 33));
    
        let mds = most_direct_sight(parse_map(indoc!(
            "#.#...#.#.
             .###....#.
             .#....#...
             ##.#.#.#.#
             ....#.#.#.
             .##..###.#
             ..#...##..
             ..##....##
             ......#...
             .####.###."
        )).as_slice());
        assert_eq!(mds, ((1,2), 35));
    
        let mds = most_direct_sight(parse_map(indoc!(
            ".#..#..###
             ####.###.#
             ....###.#.
             ..###.##.#
             ##.##.#.#.
             ....###..#
             ..#.#..#.#
             #..#.#.###
             .##...##.#
             .....#.#.."
        )).as_slice());
        assert_eq!(mds, ((6,3), 41));
    
        let mds = most_direct_sight(parse_map(indoc!(
            ".#..##.###...#######
             ##.############..##.
             .#.######.########.#
             .###.#######.####.#.
             #####.##.#.##.###.##
             ..#####..#.#########
             ####################
             #.####....###.#.#.##
             ##.#################
             #####.##.###..####..
             ..######..##.#######
             ####.##.####...##..#
             .#####..#.######.###
             ##...#.##########...
             #.##########.#######
             .####.#.###.###.#.##
             ....##.##.###..#####
             .#.#.###########.###
             #.#.#.#####.####.###
             ###.##.####.##.#..##"
        )).as_slice());
        assert_eq!(mds, ((11,13), 210));
    }

    #[test]
    fn targets() {
        let t = vaporize_targets(parse_map(indoc!(
            ".#....#####...#..
             ##...##.#####..##
             ##...#...#.#####.
             ..#.....X...###..
             ..#.#.....#....##"
        )).as_slice(), &(8, 3));
        assert_eq!(&t[0..5], &[(8, 1), (9, 0), (9, 1), (10, 0), (9, 2)]);
    }
    #[test]
    fn target_order() {
        let o = vaporize_order(&mut parse_map(indoc!(
            ".#....#####...#..
             ##...##.#####..##
             ##...#...#.#####.
             ..#.....X...###..
             ..#.#.....#....##"
        )), &(8, 3));
        assert_eq!(&o, &[(8, 1), (9, 0), (9, 1), (10, 0), (9, 2), (11, 1), (12, 1), (11, 2), (15, 1), (12, 2), (13, 2), (14, 2), (15, 2), (12, 3), (16, 4), (15, 4), (10, 4), (4, 4), (2, 4), (2, 3), (0, 2), (1, 2), (0, 1), (1, 1), (5, 2), (1, 0), (5, 1), (6, 1), (6, 0), (7, 0), (8, 0), (10, 1), (14, 0), (16, 1), (13, 3), (14, 3)]);
    
        let o = vaporize_order(&mut parse_map(indoc!(
            ".#..##.###...#######
             ##.############..##.
             .#.######.########.#
             .###.#######.####.#.
             #####.##.#.##.###.##
             ..#####..#.#########
             ####################
             #.####....###.#.#.##
             ##.#################
             #####.##.###..####..
             ..######..##.#######
             ####.##.####...##..#
             .#####..#.######.###
             ##...#.##########...
             #.##########.#######
             .####.#.###.###.#.##
             ....##.##.###..#####
             .#.#.###########.###
             #.#.#.#####.####.###
             ###.##.####.##.#..##"
        )), &(11, 13));
        assert_eq!(o[0], (11, 12));
        assert_eq!(o[1], (12, 1));
        assert_eq!(o[2], (12, 2));
        assert_eq!(o[9], (12, 8));
        assert_eq!(o[19], (16, 0));
        assert_eq!(o[49], (16, 9));
        assert_eq!(o[99], (10, 16));
        assert_eq!(o[198], (9, 6));
        assert_eq!(o[199], (8, 2));
        assert_eq!(o[200], (10, 9));
        assert_eq!(o[298], (11, 1));
        assert_eq!(o.len(), 299);
    }
}
