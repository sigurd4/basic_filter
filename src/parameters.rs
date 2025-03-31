use core::f64::consts::TAU;

use real_time_fir_iir_filters::param::OmegaZeta;
use vst::{prelude::PluginParameters, util::AtomicFloat};

use crate::bank::BasicFilterBank;

const MIN_FREQ: f32 = 20.0;
const MAX_FREQ: f32 = 20000.0;
const MIN_RES: f32 = 0.0;
const RES_CURVE: f32 = 4.0;
const MAX_RES: f32 = 20.0;

#[derive(Debug)]
pub struct BasicFilterParameters
{
    pub filter: AtomicFloat,
    pub frequency: AtomicFloat,
    pub resonance: AtomicFloat,
    pub mix: AtomicFloat
}

impl BasicFilterParameters
{
    pub fn store(&self, data: BasicFilterBank)
    {
        self.filter.set(data.filter);
        self.frequency.set(data.frequency);
        self.resonance.set(data.resonance);
        self.mix.set(data.mix);
    }

    pub fn load(&self) -> Result<BasicFilterBank, ()>
    {
        self.try_into()
    }

    pub fn omega(&self) -> f64
    {
        self.frequency.get() as f64*TAU
    }
    pub fn zeta(&self) -> f64
    {
        0.5/(self.resonance.get() as f64 + f64::EPSILON)
    }
    pub fn omega_zeta(&self) -> OmegaZeta<f64>
    {
        OmegaZeta {
            omega: self.omega(),
            zeta: self.zeta()
        }
    }

    pub fn blend(&self) -> [f64; 3]
    {
        let f = self.filter.get() as f64*2.0;
        if f >= 1.0
        {
            [0.0, 2.0 - f, 1.0 - f]
        }
        else
        {
            [1.0 - f, f, 0.0]
        }
    }
}

impl From<BasicFilterBank> for BasicFilterParameters
{
    fn from(value: BasicFilterBank) -> Self
    {
        let BasicFilterBank {filter, frequency, resonance, mix} = value;
        Self {
            filter: AtomicFloat::new(filter),
            frequency: AtomicFloat::new(frequency),
            resonance: AtomicFloat::new(resonance),
            mix: AtomicFloat::new(mix)
        }
    }
}
impl Default for BasicFilterParameters
{
    fn default() -> Self
    {
        BasicFilterBank::default().into()
    }
}

impl PluginParameters for BasicFilterParameters
{
    fn get_parameter_label(&self, index: i32) -> String
    {
        match index
        {
            0 => "%".to_string(),
            1 => "Hz".to_string(),
            2 => "".to_string(),
            3 => "%".to_string(),
            _ => "".to_string()
        }
    }

    fn get_parameter_text(&self, index: i32) -> String
    {
        match index
        {
            0 => format!("{:.3}", 100.0*self.filter.get()),
            1 => format!("{:.3}", self.frequency.get()),
            2 => format!("{:.3}", self.resonance.get()),
            3 => format!("{:.3}", 100.0*self.mix.get()),
            _ => "".to_string()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String
    {
        match index
        {
            0 => "Filter".to_string(),
            1 => "Frequency".to_string(),
            2 => "Resonance".to_string(),
            3 => "Mix".to_string(),
            _ => "".to_string()
        }
    }

    /// Get the value of parameter at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32
    {
        match index
        {
            0 => self.filter.get(),
            1 => (self.frequency.get().log2() - MIN_FREQ.log2())/(MAX_FREQ.log2() - MIN_FREQ.log2()),
            2 => ((self.resonance.get() - MIN_RES)/(MAX_RES - MIN_RES)).powf(1.0/RES_CURVE),
            3 => self.mix.get(),
            _ => 0.0
        }
    }
    
    fn set_parameter(&self, index: i32, value: f32)
    {
        match index
        {
            0 => self.filter.set(value),
            1 => self.frequency.set((value*(MAX_FREQ.log2() - MIN_FREQ.log2()) + MIN_FREQ.log2()).exp2()),
            2 => self.resonance.set(MIN_RES + (MAX_RES - MIN_RES)*value.powf(RES_CURVE)),
            3 => self.mix.set(value),
            _ => ()
        }
    }

    fn change_preset(&self, _preset: i32) {}

    fn get_preset_num(&self) -> i32 {
        0
    }

    fn set_preset_name(&self, _name: String) {}

    fn get_preset_name(&self, _preset: i32) -> String {
        "".to_string()
    }

    fn can_be_automated(&self, index: i32) -> bool {
        index < 4
    }

    fn get_preset_data(&self) -> Vec<u8>
    {
        self.get_bank_data()
    }

    fn get_bank_data(&self) -> Vec<u8>
    {
        serde_json::to_vec(&self.load().expect("Serialization error")).expect("Serialization error")
    }

    fn load_preset_data(&self, data: &[u8])
    {
        self.load_bank_data(data);
    }

    fn load_bank_data(&self, data: &[u8])
    {
        self.store(serde_json::from_slice(data).expect("Deserialization error"))
    }
}