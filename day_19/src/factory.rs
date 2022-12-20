#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

impl Cost {
    pub fn new(ore: u32, clay: u32, obsidian: u32) -> Self {
        Self {
            ore,
            clay,
            obsidian,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Blueprint {
    name: u32,
    cost_ore_robot: Cost,
    cost_clay_robot: Cost,
    cost_obsidian_robot: Cost,
    cost_geode_robot: Cost,
}

impl Blueprint {
    pub fn new(
        name: u32,
        cost_ore_robot: Cost,
        cost_clay_robot: Cost,
        cost_obsidian_robot: Cost,
        cost_geode_robot: Cost,
    ) -> Self {
        Self {
            name,
            cost_ore_robot,
            cost_clay_robot,
            cost_obsidian_robot,
            cost_geode_robot,
        }
    }
}
