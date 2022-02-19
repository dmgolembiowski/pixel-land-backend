use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum CubeColor {
    Blue,
    Green,
    Red,
    White,
    Yellow,
    Black,
    Purple,
    Orange,
}

impl CubeColor {
    pub fn value(&self) -> &str {
        match *self {
            CubeColor::Blue => "#0000FF",
            CubeColor::Green => "#008000",
            CubeColor::Red => "#FF0000",
            CubeColor::White => "#FFFFFF",
            CubeColor::Yellow => "#FFFF00",
            CubeColor::Black => "#000000",
            CubeColor::Purple => "#800080",
            CubeColor::Orange => "#FFA500",
        }
    }
}

impl Default for CubeColor {
    fn default() -> Self {
        CubeColor::White
    }
}

#[derive(Debug)]
pub struct Cubes {
    pub grid: RwLock<Vec<Vec<CubeColor>>>,
}

impl Default for Cubes {
    fn default() -> Self {
        Cubes {
            grid: RwLock::new(vec![vec![CubeColor::default(); 5]; 5]),
        }
    }
}
