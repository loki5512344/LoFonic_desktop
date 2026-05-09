# LoFonic Desktop MVP Structure

Project scaffold:

- `src/main.rs`: desktop app entrypoint (`eframe`)
- `src/desktop/app`: app shell and screen rendering
- `src/desktop/components`: sidebar, queue panel, playback bar, import controls
- `src/desktop/screens`: route enum and labels
- `src/desktop/widgets`: reusable metadata strip and waveform/progress widget
- `src/desktop/theme`: palette + egui style setup
- `src/desktop/playback`: queue + track models
- `src/desktop/grpc`: backend API contract + mock client
- `src/desktop/state`: global app state and dependency wiring
- `src/desktop/assets`: app-level constants and future static assets
- `proto/`: gRPC protocol definitions
- `docs/`: implementation notes for MVP workflow and integration
