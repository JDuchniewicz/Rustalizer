mod dsp;

use crate::equalizer::dsp::DSP;
use crate::errors::{Error, StreamOp};
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
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
    pub fn new(
        device_name: &Option<String>,
        host_name: &Option<String>,
        bins: Option<usize>,
    ) -> Result<Equalizer, Error> {
        let mut host = cpal::default_host(); // default host [ALSA]
        if let Some(hostname) = host_name.as_ref() {
            for h in cpal::available_hosts() {
                if h.name() == hostname {
                    if let Ok(host_enum) = cpal::host_from_id(h) {
                        host = host_enum;
                    } else {
                        warn!("There is no such host, defaulting to ALSA");
                    }
                }
            }
        }

        let mut device: Option<cpal::Device> = None;

        if let Some(devicename) = device_name.as_ref() {
            for dev in host.input_devices().unwrap() {
                if let Ok(dev_name) = dev.name() {
                    if dev_name.eq(devicename) {
                        device.replace(dev);
                        info!("device {}", device.as_ref().unwrap().name().unwrap());
                    }
                }
            }
        }

        if device.is_none() {
            return Err(Error::NoCpalDevice);
        }

        /*
        let mut supported_configs_range = device
            .unwrap()
            .supported_input_configs()
            .expect("error while querying configs");

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
        let device = device.unwrap();

        Ok(Equalizer {
            core: Arc::new(Mutex::new(DSP::new(bins))), // TODO: extend to different formats?
            device,
            config,
            stream: None,
            status: false,
        })
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        let err_fn = move |err| {
            error!("An error ocurred on stream: {}", err);
        };
        let core_arc_clone = self.core.clone(); // local reference that is shared with the closure
        let stream = self.device.build_input_stream(
            &self.config,
            move |data, _: &cpal::InputCallbackInfo| {
                // note to self -> because rust moves all what closure captures, need a cloned Arc reference and thread safety -> Mutex
                // stream events etc here
                info!("Data received from CPAL, length {}", data.len());
                if let Ok(core) = core_arc_clone.try_lock() {
                    core.send(data);
                }
            },
            err_fn,
        )?;

        self.stream = Some(stream);
        Ok(())
    }

    pub fn play(&self) -> Result<(), Error> {
        match &self.stream {
            Some(stream) => {
                stream.play()?;
                Ok(())
            }
            None => Err(Error::StreamOperation(StreamOp::Play)),
        }
    }

    pub fn pause(&self) -> Result<(), Error> {
        match &self.stream {
            Some(stream) => {
                stream.pause()?;
                Ok(())
            }
            None => Err(Error::StreamOperation(StreamOp::Pause)),
        }
    }

    pub fn get_processed_samples(&self) -> Option<Vec<usize>> {
        if let Ok(core) = self.core.try_lock() {
            core.receive()
        } else {
            None
        }
    }

    // function for processing data, need special AudioCORE

    pub fn query() -> () {
        let available_hosts = cpal::available_hosts();
        error!("Available hosts: \n {:?}", available_hosts);

        for host_id in available_hosts {
            error!("{}", host_id.name());
            let host = cpal::host_from_id(host_id).unwrap();

            let default_in = host.default_input_device().map(|e| e.name().unwrap());
            let default_out = host.default_output_device().map(|e| e.name().unwrap());
            error!("Default Input Device: \n {:?}", default_in);
            error!("Default Output Device: \n {:?}", default_out);

            let devices = host.devices().unwrap();

            for (device_idx, device) in devices.enumerate() {
                error!("{} \t {}", device_idx, device.name().unwrap());
            }
        }
    }
}
