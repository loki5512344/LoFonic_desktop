# LoFonic Desktop MVP UI Workflow

This document maps the MVP UI to the design system and runtime data.

## Global Layout

- Left sidebar (`220-260px`) with route switch:
  `Home`, `Library`, `Artists`, `Albums`, `Playlists`, `Downloads`, `Tasks`, `Settings`
- Main content area:
  active screen body and primary actions
- Right queue panel (always available):
  upcoming tracks, repeat/shuffle toggles, queue state
- Bottom playback bar (always available):
  transport controls, progress/waveform placeholder, metadata strip

## Screen Responsibilities

## Home

- Entry point for paste/import flow
- Controls:
  - playlist URL input
  - import action button
- Data:
  - current status message from import pipeline

## Library

- Lists indexed tracks available for playback
- Data:
  - track title
  - artist
  - duration

## Artists

- Placeholder for artist-centric browsing
- Planned data:
  - artist image
  - top tracks
  - album list

## Albums

- Placeholder for album-centric browsing
- Planned data:
  - large cover
  - track listing
  - metadata preview

## Playlists

- User and imported playlists view
- Planned controls:
  - open playlist
  - queue playlist

## Downloads

- Shows ingestion/download jobs from backend
- Data:
  - job ID
  - progress (%)
  - state (`queued`, `downloading`, `normalized`, ...)

## Tasks

- Operational visibility for background pipeline
- Planned data:
  - import, download, normalize, index task stages

## Settings

- Client configuration view
- Planned controls:
  - server endpoint
  - audio quality/profile
  - client theme settings

## Always-On Components

## Right Queue Panel

- Upcoming track list
- Repeat/shuffle toggles
- Queue visibility regardless of active screen

## Bottom Playback Bar

- Transport buttons:
  previous / play-pause / next
- Progress + waveform placeholder
- Audio metadata strip:
  `codec`, `bit depth/sample rate`, `bitrate`, `LUFS`

## Theme Mapping

Palette follows the design system:

- Background: dark warm tones
- Accent: orange (`#ff6b35`)
- Text: warm beige hierarchy (`primary`, `secondary`, `muted`)
