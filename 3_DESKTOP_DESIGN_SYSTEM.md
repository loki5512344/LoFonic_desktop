# LoFonic Desktop Design System

---

# Design Identity

LoFonic Desktop should feel like:

- premium audio software
- studio hardware
- modern hi-fi system
- native Linux desktop app
- warm analog interface

NOT:

- RGB gaming UI
- Electron SaaS clone
- dashboard overload

---

# Core Visual Style

## Keywords

- dark
- warm
- analog
- minimal
- cinematic
- focused
- high fidelity

---

# Main Color Palette

## Background

```css
#0f0f0f
#121212
#161616
#1b1b1b
```

---

## Accent Colors

```css
#ff6b35
#e85d04
#f7c59f
#ff9b54
```

---

## Text Colors

Primary:

```css
#f4c89a
```

Secondary:

```css
#c88b5a
```

Muted:

```css
#8a6a52
```

---

# UI Philosophy

Music first.

UI should disappear while listening.

---

# Layout

## Sidebar

```text
Home
Library
Artists
Albums
Playlists
Downloads
Tasks
Settings
```

Width:

```text
220px - 260px
```

---

# Main Area

Contains:

- album grids
- playlists
- search
- recommendations
- queue
- metadata

---

# Right Queue Panel

ALWAYS AVAILABLE.

Queue is important.

Should contain:

- upcoming tracks
- duration
- drag reorder
- remove track
- repeat/shuffle indicators

---

# Bottom Playback Bar

Contains:

- cover art
- title
- artist
- controls
- progress bar
- waveform
- volume
- quality info
- LUFS info

---

# Audio Metadata Strip

Very important visual identity.

Examples:

```text
FLAC
24bit/96kHz
1411 kbps
-14.2 LUFS
```

This gives:

"real audio software" feeling.

---

# Album Covers

VERY IMPORTANT.

Rules:

- large covers
- soft corners
- subtle shadows
- cinematic presentation
- no aggressive borders

---

# Animations

Animations must be:

- smooth
- slow
- subtle
- premium feeling

Avoid:

- fast flashing
- aggressive transitions
- gamer effects

---

# Waveforms

Waveforms should:

- react smoothly
- use warm gradients
- feel expensive

NOT:

- dubstep visualizer
- random RGB bars

---

# Typography

Recommended:

- Inter
- Geist
- SF Pro
- IBM Plex Sans

Avoid:

- futuristic fonts
- cyberpunk fonts
- meme fonts

---

# Buttons

Rounded:

```text
8px - 14px
```

Soft glow allowed:

```css
box-shadow: 0 0 40px rgba(255,107,53,0.12);
```

ONLY subtle.

---

# Egui Architecture

Structure:

```text
desktop/
├── app/
├── components/
├── screens/
├── widgets/
├── theme/
├── playback/
├── grpc/
├── state/
└── assets/
```

---

# Main Screens

## Home

- recent tracks
- recommendations
- continue listening
- playlists

---

## Library

- all tracks
- fast search
- filters
- sorting

---

## Artist View

- artist image
- albums
- top tracks
- metadata

---

## Album View

- giant cover
- tracks
- waveform preview
- metadata

---

## Downloads

Shows:

- current jobs
- progress
- failed downloads
- retries

---

# Playback Engine

Use:

- rodio
- symphonia

Desktop handles:

- queue
- playback
- buffering
- gain normalization

Backend only streams.

---

# Egui Rules

## NEVER

- giant update() function
- business logic inside UI code
- blocking operations in render loop
- huge shared mutable state

---

# State Management

Use:

- Arc
- RwLock
- channels
- event-driven updates

---

# Rendering Philosophy

Prefer:

- simple widgets
- predictable rendering
- stable FPS

Avoid:

- overengineered custom rendering
- excessive effects

---

# UX Goal

User experience should feel:

- instant
- smooth
- calm
- premium
- focused on music

---

# Final Design Goal

LoFonic Desktop should feel like:

"high-end self-hosted audio workstation"

instead of:

"another web app inside desktop shell"
