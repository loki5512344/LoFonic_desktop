use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;

use crate::desktop::playback::TrackSummary;

#[derive(Debug, Clone)]
pub struct ImportPlaylistRequest {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ImportPlaylistResponse {
    pub job_id: String,
}

#[derive(Debug, Clone)]
pub struct DownloadJobStatus {
    pub id: String,
    pub progress_percent: f32,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct PipelineStep {
    pub step: String,
    pub status: String,
}

pub trait DesktopBackendApi: Send + Sync {
    fn import_playlist(&self, req: ImportPlaylistRequest) -> Result<ImportPlaylistResponse>;
    fn library_tracks(&self) -> Result<Vec<TrackSummary>>;
    fn download_jobs(&self) -> Result<Vec<DownloadJobStatus>>;
    fn pipeline_for_job(&self, job_id: &str) -> Result<Vec<PipelineStep>>;
}

#[derive(Debug, Default, Clone)]
pub struct MockBackendClient {
    latest_job: Arc<RwLock<Option<String>>>,
}

impl DesktopBackendApi for MockBackendClient {
    fn import_playlist(&self, req: ImportPlaylistRequest) -> Result<ImportPlaylistResponse> {
        let job_id = format!("job-{}", req.url.len());
        if let Ok(mut guard) = self.latest_job.try_write() {
            *guard = Some(job_id.clone());
        }
        Ok(ImportPlaylistResponse { job_id })
    }

    fn library_tracks(&self) -> Result<Vec<TrackSummary>> {
        Ok(vec![
            TrackSummary::demo("Studio Warmup", "LoFonic Artist", 242),
            TrackSummary::demo("Tape Delay", "LoFonic Artist", 211),
            TrackSummary::demo("Cloudless Night", "Guest Vocal", 268),
        ])
    }

    fn download_jobs(&self) -> Result<Vec<DownloadJobStatus>> {
        let mut jobs = vec![
            DownloadJobStatus {
                id: "job-a1".to_owned(),
                progress_percent: 72.0,
                state: "downloading".to_owned(),
            },
            DownloadJobStatus {
                id: "job-b7".to_owned(),
                progress_percent: 100.0,
                state: "normalized".to_owned(),
            },
        ];

        if let Ok(guard) = self.latest_job.try_read() {
            if let Some(job_id) = guard.as_ref() {
                jobs.insert(
                    0,
                    DownloadJobStatus {
                        id: job_id.clone(),
                        progress_percent: 14.0,
                        state: "queued".to_owned(),
                    },
                );
            }
        }

        Ok(jobs)
    }

    fn pipeline_for_job(&self, _job_id: &str) -> Result<Vec<PipelineStep>> {
        Ok(vec![
            PipelineStep {
                step: "parse_playlist".to_owned(),
                status: "done".to_owned(),
            },
            PipelineStep {
                step: "resolve_tracks".to_owned(),
                status: "done".to_owned(),
            },
            PipelineStep {
                step: "download_audio".to_owned(),
                status: "running".to_owned(),
            },
            PipelineStep {
                step: "normalize_lufs".to_owned(),
                status: "pending".to_owned(),
            },
            PipelineStep {
                step: "index_library".to_owned(),
                status: "pending".to_owned(),
            },
            PipelineStep {
                step: "queue_available".to_owned(),
                status: "pending".to_owned(),
            },
        ])
    }
}
