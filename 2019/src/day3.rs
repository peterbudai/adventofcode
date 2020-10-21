use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Location {
    pub x: isize,
    pub y: isize,
}

impl Location {
    fn central() -> Location {
        Location { x: 0, y: 0 }
    }

    fn distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[cfg(test)]
#[test]
fn test_location_distance() {
    assert_eq!(Location::central().distance(), 0);
    assert_eq!(Location { x: 1, y: 0 }.distance(), 1);
    assert_eq!(Location { x: 0, y: -1 }.distance(), 1);
    assert_eq!(Location { x: 1, y: 1 }.distance(), 2);
    assert_eq!(Location { x: -2, y: 1 }.distance(), 3);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Trace {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl Trace {
    fn parse(s: &str) -> Result<Self> {
        anyhow::ensure!(s.len() > 0, "Empty trace");

        let distance = s[1..].parse::<isize>()?;
        anyhow::ensure!(distance > 0, "Invalid distance");

        match s.chars().nth(0) {
            Some('U') => Ok(Trace::Up(distance)),
            Some('D') => Ok(Trace::Down(distance)),
            Some('L') => Ok(Trace::Left(distance)),
            Some('R') => Ok(Trace::Right(distance)),
            _ => anyhow::bail!("Invalid direction")
        }
    }
}

#[cfg(test)]
#[test]
fn test_trace_parse() {
    assert_eq!(Trace::parse("U20").unwrap(), Trace::Up(20));
    assert_eq!(Trace::parse("D2").unwrap(), Trace::Down(2));
    assert_eq!(Trace::parse("L1").unwrap(), Trace::Left(1));
    assert_eq!(Trace::parse("R3333").unwrap(), Trace::Right(3333));

    assert!(Trace::parse("X3").is_err());
    assert!(Trace::parse("U0").is_err());
    assert!(Trace::parse("R-3").is_err());
}

#[derive(Debug, Clone, PartialEq)]
struct Path(Vec<Trace>);

impl Path {
    fn parse(s: &str) -> Result<Self> {
        s.split(',').filter_map(|s| if s.is_empty() { None } else { Some(Trace::parse(s))}).collect::<Result<Vec<_>, _>>().map(|v| Path(v))
    }
}

impl IntoIterator for Path {
    type Item = Location;
    type IntoIter = PathIterator;

    fn into_iter(self) -> Self::IntoIter {
        PathIterator {
            location: None,
            path: self.0,
        }
    }
}

#[cfg(test)]
#[test]
fn test_path_parse() {
    assert_eq!(Path::parse("").unwrap(), Path(vec![]));
    assert_eq!(Path::parse("L2").unwrap(), Path(vec![Trace::Left(2)]));
    assert_eq!(Path::parse("L2,U4").unwrap(), Path(vec![Trace::Left(2),Trace::Up(4)]));
    assert_eq!(Path::parse("L2,U4,").unwrap(), Path(vec![Trace::Left(2),Trace::Up(4)]));

    assert!(Path::parse("X2").is_err());
    assert!(Path::parse("L2;U4").is_err());
}

struct PathIterator {
    location: Option<Location>,
    path: Vec<Trace>,
}

impl Iterator for PathIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(loc) = self.location {
            if let Some(trace) = self.path.first_mut() {
                let (nloc, ntrace) = match trace {
                    Trace::Up(1) => (Location {x: loc.x, y: loc.y + 1}, None),
                    Trace::Up(d) => (Location {x: loc.x, y: loc.y + 1}, Some(Trace::Up(*d - 1))),
                    Trace::Down(1) => (Location {x: loc.x, y: loc.y - 1}, None),
                    Trace::Down(d) => (Location {x: loc.x, y: loc.y - 1}, Some(Trace::Down(*d - 1))),
                    Trace::Left(1) => (Location {x: loc.x - 1, y: loc.y}, None),
                    Trace::Left(d) => (Location {x: loc.x - 1, y: loc.y}, Some(Trace::Left(*d - 1))),
                    Trace::Right(1) => (Location {x: loc.x + 1, y: loc.y}, None),
                    Trace::Right(d) => (Location {x: loc.x + 1, y: loc.y}, Some(Trace::Right(*d - 1))),
                };

                self.location = Some(nloc);
                if let Some(nntrace) = ntrace {
                    *trace = nntrace;
                } else {
                    self.path.remove(0);
                }
            } else {
                return None;
            }
        } else {
            self.location = Some(Location::central());
        }
        self.location
    }
}

#[cfg(test)]
#[test]
fn test_path_iterator() {
    let path = Path(vec![Trace::Left(2000)]);
    assert!(path.into_iter().all(|l| l.distance() <= 2000));
}

#[derive(Debug)]
struct Crossing {
    pub distance: isize,
    pub steps: usize,
}

pub fn solution(data: &str) -> Result<(isize, usize)> {
    let mut lines = data.lines().map(|s| Path::parse(s)).collect::<Result<Vec<_>, _>>()?;

    let loc1 = lines.remove(0).into_iter().collect::<Vec<Location>>();
    let loc2 = lines.remove(0).into_iter().collect::<Vec<Location>>();

    let mut cross = Vec::<Crossing>::new();
    for (i, l) in loc1.iter().enumerate() {
        for (j, k) in loc2.iter().enumerate() {
            if l == k && l.distance() > 0 {
                cross.push(Crossing {distance: l.distance(), steps: i + j});
            }
        }
    }
    Ok((cross.iter().map(|c| c.distance).min().unwrap(), cross.iter().map(|c| c.steps).min().unwrap()))
}
