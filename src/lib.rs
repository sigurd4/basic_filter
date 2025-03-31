#![feature(split_array)]
#![feature(iter_array_chunks)]
#![feature(const_format_args)]
#![feature(generic_const_exprs)]

use std::sync::Arc;

use num::Float;
use real_time_fir_iir_filters::filters::iir::second::SecondOrderFilter;
use real_time_fir_iir_filters::rtf::Rtf;
use real_time_fir_iir_filters::{change::Change, conf::All};
use vst::{plugin_main, prelude::*};

use self::parameters::BasicFilterParameters;

moddef::moddef!(
    mod {
        bank,
        parameters
    },
);

const CHANGE: f64 = 0.2;

struct BasicFilterPlugin
{
    pub param: Arc<BasicFilterParameters>,
    pub filter: [SecondOrderFilter<All, f64>; CHANNEL_COUNT],
    pub rate: f64
}

const CHANNEL_COUNT: usize = 2;

impl BasicFilterPlugin
{
    fn process<F>(&mut self, buffer: &mut AudioBuffer<F>)
    where
        F: Float
    {
        let blend = self.param.blend();
        let mix = self.param.mix.get() as f64;
        let param = self.param.omega_zeta();

        for ((input_channel, output_channel), filter) in buffer.zip().zip(self.filter.iter_mut())
        {
            filter.param.change(param, CHANGE);

            for (input_sample, output_sample) in input_channel.into_iter().zip(output_channel.into_iter())
            {
                let x = input_sample.to_f64().unwrap();
                let y = filter.filter(self.rate, x).into_iter().zip(&blend).map(|(y, b)| y * *b).sum::<f64>();
                *output_sample = F::from(y * mix + x * (1.0 - mix)).unwrap();
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
        let param = BasicFilterParameters::default();
        let filter = SecondOrderFilter::new::<All>(param.omega_zeta());

        BasicFilterPlugin {
            param: Arc::new(param),
            filter: [filter; CHANNEL_COUNT],
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
