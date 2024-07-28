use serde::{Deserialize, Serialize, Serializer};

use std::path::Path;

use crate::save_file::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Component {
    Motor,
    Resistor,
    Junction,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ChainLevel {
    Bottom,
    Middle,
    Top,
}

impl Serialize for ChainLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Deserialize)]
pub enum Rotation {
    CounterClockwise,
    Clockwise,
}

impl Serialize for Rotation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(*self as u8 != 0)
    }
}

/// A component in the circuit
#[derive(Clone, Serialize, Deserialize)]
pub struct Part {
    /// The name of this part
    #[serde(rename(serialize = "type"))]
    name: Component,

    /// x coordinate of this part
    x: i32,

    /// y coordinate of this part
    y: i32,

    /// Optional value for this part (like the resistor value)
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<u32>,
}

impl Part {
    fn new(name: Component, x: i32, y: i32, value: Option<u32>) -> Self {
        Self { name, x, y, value }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PartIndex(usize);

impl From<PartIndex> for u32 {
    fn from(val: PartIndex) -> u32 {
        val.0 as u32
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ChainIndex(usize);

/// A circuit containing a connection of components
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Circuit {
    /// The list of parts currently in this circuit
    parts: Vec<(Component, Option<u32>)>,

    /// Lookup for which levels are currently occupied by a chain
    /// for each component
    level_occupied: Vec<[Option<ChainIndex>; 3]>,

    /// The current positions of each part
    positions: Vec<(i32, i32)>,

    /// Chains currently connecting components in this circuit
    chains: Vec<(ChainLevel, Vec<PartIndex>)>,
}

impl Circuit {
    pub fn new() -> Self {
        Self::default()
    }

    fn _add_part(&mut self, part: Component, value: Option<u32>) -> PartIndex {
        let index = self.parts.len();
        self.parts.push((part, value));
        self.level_occupied.push([None; 3]);
        self.positions.push((0, 0));

        self._adjust_positions();

        PartIndex(index)
    }

    fn _add_chain(&mut self, level: ChainLevel, parts: &[PartIndex]) -> ChainIndex {
        println!("WARN: ADD THE `make-chain-as-rectangular-as-possible` function");
        let index = self.chains.len();
        let parts = parts.to_vec();
        self.chains.push((level, parts));
        ChainIndex(index)
    }

    fn _adjust_positions(&mut self) {
        const PART_SPACING: i32 = 300;

        let grid_size = (self.parts.len().next_power_of_two() as f64).sqrt() as usize;

        // Update the position of all elements in the grid
        for index in 0..self.parts.len() {
            let row = (index % grid_size) as i32 * PART_SPACING;
            let col = (index / grid_size) as i32 * PART_SPACING;

            // Invert the y coordinate since the simluator has -y up and +y down
            let col = -col;
            self.positions[index] = (row, col);
        }
    }

    pub fn resistor(&mut self, value: u32) -> PartIndex {
        self._add_part(Component::Resistor, Some(value))
    }

    pub fn motor(&mut self) -> PartIndex {
        self._add_part(Component::Motor, None)
    }

    pub fn junction(&mut self) -> PartIndex {
        self._add_part(Component::Junction, None)
    }

    pub fn connect(&mut self, parts: &[PartIndex]) {
        let mut found = None;

        'next_level: for level in [ChainLevel::Bottom, ChainLevel::Middle, ChainLevel::Top] {
            for part in parts.iter() {
                if self.level_occupied[part.0][level as usize].is_some() {
                    continue 'next_level;
                }
            }

            // All parts have this level available
            found = Some(level);
            break;
        }

        let Some(level) = found else {
            panic!("No available chain to place this component");
        };

        // Make a new chain for this connection
        let chain = self._add_chain(level, parts);

        // Add the chain to each of the parts
        for part in parts.iter() {
            self.level_occupied[part.0][level as usize] = Some(chain);
        }
    }

    pub fn save<P: AsRef<Path>>(&self, save_path: P) {
        let mut parts = Vec::new();
        let mut chains = Vec::new();

        for (index, (x, y)) in self.positions.iter().enumerate() {
            let (part, value) = self.parts[index];
            parts.push(Part::new(part, *x, *y, value));
        }

        for (level, parts) in &self.chains {
            chains.push(Chain::new(parts.as_slice(), *level, Rotation::Clockwise));
        }

        let save_data = SaveFile {
            version: 1,
            zoom: 1.0,
            view_dimensions: Dimensions {
                width: 2000.,
                height: 2000.,
            },
            parts,
            chains,
        };

        let res = serde_json::to_string_pretty(&save_data).expect("Failed to save to json");

        std::fs::write(save_path, res).expect("Failed to write save file");
    }
}
