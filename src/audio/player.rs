// Play sounds
use std::collections::HashMap;
use std::io::Cursor;
use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::sync::{Arc, Mutex};

/// Very small player that holds loaded sample bytes in memory by name.
/// Each play clones the bytes into a Cursor and decodes.
pub struct Player {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    samples: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl Player {
    pub fn new() -> Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        Ok(Self {
            _stream: stream,
            handle,
            samples: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Load a sample into memory. Path should point to a wav/mp3 file.
    pub fn load(&mut self, name: &str, path: &str) -> Result<()> {
        let bytes = crate::audio::loader::load_file_bytes(path)?;
        self.samples.lock().unwrap().insert(name.to_string(), bytes);
        Ok(())
    }

    /// Play a named sample (non-blocking). If sample not found, returns silently.
    pub fn play(&self, name: &str) {
        if let Some(bytes) = self.samples.lock().unwrap().get(name).cloned() {
            // create a new sink for short-lived playback
            if let Ok(sink) = Sink::try_new(&self.handle) {
                let cursor = Cursor::new(bytes);
                // Attempt to decode; if it fails, ignore
                if let Ok(decoder) = Decoder::new(cursor) {
                    sink.append(decoder);
                    sink.detach(); // allow sink to play in background
                }
            }
        }
    }
}
