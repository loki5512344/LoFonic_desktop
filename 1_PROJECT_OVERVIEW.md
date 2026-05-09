# LoFonic

LoFonic is a Linux-first self-hosted music platform written in Rust.

The goal of the project is to provide:

- fast local music streaming
- playlist importing from external providers
- automatic downloading and normalization
- clean desktop listening experience
- permanent personal music cloud
- lightweight deployment on VPS or home server

LoFonic is NOT:

- another Plex clone
- video platform
- social music service
- bloated Electron app

Core philosophy:

"Paste playlist -> fully indexed normalized local library"

---

# Main Components

## LoFonic Server

Backend service responsible for:

- scanning music library
- metadata indexing
- playlist imports
- provider integrations
- downloading tracks
- transcoding
- audio normalization
- streaming audio
- authentication
- library management
- job processing

Linux-first.
Single binary.
Docker-friendly.
Self-hosted.

---

## LoFonic Desktop

Native desktop client built with:

- egui
- eframe
- Rust

Responsible for:

- playback
- queue
- playlists
- search
- remote streaming
- local controls
- waveform UI
- metadata display

Desktop is NOT a thin web wrapper.
It is a real native application.

---

# Core Features

## Playlist Import

User pastes:

- SoundCloud playlist
- YouTube playlist
- album URL
- track URL

LoFonic:

1. parses playlist
2. resolves tracks
3. downloads audio
4. normalizes loudness
5. fetches metadata
6. stores artwork
7. indexes library
8. updates DB
9. makes tracks available instantly

---

# Audio Philosophy

LoFonic focuses on:

- high quality audio
- loudness consistency
- clean metadata
- minimal latency
- stable playback

Supported:

- FLAC
- MP3
- OPUS
- AAC
- WAV
- M4A

---

# Loudness Normalization

LoFonic scans imported tracks using:

- LUFS
- ReplayGain
- ebur128

Goal:

No more:

- one song quiet
- another song extremely loud

Playback engine applies gain correction automatically.

---

# Architecture Style

LoFonic uses:

- modular monolith architecture
- async workers
- provider abstraction
- strict module boundaries
- job-based ingestion pipeline

NOT microservices.

At least not initially.

---

# Backend Stack

- Rust
- Tokio
- Axum
- Tonic (gRPC)
- SQLx
- SQLite/Postgres
- FFmpeg
- yt-dlp

---

# Desktop Stack

- Rust
- egui
- eframe
- rodio
- symphonia
- tonic gRPC client

---

# Deployment Philosophy

Primary target:

- Linux VPS
- home server
- mini PC
- NAS

Deployment methods:

- Docker
- docker-compose
- systemd service
- static binaries

---

# Database Philosophy

Database is source of truth.

Files are assets.

Metadata should not rely only on ID3 tags.

---

# Open Source

License:

GPL-3.0

---

# Product Identity

LoFonic identity:

- dark
- warm
- audio-focused
- minimal
- high fidelity
- native desktop feeling
- studio-inspired visuals

Not:

- RGB gamer UI
- neon cyberpunk overload
- web app in desktop wrapper

---

# Long Term Vision

LoFonic should feel like:

"Your permanent self-hosted music system"

not:

"temporary streaming cache"
