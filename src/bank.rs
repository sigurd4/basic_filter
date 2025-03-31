use core::f32::consts::FRAC_1_SQRT_2;

use serde::{Deserialize, Serialize};

use crate::parameters::BasicFilterParameters;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BasicFilterBank
{
    #[serde(default = "BasicFilterBank::default_filter")]
    pub filter: f32,
    #[serde(default = "BasicFilterBank::default_frequency")]
    pub frequency: f32,
    #[serde(default = "BasicFilterBank::default_resonance")]
    pub resonance: f32,
    #[serde(default = "BasicFilterBank::default_mix")]
    pub mix: f32
}

impl BasicFilterBank
{
    pub const fn default_filter() -> f32
    {
        0.0
    }

    pub const fn default_frequency() -> f32
    {
        880.0
    }
    
    pub const fn default_resonance() -> f32
    {
        FRAC_1_SQRT_2
    }
    
    pub const fn default_mix() -> f32
    {
        1.0
    }
}

impl Default for BasicFilterBank
{
    fn default() -> Self
    {
        Self {
            filter: Self::default_filter(),
            frequency: Self::default_frequency(),
            resonance: Self::default_resonance(),
            mix: Self::default_mix()
        }
    }
}

impl TryFrom<&BasicFilterParameters> for BasicFilterBank
{
    type Error = ();

    fn try_from(value: &BasicFilterParameters) -> Result<Self, Self::Error>
    {
        Ok(Self {
            filter: value.filter.get(),
            frequency: value.frequency.get(),
            resonance: value.resonance.get(),
            mix: value.mix.get()
        })
    }
}
impl TryFrom<BasicFilterParameters> for BasicFilterBank
{
    type Error = ();

    fn try_from(value: BasicFilterParameters) -> Result<Self, Self::Error>
    {
        (&value).try_into()
    }
}