use crate::circuit::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

/// A series of components that are connected by a chain
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Chain {
    pub connections: Vec<Connection>,
}

impl Chain {
    pub fn new(parts: &[PartIndex], level: ChainLevel, cw: Rotation) -> Self {
        let connections = parts
            .iter()
            .map(|part_index| Connection {
                part_index: (*part_index).into(),
                level,
                cw,
            })
            .collect();

        Self { connections }
    }
}

/// The location on a component where a chain connects
#[derive(Clone, Serialize, Deserialize)]
pub struct Connection {
    /// The index of the corresponding part in the `Chain.connections`
    #[serde(rename(serialize = "partIndex"))]
    pub part_index: u32,

    /// The level on the part where the chain is 0, 1, or 2
    pub level: ChainLevel,

    /// Direction of rotation for this chain
    pub cw: Rotation,
}

#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    /// The version of Spintronics simulator
    pub version: u32,

    /// Camera zoom [0.0, 1.0]
    pub zoom: f32,

    /// Camera view dimensions
    #[serde(rename(serialize = "viewDimensions"))]
    pub view_dimensions: Dimensions,

    /// Parts currently placed in this circuit
    pub parts: Vec<Part>,

    /// Chains currently connecting
    pub chains: Vec<Chain>,
}
