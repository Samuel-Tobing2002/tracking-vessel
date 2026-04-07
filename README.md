# 🚢 Ship Tracking System (AIS Simulation)

A real-time ship tracking system built using **Rust, Redis, PostgreSQL, and WebSocket**.

This project simulates AIS (Automatic Identification System) data and processes it through a modern backend architecture.

Dummy AIS → WebSocket → Redis → PostgreSQL → API

## ⚙️ Tech Stack

- 🦀 Rust (Axum)
- ⚡ Redis (Pub/Sub Message Broker)
- 🐘 PostgreSQL (Persistent Storage)
- 🔌 WebSocket (Real-time streaming)
- 📦 SQLx (Async DB Query)

---

## 🚀 Features

- ✅ Real-time ship position streaming (WebSocket)
- ✅ Redis Pub/Sub for decoupled architecture
- ✅ PostgreSQL for persistent storage
- ✅ Upsert system (no duplicate ships)
- ✅ Clean modular backend structure
- ✅ AIS simulation (dummy data generator)

