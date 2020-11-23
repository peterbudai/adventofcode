use anyhow::{ensure, Result};
use itertools::Itertools;
use std::collections::HashMap;

type Ingredient = (usize, String);
type Recipe = (usize, Vec<Ingredient>);

fn parse_ingredient(data: &str) -> Result<Ingredient> {
    let parts = data.trim().split_whitespace().collect_vec();
    ensure!(parts.len() == 2, "ingredient has an amount and material");
    Ok((parts[0].parse::<usize>()?, parts[1].to_owned()))
}

fn parse_recipe(data: &str) -> Result<(String, Recipe)> {
    let parts = data.split("=>").collect_vec();
    ensure!(parts.len() == 2, "recipe has ingredients and result");
    let (amount, material) = parse_ingredient(parts[1])?;
    let ingredients = parts[0].split(',').map(|i| parse_ingredient(i)).collect::<Result<Vec<_>, _>>()?;
    Ok((material, (amount, ingredients)))
}

fn parse_recipes(data: &str) -> Result<HashMap<String, Recipe>> {
    let mut recipes = data.lines().map(|l| parse_recipe(l)).collect::<Result<Vec<_>, _>>()?;
    Ok(recipes.drain(..).fold(HashMap::<String, Recipe>::new(), |mut h, (m, r)| { h.insert(m, r); h }))
}

fn produce_fuel(recipes: &HashMap<String, Recipe>, surplus: &mut HashMap<String, usize>, amount: usize) -> usize {
    let mut needed = HashMap::<String, usize>::new();
    for material in recipes.keys() {
        needed.insert(material.to_owned(), 0);
    }
    needed.insert("ORE".to_owned(), 0);
    *needed.get_mut("FUEL").unwrap() = amount;

    let mut ores = 0;
    loop {
        let need_material: String;
        let need_amount: usize;
        if let Some((nm, na)) = needed.iter().filter(|(_, v)| **v > 0).next() {
            need_material = nm.to_owned();
            need_amount = *na;
        } else {
            break;
        }

        let have_amount = *surplus.get(&need_material).unwrap();
        let required_amount = need_amount.saturating_sub(have_amount);

        let produced_amount = if need_material == "ORE" {
            ores += required_amount;
            required_amount
        } else {
            let (recipe_amount, ingredients) = recipes.get(&need_material).unwrap();

            let multiplier = (required_amount + *recipe_amount - 1) / *recipe_amount;

            for (ing_amount, ing_material) in ingredients {
                *needed.get_mut(ing_material).unwrap() += ing_amount * multiplier;
            }
            *recipe_amount * multiplier
        };

        *needed.get_mut(&need_material).unwrap() = 0;
        *surplus.get_mut(&need_material).unwrap() = have_amount + produced_amount - need_amount;
    }
    ores
}

fn calc_ore_per_fuel(recipes: &HashMap<String, Recipe>) -> usize {
    let mut surplus = HashMap::<String, usize>::new();
    for material in recipes.keys() {
        surplus.insert(material.to_owned(), 0);
    }
    surplus.insert("ORE".to_owned(), 0);
    
    produce_fuel(recipes, &mut surplus, 1)
}

