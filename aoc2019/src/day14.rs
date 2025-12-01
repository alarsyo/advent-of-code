use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day14.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn parse_recipes(input: &str) -> Result<HashMap<String, Recipe>> {
    let mut recipes = HashMap::new();

    for line in input.lines() {
        let arrow = line
            .find(" => ")
            .with_context(|| format!("couldn't find arrow in line: {}", line))?;

        let elems = &line[..arrow];
        let elems = elems
            .split(", ")
            .map(|elem| {
                let space = elem
                    .find(' ')
                    .with_context(|| format!("couldn't find separator for elem {}", elem))?;
                let amount = elem[..space].parse()?;
                let name = &elem[(space + 1)..];

                Ok(RecipeElem {
                    name: name.to_string(),
                    amount,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let result = &line[(arrow + 4)..].trim_end();
        let space = result
            .find(' ')
            .with_context(|| format!("couldn't find separator for result {}", result))?;
        let result_amount = result[..space].parse()?;
        let result_name = &result[(space + 1)..];

        recipes.insert(
            result_name.to_string(),
            Recipe {
                produced: result_amount,
                elems,
            },
        );
    }

    Ok(recipes)
}

fn get_ore_cost(
    material: String,
    quantity: u64,
    recipes: &HashMap<String, Recipe>,
    inventory: &mut HashMap<String, u64>,
) -> Result<u64> {
    if material == "ORE" {
        return Ok(quantity);
    }

    let mut total = 0;

    let mut in_stock = *inventory.entry(material.clone()).or_default();
    if in_stock < quantity {
        let recipe = recipes
            .get(&material)
            .with_context(|| format!("couldn't find recipe for {}", material))?;

        let needed = quantity - in_stock;
        let num_reactions = needed.div_ceil(recipe.produced);
        for elem in &recipe.elems {
            total += get_ore_cost(
                elem.name.clone(),
                elem.amount * num_reactions,
                recipes,
                inventory,
            )?;
        }
        in_stock += num_reactions * recipe.produced;
    }

    inventory.insert(material, in_stock - quantity);

    Ok(total)
}

fn part1(input: &str) -> Result<u64> {
    let recipes = parse_recipes(input)?;
    let mut inventory = HashMap::new();

    get_ore_cost("FUEL".to_string(), 1, &recipes, &mut inventory)
}

fn part2(input: &str) -> Result<u64> {
    let mut begin: u64 = 0;
    let mut end: u64 = 1_000_000_000_000;

    let recipes = parse_recipes(input)?;

    while begin <= end {
        let mid = begin + (end - begin) / 2;
        let mut inventory = HashMap::new();

        let ore_cost = get_ore_cost("FUEL".to_string(), mid, &recipes, &mut inventory)?;
        match ore_cost.cmp(&1_000_000_000_000) {
            Ordering::Greater => end = mid - 1,
            Ordering::Less => begin = mid + 1,
            Ordering::Equal => return Ok(mid),
        }
    }

    Ok(end)
}

struct RecipeElem {
    name: String,
    amount: u64,
}

struct Recipe {
    produced: u64,
    elems: Vec<RecipeElem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
";

    const PROVIDED2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

    const PROVIDED3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";

    const PROVIDED4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
176 ORE => 6 VJHF
";

    const PROVIDED5: &str = "171 ORE => 8 CNZTR
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
5 BHXH, 4 VRPVC => 5 LTCX
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 31);
        assert_eq!(part1(PROVIDED2).unwrap(), 165);
        assert_eq!(part1(PROVIDED3).unwrap(), 13312);
        assert_eq!(part1(PROVIDED4).unwrap(), 180697);
        assert_eq!(part1(PROVIDED5).unwrap(), 2210736);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 532506);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED3).unwrap(), 82892753);
        assert_eq!(part2(PROVIDED4).unwrap(), 5586022);
        assert_eq!(part2(PROVIDED5).unwrap(), 460664);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2595245);
    }
}
