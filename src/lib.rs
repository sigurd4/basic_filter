#![feature(split_array)]

use core::f64::consts::FRAC_1_SQRT_2;
use std::f64::EPSILON;
use std::f64::consts::TAU;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU8};

use num::Float;
use real_time_fir_iir_filters::iir::second::{OmegaZeta, SecondOrderFilter};
use real_time_fir_iir_filters::rtf::Rtf;
use vst::{prelude::*, plugin_main};

use self::parameters::BasicFilterParameters;

pub mod parameters;

const CHANGE: f64 = 0.2;

struct BasicFilterPlugin
{
    pub param: Arc<BasicFilterParameters>,
    filter: [SecondOrderFilter<f64>; CHANNEL_COUNT],
    rate: f64
}

const CHANNEL_COUNT: usize = 2;

impl BasicFilterPlugin
{
    fn process<F>(&mut self, buffer: &mut AudioBuffer<F>)
    where
        F: Float
    {
        let filter_type = self.param.filter.load(Ordering::Relaxed);
        let mix = self.param.mix.get() as f64;

        let omega = self.param.frequency.get() as f64*TAU;
        let zeta = 0.5/(self.param.resonance.get() as f64 + EPSILON);

        for ((input_channel, output_channel), filter) in buffer.zip()
            .zip(self.filter.iter_mut())
        {
            filter.param.omega.assign(CHANGE*omega + (1.0 - CHANGE)**filter.param.omega);
            filter.param.zeta.assign(CHANGE*zeta + (1.0 - CHANGE)**filter.param.zeta);

            for (input_sample, output_sample) in input_channel.into_iter()
                .zip(output_channel.into_iter())
            {
                let x = input_sample.to_f64().unwrap();
                let y = filter.filter(self.rate, x)[filter_type as usize];
                *output_sample = F::from(y*mix + x*(1.0 - mix)).unwrap();
            }
        }
    }
}

#[allow(deprecated)]
impl Plugin for BasicFilterPlugin
{
    fn new(_host: HostCallback) -> Self
    where
        Self: Sized
    {
        BasicFilterPlugin {
            param: Arc::new(BasicFilterParameters {
                filter: AtomicU8::from(0),
                frequency: AtomicFloat::from(880.0),
                resonance: AtomicFloat::from(0.5f32.sqrt()),
                mix: AtomicFloat::from(1.0)
            }),
            filter: [SecondOrderFilter::new(OmegaZeta::new(TAU*880.0, FRAC_1_SQRT_2)); CHANNEL_COUNT],
            rate: 44100.0
        }
    }

    fn get_info(&self) -> Info
    {
        Info {
            name: "Basic Filter".to_string(),
            vendor: "Soma FX".to_string(),
            presets: 0,
            parameters: 4,
            inputs: CHANNEL_COUNT as i32,
            outputs: CHANNEL_COUNT as i32,
            midi_inputs: 0,
            midi_outputs: 0,
            unique_id: 6436354,
            version: 1,
            category: Category::Effect,
            initial_delay: 0,
            preset_chunks: false,
            f64_precision: true,
            silent_when_stopped: true,
            ..Default::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32)
    {
        self.rate = rate as f64;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>)
    {
        self.process(buffer)
    }

    fn process_f64(&mut self, buffer: &mut AudioBuffer<f64>)
    {
        self.process(buffer)
    }

    fn get_tail_size(&self) -> isize
    {
        2
    }

    fn suspend(&mut self)
    {
        for filter in self.filter.iter_mut()
        {
            filter.reset()
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters>
    {
        self.param.clone()
    }
}

plugin_main!(BasicFilterPlugin);