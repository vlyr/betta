use betta_core::error::Result;
use rodio::queue::SourcesQueueInput;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct Server {
    queue: VecDeque<PathBuf>,
    audio_input: Arc<SourcesQueueInput<f32>>,
}

impl Server {
    pub fn new(input: Arc<SourcesQueueInput<f32>>) -> Self {
        Self {
            audio_input: input,
            queue: VecDeque::new(),
        }
    }

    pub fn queue_mut(&mut self) -> &mut VecDeque<PathBuf> {
        &mut self.queue
    }

    pub fn queue(&self) -> &VecDeque<PathBuf> {
        &self.queue
    }

    pub fn audio_input(&self) -> &Arc<SourcesQueueInput<f32>> {
        &self.audio_input
    }

    pub fn queue_directory<P>(&mut self, dir: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        Ok(fs::read_dir(dir.as_ref())?
            .filter_map(|res| res.ok())
            .map(|f| f.path())
            .for_each(|path| self.queue.push_back(path)))
    }
}
