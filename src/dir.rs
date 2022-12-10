use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "U" => Ok(Self::North),
            "R" => Ok(Self::East),
            "D" => Ok(Self::South),
            "L" => Ok(Self::West),
            _ => Err(anyhow::format_err!("Could not parse '{s}' as a direction")),
        }
    }
}

impl Dir {
    /// Returns (i,j) where:
    /// 0 j →
    /// i
    /// ↓
    pub fn forward(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }
}
