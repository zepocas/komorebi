use eframe::egui::{
    text::LayoutJob, Align, Context, Label, TextFormat, Ui,
};
use windows::Win32::Media::Audio::{
    IMMDeviceEnumerator, MMDeviceEnumerator, IAudioSessionManager2, 
    IAudioSessionControl2, IAudioEndpointVolume, EDataFlow_eCapture,
    ERole_eCommunications, AudioSessionState_AudioSessionStateActive,
    PKEY_AudioEndpoint_Volume,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
use windows::core::{Interface, GUID};
use std::time::{Duration, Instant};
use std::process::Command;
use serde::{Deserialize, Serialize};

use crate::config::LabelPrefix;
use crate::render::RenderConfig;
use crate::widget::BarWidget;
use crate::selected_frame::SelectableFrame;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct MicrophoneConfig {
    /// Enable the Microphone widget
    pub enable: bool,
    /// Data refresh interval (default: 1 second)
    pub data_refresh_interval: Option<u64>,
    /// Display label prefix
    pub label_prefix: Option<LabelPrefix>,
    /// Show volume percentage
    pub show_volume: Option<bool>,
}

pub struct Microphone {
    pub enable: bool,
    data_refresh_interval: u64,
    label_prefix: LabelPrefix,
    last_updated: Instant,
    is_active: bool,
    is_muted: bool,
}

impl From<MicrophoneConfig> for Microphone {
    fn from(value: MicrophoneConfig) -> Self {
        let data_refresh_interval = value.data_refresh_interval.unwrap_or(1);

        Self {
            enable: value.enable,
            data_refresh_interval,
            label_prefix: value.label_prefix.unwrap_or(LabelPrefix::Icon),
            last_updated: Instant::now()
                .checked_sub(Duration::from_secs(data_refresh_interval))
                .unwrap(),
            is_active: false,
            is_muted: false,
        }
    }
}

impl Microphone {
    

    
}

impl BarWidget for Microphone {
    fn render(&mut self, ctx: &Context, ui: &mut Ui, config: &mut RenderConfig) {
        if self.enable {
            
        }
    }
}