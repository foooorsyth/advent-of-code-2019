use crate::d1_1;

pub fn solve() -> std::io::Result<i32> {
    return d1_1::solve_common(recursive_fuel_required);
}

pub fn recursive_fuel_required(module_mass: &i32) -> i32 {
    let current = module_mass / 3 - 2;
    if current < 0 {
        return 0;
    }
    return current + recursive_fuel_required(&current);
}