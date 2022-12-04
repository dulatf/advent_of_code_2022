pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Solution {
    fn part1(&self) -> Result<()>;
    fn part2(&self) -> Result<()>;
}