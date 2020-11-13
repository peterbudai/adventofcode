use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }

    pub fn central() -> Self {
        Pos { x: 0, y: 0 }
    }

    pub fn dist(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Dir {
    fn parse(s: &str) -> Result<Self> {
        anyhow::ensure!(s.len() > 0, "Empty trace");

        let dist = s[1..].parse::<usize>()?;
        anyhow::ensure!(dist > 0, "Invalid distance");

        match s.chars().nth(0) {
            Some('U') => Ok(Dir::Up(dist)),
            Some('D') => Ok(Dir::Down(dist)),
            Some('L') => Ok(Dir::Left(dist)),
            Some('R') => Ok(Dir::Right(dist)),
            _ => anyhow::bail!("Invalid direction")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Path(Vec<Dir>);

impl Path {
    pub fn parse(s: &str) -> Result<Self> {
        s.split(',').filter_map(|s| if s.is_empty() { None } else { Some(Dir::parse(s))}).collect::<Result<Vec<_>, _>>().map(|v| Path(v))
    }

    pub fn trace(&self) -> Vec<Pos> {
        let mut trace = Vec::<Pos>::new();
        trace.push(Pos::central());

        for dir in &self.0 {
            let (d, dx, dy) = match dir {
                Dir::Up(d) => (d, 0isize, 1isize),
                Dir::Down(d) => (d, 0isize, -1isize),
                Dir::Left(d) => (d, -1isize, 0isize),
                Dir::Right(d) => (d, 1isize, 0isize),
            };
            let last = *trace.last().unwrap();
            for i in 1..(*d as isize)+1 {
                trace.push(Pos::new(last.x + dx * i, last.y + dy * i))
            }
        }
        trace
    }
}

fn nearest_crossing(path1: &Path, path2: &Path) -> (usize, usize) {
    let trace1 = path1.trace();
    let trace2 = path2.trace();

    let mut dist = 0usize;
    let mut step = 0usize;
    for (i, l) in trace1.iter().enumerate() {
        for (j, k) in trace2.iter().enumerate() {
            if l == k && l.dist() > 0 {
                if dist == 0 || dist > l.dist() {
                    dist = l.dist();
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
    let paths = data.lines().map(|s| Path::parse(s)).collect::<Result<Vec<_>, _>>()?;
    Ok(nearest_crossing(&paths[0], &paths[1]))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn distance() {
        assert_eq!(Pos::central().dist(), 0);
        assert_eq!(Pos::new(1, 0).dist(), 1);
        assert_eq!(Pos::new(0, -1).dist(), 1);
        assert_eq!(Pos::new(1, 1).dist(), 2);
        assert_eq!(Pos::new(-2, 1).dist(), 3);
    }
    
    #[test]
    fn parse_dir() {
        assert_eq!(Dir::parse("U20").unwrap(), Dir::Up(20));
        assert_eq!(Dir::parse("D2").unwrap(), Dir::Down(2));
        assert_eq!(Dir::parse("L1").unwrap(), Dir::Left(1));
        assert_eq!(Dir::parse("R3333").unwrap(), Dir::Right(3333));
    
        assert!(Dir::parse("X3").is_err());
        assert!(Dir::parse("U0").is_err());
        assert!(Dir::parse("R-3").is_err());
    }

    #[test]
    fn path_parse() {
        assert_eq!(Path::parse("").unwrap(), Path(vec![]));
        assert_eq!(Path::parse("L2").unwrap(), Path(vec![Dir::Left(2)]));
        assert_eq!(Path::parse("L2,U4").unwrap(), Path(vec![Dir::Left(2), Dir::Up(4)]));
        assert_eq!(Path::parse("L2,U4,").unwrap(), Path(vec![Dir::Left(2), Dir::Up(4)]));
    
        assert!(Path::parse("X2").is_err());
        assert!(Path::parse("L2;U4").is_err());
    }
    
    #[test]
    fn path_trace() {
        let mut path = Path(vec![Dir::Right(2000)]);
        assert!(path.trace().iter().enumerate().all(|(i, l)| l.x == i as isize && l.y == 0));
        path = Path(vec![Dir::Up(1), Dir::Right(2), Dir::Down(3), Dir::Left(4)]);
        assert_eq!(path.trace(), vec![Pos::central(), Pos::new(0,1), Pos::new(1,1),Pos::new(2,1),Pos::new(2,0),Pos::new(2,-1),Pos::new(2,-2),Pos::new(1,-2),Pos::new(0,-2),Pos::new(-1,-2),Pos::new(-2,-2)]);
    }    

    #[test]
    fn path_crossing() {
        assert_eq!(nearest_crossing(
            &Path::parse("R8,U5,L5,D3").unwrap(),
            &Path::parse("U7,R6,D4,L4").unwrap()
        ), (6, 30));
        assert_eq!(nearest_crossing(
            &Path::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap(),
            &Path::parse("U62,R66,U55,R34,D71,R55,D58,R83").unwrap()
        ), (159, 610));
        assert_eq!(nearest_crossing(
            &Path::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
            &Path::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()
        ), (135, 410));
    }
}
