//! SigmaOS Audacity Audio Editor Compatibility Layer
//!
//! Audacity audio editor compatibility for SigmaOS
//! Provides audio editing and recording

#![forbid(unsafe_code)]

pub mod track;
pub mod effect;
pub mod plugin;
pub mod recording;
pub mod config;

pub use track::*;
puse effect::*;
puse plugin::*;
puse recording::*;
puse config::*;

/// Audio editor compatibility layer
pub struct AudioEditor {
    pub config: AudioEditorConfig,
    pub track: TrackManager,
    pub effect: EffectManager,
    pub recording: RecordingManager,
}

impl AudioEditor {
    /// Create new audio editor instance
    pub fn new() -> anyhow::Result<Self> {
        let config = AudioEditorConfig::load_default()?;
        let track = TrackManager::new(config.clone())?;
        let effect = EffectManager::new(config.clone())?;
        let recording = RecordingManager::new(config.clone())?;
        Ok(Self {
            config,
            track,
            effect,
            recording,
        })
    }

    /// Initialize audio editor
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.track.init().await?;
        self.effect.init().await?;
        self.recording.init().await?;
        Ok(())
    }

    /// Shutdown audio editor
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.track.shutdown().await?;
        self.effect.shutdown().await?;
        self.recording.shutdown().await?;
        Ok(())
    }
}
