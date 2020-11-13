use anyhow::Result;
use crate::util::{Coord, Dir};

fn manhattan_distance((x, y): &Coord) -> usize {
    x.abs() as usize + y.abs() as usize
}

type Step = (Dir, usize);

fn parse_step(s: &str) -> Result<Step> {
    anyhow::ensure!(s.len() > 0, "Empty trace");

    let dist = s[1..].parse::<usize>()?;
    anyhow::ensure!(dist > 0, "Invalid distance");

    let dir = match s.chars().nth(0) {
        Some('U') => Dir::Up,
        Some('D') => Dir::Down,
        Some('L') => Dir::Left,
        Some('R') => Dir::Right,
        _ => anyhow::bail!("Invalid direction"),
    };

    Ok((dir, dist))
}

type Path = Vec<Step>;

fn parse_path(s: &str) -> Result<Path> {
    s.split(',').filter_map(|s| if s.is_empty() { None } else { Some(parse_step(s))}).collect::<Result<Vec<_>, _>>()
}

fn walk_path(path: &Path) -> Vec<Coord> {
    let mut trace = Vec::<Coord>::new();
    trace.push((0, 0));

    for (dir, dist) in path {
        let (dx, dy) = dir.delta();
        let (lastx, lasty) = *trace.last().unwrap();
        for i in 1..(*dist as isize)+1 {
            trace.push((lastx + dx * i, lasty + dy * i))
        }
    }
    trace
}

fn nearest_crossing(path1: &Path, path2: &Path) -> (usize, usize) {
    let trace1 = walk_path(path1);
    let trace2 = walk_path(path2);

    let mut dist = 0usize;
    let mut step = 0usize;
    for (i, l) in trace1.iter().enumerate() {
        for (j, k) in trace2.iter().enumerate() {
            if l == k && manhattan_distance(l) > 0 {
                if dist == 0 || dist > manhattan_distance(l) {
                    dist = manhattan_distance(l);
                }
                if step == 0 || step > i + j {
                    step = i + j;
                }
            }
        }
    }
    (dist, step)
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let paths = data.lines().map(|s| parse_path(s)).collect::<Result<Vec<_>, _>>()?;
    Ok(nearest_crossing(&paths[0], &paths[1]))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn distance() {
        assert_eq!(manhattan_distance(&(0, 0)), 0);
        assert_eq!(manhattan_distance(&(1, 0)), 1);
        assert_eq!(manhattan_distance(&(0, -1)), 1);
        assert_eq!(manhattan_distance(&(1, 1)), 2);
        assert_eq!(manhattan_distance(&(-2, 1)), 3);
    }
    
    #[test]
    fn step_parse() {
        assert_eq!(parse_step("U20").unwrap(), (Dir::Up, 20));
        assert_eq!(parse_step("D2").unwrap(), (Dir::Down, 2));
        assert_eq!(parse_step("L1").unwrap(), (Dir::Left, 1));
        assert_eq!(parse_step("R3333").unwrap(), (Dir::Right, 3333));
    
        assert!(parse_step("X3").is_err());
        assert!(parse_step("U0").is_err());
        assert!(parse_step("R-3").is_err());
    }

    #[test]
    fn path_parse() {
        assert_eq!(parse_path("").unwrap(), vec![]);
        assert_eq!(parse_path("L2").unwrap(), vec![(Dir::Left, 2)]);
        assert_eq!(parse_path("L2,U4").unwrap(), vec![(Dir::Left, 2), (Dir::Up, 4)]);
        assert_eq!(parse_path("L2,U4,").unwrap(), vec![(Dir::Left, 2), (Dir::Up, 4)]);
    
        assert!(parse_path("X2").is_err());
        assert!(parse_path("L2;U4").is_err());
    }
    
    #[test]
    fn path_trace() {
        let mut path = vec![(Dir::Right, 2000)];
        assert!(walk_path(&path).iter().enumerate().all(|(i, (x,y))| *x == i as isize && *y == 0));
        path = vec![(Dir::Up, 1), (Dir::Right, 2), (Dir::Down, 3), (Dir::Left, 4)];
        assert_eq!(walk_path(&path), vec![(0, 0), (0,1), (1,1), (2,1), (2,0), (2,-1), (2,-2), (1,-2), (0,-2), (-1,-2), (-2,-2)]);
    }    

    #[test]
    fn path_crossing() {
        assert_eq!(nearest_crossing(
            &parse_path("R8,U5,L5,D3").unwrap(),
            &parse_path("U7,R6,D4,L4").unwrap()
        ), (6, 30));
        assert_eq!(nearest_crossing(
            &parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap(),
            &parse_path("U62,R66,U55,R34,D71,R55,D58,R83").unwrap()
        ), (159, 610));
        assert_eq!(nearest_crossing(
            &parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
            &parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()
        ), (135, 410));
    }
}
