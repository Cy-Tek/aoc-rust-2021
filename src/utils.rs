use std::fmt::Debug;
use std::str::FromStr;

pub fn read_input_from_string<T>(s: &str) -> Vec<T>
where T: FromStr
{
    s.lines().flat_map(|s| s.parse::<T>()).collect()
}