fn calc_fuel_produced(recipes: &HashMap<String, Recipe>) -> usize {
    let mut surplus = HashMap::<String, usize>::new();
    for material in recipes.keys() {
        surplus.insert(material.to_owned(), 0);
    }
    surplus.insert("ORE".to_owned(), 0);
    
    let target = 1000000000000usize;
    let mut used = 0usize;
    
    let mut amount = 1;
    let mut fuel = 0;
    while amount > 0 {
        let mut s = surplus.clone();
        let ores = produce_fuel(recipes, &mut s, amount);
        if used + ores > target {
            amount /= 2;
            continue;
        }
        fuel += amount;
        used += ores;

        if used < target / 2 {
            amount *= 2;
        }
        surplus = s;
    }
    fuel
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let recipes = parse_recipes(data)?;
    Ok((calc_ore_per_fuel(&recipes), calc_fuel_produced(&recipes)))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse() {
        assert_eq!(parse_ingredient("10 ORE").unwrap(), (10, "ORE".into()));
        assert_eq!(parse_ingredient(" 10 A ").unwrap(), (10, "A".into()));
        assert_eq!(parse_ingredient("1   B").unwrap(), (1, "B".into()));
        assert_eq!(parse_ingredient("7 A  ").unwrap(), (7, "A".into()));
        assert_eq!(parse_ingredient("  1 C").unwrap(), (1, "C".into()));

        assert_eq!(parse_recipe("10 ORE => 10 A").unwrap(), ("A".into(), (10, vec![(10, "ORE".into())])));
        assert_eq!(parse_recipe("1 ORE => 1 B").unwrap(), ("B".into(), (1, vec![(1, "ORE".into())])));
        assert_eq!(parse_recipe("7 A, 1 B => 1 C").unwrap(), ("C".into(), (1, vec![(7, "A".into()), (1, "B".into())])));
        assert_eq!(parse_recipe("7 A, 1 C => 1 D").unwrap(), ("D".into(), (1, vec![(7, "A".into()), (1, "C".into())])));
        assert_eq!(parse_recipe("7 A, 1 D => 1 E").unwrap(), ("E".into(), (1, vec![(7, "A".into()), (1, "D".into())])));
        assert_eq!(parse_recipe("7 A, 1 E => 1 FUEL").unwrap(), ("FUEL".into(), (1, vec![(7, "A".into()), (1, "E".into())])));

        let recipes = parse_recipes(indoc!(
            "10 ORE => 10 A
             1 ORE => 1 B
             7 A, 1 B => 1 C
             7 A, 1 C => 1 D
             7 A, 1 D => 1 E
             7 A, 1 E => 1 FUEL"
        )).unwrap();

        assert_eq!(recipes.len(), 6);
        assert_eq!(recipes.get("A").unwrap(), &(10, vec![(10, "ORE".into())]));
        assert_eq!(recipes.get("B").unwrap(), &(1, vec![(1, "ORE".into())]));
        assert_eq!(recipes.get("C").unwrap(), &(1, vec![(7, "A".into()), (1, "B".into())]));
        assert_eq!(recipes.get("D").unwrap(), &(1, vec![(7, "A".into()), (1, "C".into())]));
        assert_eq!(recipes.get("E").unwrap(), &(1, vec![(7, "A".into()), (1, "D".into())]));
        assert_eq!(recipes.get("FUEL").unwrap(), &(1, vec![(7, "A".into()), (1, "E".into())]));
    }

    #[test]
    fn ore_needed_for_one_fuel() {
        let recipes = parse_recipes(indoc!(
            "10 ORE => 10 A
             1 ORE => 1 B
             7 A, 1 B => 1 C
             7 A, 1 C => 1 D
             7 A, 1 D => 1 E
             7 A, 1 E => 1 FUEL"
        )).unwrap();
        assert_eq!(calc_ore_per_fuel(&recipes), 31);

        let recipes = parse_recipes(indoc!(
            "9 ORE => 2 A
             8 ORE => 3 B
             7 ORE => 5 C
             3 A, 4 B => 1 AB
             5 B, 7 C => 1 BC
             4 C, 1 A => 1 CA
             2 AB, 3 BC, 4 CA => 1 FUEL"
        )).unwrap();
        assert_eq!(calc_ore_per_fuel(&recipes), 165);

        let recipes = parse_recipes(indoc!(
            "157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        )).unwrap();
        assert_eq!(calc_ore_per_fuel(&recipes), 13312);
        
        let recipes = parse_recipes(indoc!(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
             17 NVRVD, 3 JNWZP => 8 VPVL
             53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
             22 VJHF, 37 MNCFX => 5 FWMGM
             139 ORE => 4 NVRVD
             144 ORE => 7 JNWZP
             5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
             5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
             145 ORE => 6 MNCFX
             1 NVRVD => 8 CXFTF
             1 VJHF, 6 MNCFX => 4 RFSQX
             176 ORE => 6 VJHF"
        )).unwrap();
        assert_eq!(calc_ore_per_fuel(&recipes), 180697);
            
        let recipes = parse_recipes(indoc!(
            "171 ORE => 8 CNZTR
             7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
             114 ORE => 4 BHXH
             14 VRPVC => 6 BMBT
             6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
             6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
             15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
             13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
             5 BMBT => 4 WPTQ
             189 ORE => 9 KTJDG
             1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
             12 VRPVC, 27 CNZTR => 2 XDBXC
             15 KTJDG, 12 BHXH => 5 XCVML
             3 BHXH, 2 VRPVC => 7 MZWV
             121 ORE => 7 VRPVC
             7 XCVML => 6 RJRHP
             5 BHXH, 4 VRPVC => 5 LTCX"
        )).unwrap();
        assert_eq!(calc_ore_per_fuel(&recipes), 2210736);
    }

    #[test]
    fn fuel_produces_from_fixed_ore() {
        let recipes = parse_recipes(indoc!(
            "157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        )).unwrap();
        assert_eq!(calc_fuel_produced(&recipes), 82892753);
        
        let recipes = parse_recipes(indoc!(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
             17 NVRVD, 3 JNWZP => 8 VPVL
             53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
             22 VJHF, 37 MNCFX => 5 FWMGM
             139 ORE => 4 NVRVD
             144 ORE => 7 JNWZP
             5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
             5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
             145 ORE => 6 MNCFX
             1 NVRVD => 8 CXFTF
             1 VJHF, 6 MNCFX => 4 RFSQX
             176 ORE => 6 VJHF"
        )).unwrap();
        assert_eq!(calc_fuel_produced(&recipes), 5586022);
            
        let recipes = parse_recipes(indoc!(
            "171 ORE => 8 CNZTR
             7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
             114 ORE => 4 BHXH
             14 VRPVC => 6 BMBT
             6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
             6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
             15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
             13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
             5 BMBT => 4 WPTQ
             189 ORE => 9 KTJDG
             1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
             12 VRPVC, 27 CNZTR => 2 XDBXC
             15 KTJDG, 12 BHXH => 5 XCVML
             3 BHXH, 2 VRPVC => 7 MZWV
             121 ORE => 7 VRPVC
             7 XCVML => 6 RJRHP
             5 BHXH, 4 VRPVC => 5 LTCX"
        )).unwrap();
        assert_eq!(calc_fuel_produced(&recipes), 460664);
    }
}