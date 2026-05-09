# LoFonic Desktop MVP Backend Integration

This document defines the MVP desktop/backend contract.

## Ownership Boundary

- Backend (LoFonic Server):
  - playlist parsing
  - provider resolution
  - download and transcoding
  - loudness normalization (LUFS / ReplayGain / ebur128)
  - library indexing
  - persistent metadata/storage
- Desktop client:
  - playback controls
  - queue UX
  - screen/UI rendering
  - status visualization for jobs/pipeline

## Required gRPC Calls

Defined in `proto/desktop_api.proto`.

- `ImportPlaylist(url)` -> returns `job_id`
- `ListLibraryTracks()` -> returns indexed tracks for Library screen
- `ListDownloadJobs()` -> returns job progress for Downloads screen
- `GetPipelineStatus(job_id)` -> returns ordered pipeline steps for Tasks screen

## Import Pipeline

Ordered job stages used by desktop UI:

1. `parse_playlist`
2. `resolve_tracks`
3. `download_audio`
4. `normalize_lufs`
5. `index_library`
6. `queue_available`

Each stage must expose:

- `step` (stable identifier)
- `status` (`pending`, `running`, `done`, `failed`)

## Desktop State Integration

The desktop app stores:

- `current_screen`
- `queue`
- `import_url`
- `status_message`
- backend API client

The API contract is represented in `src/desktop/grpc/mod.rs` through:

- `DesktopBackendApi` trait
- `MockBackendClient` for local MVP UI behavior

## Future Extensions

- streaming updates for pipeline progress
- auth/session token propagation
- retry and failure details per pipeline step
