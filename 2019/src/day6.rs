use anyhow::Result;
use std::collections::HashMap;

fn parse_map(data: &str) -> HashMap<&str, &str> {
    data.lines().fold(HashMap::<&str, &str>::new(), |mut h, line| { 
        let parts = line.split(')').collect::<Vec<&str>>();
        h.insert(parts[1], parts[0]);
        h
    })
}

fn route_to_com<'a>(orbits: &'a HashMap::<&str, &str>, start: &'a str) -> Vec<&'a str> {
    let mut route = Vec::<&str>::new();
    while let Some(c) = orbits.get(route.last().unwrap_or(&start)) {
        route.push(c);
    }
    route
}

fn route_between<'a>(orbits: &'a HashMap::<&str, &str>, start: &'a str, end: &'a str) -> Vec<&'a str> {
    let mut route1 = route_to_com(orbits, start);
    let mut route2 = route_to_com(orbits, end);

    // On the same path -> keep the separate part
    if route1.len() > route2.len() && route1.ends_with(&route2) {
        route1.truncate(route1.len()-(route2.len()+1));
        route2.clear();
    } else if route2.len() > route1.len() && route2.ends_with(&route1) {
        route2.truncate(route2.len()-(route1.len()+1));
        route1.clear();
    } else {
        // On divergent paths -> keep separate parts and first common point
        let l = route2.len();
        for n in 0..l {
            if route1.ends_with(&route2[n..l]) {
                route2.truncate(n);
                route1.truncate(route1.len()-(l-n-1));
                break;
            }
        }
    }
    route2.reverse();
    route1.append(&mut route2);
    route1
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let orbits = parse_map(data);
    Ok((orbits.keys().map(|k| route_to_com(&orbits, k).len()).sum(), route_between(&orbits, "YOU", "SAN").len()-1))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn to_com() {
        //         G - H       J - K - L
        //        /           /
        // COM - B - C - D - E - F
        //                \
        //                 I
        let map = parse_map(indoc! {"COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L"});

        assert_eq!(route_to_com(&map, "D"), &["C", "B", "COM"]);
        assert_eq!(route_to_com(&map, "D").len(), 3);
        assert_eq!(route_to_com(&map, "L"), &["K", "J", "E", "D", "C", "B", "COM"]);
        assert_eq!(route_to_com(&map, "L").len(), 7);
        assert!(route_to_com(&map, "COM").is_empty());
    }

    #[test]
    fn between() {
        //                           YOU
        //                          /
        //         G - H       J - K - L
        //        /           /
        // COM - B - C - D - E - F
        //                \
        //                 I - SAN
        let map = parse_map(indoc! {"COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            K)YOU
            I)SAN"});
    
        assert_eq!(route_between(&map, "YOU", "SAN"), &["K", "J", "E", "D", "I"]);
        assert_eq!(route_between(&map, "SAN", "YOU"), &["I", "D", "E", "J", "K"]);
        assert_eq!(route_between(&map, "E","I"), &["D"]);
        assert_eq!(route_between(&map, "I","E"), &["D"]);
    
        assert_eq!(route_between(&map, "B","H"), &["G"]);
        assert_eq!(route_between(&map, "F","C"), &["E", "D"]);
        assert_eq!(route_between(&map, "C","F"), &["D", "E"]);
    }
}
