use crate::packet::var_int::VarInt;
use crate::packet::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode)]
pub struct HealthUpdateS2c {
    pub health: f32,
    pub food: VarInt,
    pub food_saturation: f32,
}
