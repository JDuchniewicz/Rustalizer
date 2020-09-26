// this should act upon data received from the audio connection module?
// data is processed and functions are tested, then it is fed to the gui app via some kind of
// connectin?

mod dsp;

use crate::equalizer::dsp::DSP;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use std::cell::Cell;
use std::sync::{Arc, Mutex};

pub struct Equalizer {
    // handle to audio file,stream etc
    core: Arc<Mutex<DSP>>,
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: Option<Stream>,
    status: bool,
}

impl Equalizer {
    pub fn new(device_nr: Option<u32>) -> Result<Equalizer, &'static str> {
        let host = cpal::default_host(); //TODO: for now default host [ALSA]

        let mut device = host
            .default_input_device()
            .expect("no input device available");

        for dev in host.input_devices().unwrap() {
            if dev.name().unwrap().contains("sysdefault:CARD=Loopback") {
                device = dev;
                debug!("device {}", device.name().unwrap());
            }
        }

        let mut supported_configs_range = device
            .supported_input_configs()
            .expect("error while querying configs");

        /*
        for config in &mut supported_configs_range {
            debug!(
                "supported_config ch {} min_sr {:?} max_sr {:?} buf_size {:?} sample_fmt {:?}",
                config.channels(),
                config.min_sample_rate(),
                config.max_sample_rate(),
                config.buffer_size(),
                config.sample_format()
            );
        }
        */
        // TODO: match on input parameters and construct the config
        // check them for correctness with supported range
        let config = cpal::StreamConfig {
            channels: 1, // TODO: crashes on more than one channel
            sample_rate: cpal::SampleRate(44100),
            buffer_size: cpal::BufferSize::Default, // TODO: magic numbers for buffer cause ALSA panics
        };

        Ok(Equalizer {
            core: Arc::new(Mutex::new(DSP::new())), // TODO: extend to different formats?
            device,
            config,
            stream: None,
            status: false,
        })
    }

    pub fn connect(&mut self) -> () {
        let err_fn = move |err| {
            error!("An error ocurred on stream: {}", err);
        };
        let core_arc_clone = self.core.clone(); // local reference that is shared with the closure
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data, _: &cpal::InputCallbackInfo| {
                    // note to self -> because rust moves all what closure captures, need a cloned Arc reference and thread safety -> Mutex
                    // TODO: add a DSP module function
                    // stream events etc here
                    info!("Data received from CPAL, length {}", data.len());
                    if let Ok(core) = core_arc_clone.try_lock() {
                        core.send(data);
                    }
                },
                err_fn,
            )
            .expect("cannot create a stream!");

        self.stream = Some(stream);
    }

    pub fn play(&self) -> Result<(), &'static str> {
        match &self.stream {
            Some(_stream) => {
                self.stream
                    .as_ref()
                    .unwrap()
                    .play()
                    .expect("cannot play the audio stream!"); // TODO: how to handle errors properly!!!??
                Ok(())
            }
            None => Err("No stream set! Run connect first!"),
        }
    }

    pub fn pause(&self) -> Result<(), &'static str> {
        match &self.stream {
            Some(_stream) => {
                self.stream
                    .as_ref()
                    .unwrap()
                    .pause()
                    .expect("cannot stop the audio stream!"); // TODO: how to handle errors properly!!!??
                Ok(())
            }
            None => Err("No stream set! Run connect first!"),
        }
    }

    pub fn get_processed_samples(&self) -> Option<Vec<f32>> {
        if let Ok(core) = self.core.try_lock() {
            core.receive()
        } else {
            None
        }
    }

    // function for processing data, need special AudioCORE

    pub fn query() -> () {
        let available_hosts = cpal::available_hosts();
        debug!("Available hosts: \n {:?}", available_hosts);

        for host_id in available_hosts {
            debug!("{}", host_id.name());
            let host = cpal::host_from_id(host_id).unwrap();

            let default_in = host.default_input_device().map(|e| e.name().unwrap());
            let default_out = host.default_output_device().map(|e| e.name().unwrap());
            debug!("Default Input Device: \n {:?}", default_in);
            debug!("Default Output Device: \n {:?}", default_out);

            let devices = host.devices().unwrap();

            for (device_idx, device) in devices.enumerate() {
                debug!("{} \t {}", device_idx, device.name().unwrap());
            }
        }
    }
}
