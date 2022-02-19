use serde::{Deserialize, Serialize};

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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CubeData {
    pub arr: Vec<Vec<CubeColor>>,
}

impl Default for CubeData {
    fn default() -> Self {
        CubeData {
            arr: vec![vec![CubeColor::default(); 1000]; 1000],
        }
    }
}
