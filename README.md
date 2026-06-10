<div align="center">

# 🏔️ NepPay

**Developer-First Infrastructure for Nepal**

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)
![Status](https://img.shields.io/badge/status-active-brightgreen.svg)

</div>

---

## Overview

NepPay provides a **unified and extensible interface** for merchants and developers to integrate digital payments into their applications through a single, consistent API.

The platform is designed to simplify payment integration by abstracting provider-specific implementations behind a common interface — managing payment creation, verification, transaction tracking, and merchant communication while maintaining a modular architecture that supports multiple payment providers.

---

## Features

| | Feature | Description |
|---|---|---|
| 💳 | **Payment Creation & Processing** | End-to-end payment lifecycle management |
| ✅ | **Transaction Verification** | Reliable confirmation of payment events |
| 📦 | **Order Management** | Structured tracking of merchant orders |
| 📡 | **Payment Status Tracking** | Real-time visibility into transaction state |
| 🔔 | **Webhook Notifications** | Event-driven delivery to merchant endpoints |
| 🔐 | **Merchant Authentication** | Secure, token-based merchant access |
| 📲 | **Dynamic QR Payment Support** | Per-transaction QR generation |
| 🧩 | **Provider-Based Architecture** | Extensible design for future integrations |
| 🌐 | **RESTful API** | Consistent, integration-ready HTTP interface |

---

## Architecture

```
                    Customer
                        │
                        ▼
              Merchant Application
                        │
                        ▼
                     NepPay API
                        │
        ┌───────────────┴───────────────┐
        │                               │
        ▼                               ▼
 Payment Processing             Transaction Store
        │                               │
        ▼                               ▼
 Payment Provider              Webhooks & Events
```

---

## Core Principles

- **Simplicity** — A single, unified API eliminates the overhead of managing multiple provider integrations
- **Security** — Security-first design embedded at every layer of the platform
- **Extensibility** — A provider-based architecture that scales without touching the core
- **Reliability** — Durable transaction management built for consistency under real-world conditions
- **Developer Experience** — Thoughtfully designed for fast, frictionless integration

---

## Technology Stack

| Technology | Role |
|---|---|
| **Rust** | Core runtime — memory-safe, fast, and production-grade |
| **Axum** | Async web framework for routing and HTTP handling |
| **Tokio** | Asynchronous runtime for non-blocking I/O |
| **SQLite** | Embedded, zero-config transaction persistence |

---

## Project Structure

The codebase is organized around a **provider-independent payment core**. Payment providers are implemented as isolated modules, allowing new integrations to be introduced without significant changes to the core system. This separation of concerns keeps the platform maintainable as it grows.

---

## Vision

> NepPay aims to become the foundational payment infrastructure for Nepal — delivering a consistent developer experience across multiple payment providers while abstracting away the complexities of payment processing, verification, and event delivery.

---

## License

Released under the **MIT License**.

---

<div align="center">

Built By Developer for Developer &nbsp;•&nbsp; NepPay

</div>
