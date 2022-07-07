use enumset::EnumSet;
use goxlr_types::{ChannelName, CompressorAttackTime, FaderName, FirmwareVersions, GateTimes, InputDevice, MicrophoneType, MuteFunction, OutputDevice, CompressorRatio, CompressorReleaseTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use strum::EnumCount;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub mixers: HashMap<String, MixerStatus>,
    pub paths: Paths,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixerStatus {
    pub hardware: HardwareStatus,
    pub fader_status: [FaderStatus; 4],
    pub mic_status: MicSettings,
    pub volumes: [u8; ChannelName::COUNT],
    pub router: [EnumSet<OutputDevice>; InputDevice::COUNT],
    pub router_table: [[bool; OutputDevice::COUNT]; InputDevice::COUNT],
    pub cough_button: CoughButton,
    pub bleep_volume: i8,
    pub profile_name: String,
    pub mic_profile_name: String,
}

impl MixerStatus {
    pub fn get_fader_status(&self, fader: FaderName) -> &FaderStatus {
        &self.fader_status[fader as usize]
    }

    pub fn get_channel_volume(&self, channel: ChannelName) -> u8 {
        return self.volumes[channel as usize];
    }

    pub fn set_channel_volume(&mut self, channel: ChannelName, volume: u8) {
        self.volumes[channel as usize] = volume;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareStatus {
    pub versions: FirmwareVersions,
    pub serial_number: String,
    pub manufactured_date: String,
    pub device_type: DeviceType,
    pub usb_device: UsbProductInformation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct FaderStatus {
    pub channel: ChannelName,
    pub mute_type: MuteFunction
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct CoughButton {
    pub is_toggle: bool,
    pub mute_type: MuteFunction
}

impl Default for FaderStatus {
    fn default() -> Self {
        FaderStatus {
            channel: ChannelName::Mic,
            mute_type: MuteFunction::All,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicSettings {
    pub mic_type: MicrophoneType,
    pub mic_gains: [u16; MicrophoneType::COUNT],

    pub equaliser: Equaliser,
    pub equaliser_mini: EqualiserMini,
    pub noise_gate: NoiseGate,
    pub compressor: Compressor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equaliser {
    pub gain: EqualiserGain,
    pub frequency: EqualiserFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualiserGain {
    pub eq_31h_gain: i8,
    pub eq_63h_gain: i8,
    pub eq_125h_gain: i8,
    pub eq_250h_gain: i8,
    pub eq_500h_gain: i8,
    pub eq_1k_gain: i8,
    pub eq_2k_gain: i8,
    pub eq_4k_gain: i8,
    pub eq_8k_gain: i8,
    pub eq_16k_gain: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualiserFrequency {
    pub eq_31h_freq: f32,
    pub eq_63h_freq: f32,
    pub eq_125h_freq: f32,
    pub eq_250h_freq: f32,
    pub eq_500h_freq: f32,
    pub eq_1k_freq: f32,
    pub eq_2k_freq: f32,
    pub eq_4k_freq: f32,
    pub eq_8k_freq: f32,
    pub eq_16k_freq: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualiserMini {
    pub gain: EqualiserMiniGain,
    pub frequency: EqualiserMiniFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualiserMiniGain {
    pub eq_90h_gain: i8,
    pub eq_250h_gain: i8,
    pub eq_500h_gain: i8,
    pub eq_1k_gain: i8,
    pub eq_3k_gain: i8,
    pub eq_8k_gain: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualiserMiniFrequency {
    pub eq_90h_freq: f32,
    pub eq_250h_freq: f32,
    pub eq_500h_freq: f32,
    pub eq_1k_freq: f32,
    pub eq_3k_freq: f32,
    pub eq_8k_freq: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseGate {
    pub threshold: i8,
    pub attack: GateTimes,
    pub release: GateTimes,
    pub enabled: bool,
    pub attenuation: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compressor {
    pub threshold: i8,
    pub ratio: CompressorRatio,
    pub attack: CompressorAttackTime,
    pub release: CompressorReleaseTime,
    pub makeup_gain: u8,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paths {
    pub profile_directory: PathBuf,
    pub mic_profile_directory: PathBuf,
    pub samples_directory: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbProductInformation {
    pub manufacturer_name: String,
    pub product_name: String,
    pub version: (u8, u8, u8),
    pub is_claimed: bool,
    pub has_kernel_driver_attached: bool,
    pub bus_number: u8,
    pub address: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Unknown,
    Full,
    Mini,
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::Unknown
    }
}
