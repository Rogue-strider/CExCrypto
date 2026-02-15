# 🧾 In-Memory Orderbook (Rust + Actix Web)

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)
![Actix Web](https://img.shields.io/badge/Actix--Web-4.x-blue?logo=actix)
![Status](https://img.shields.io/badge/Status-In%20Progress-yellow)
![License](https://img.shields.io/badge/License-MIT-green)

A simple **exchange-style in-memory orderbook** implemented in **Rust** using **Actix Web**.  
This project demonstrates how real trading platforms manage **buy/sell orders**, **market depth**, and **shared state** in a backend system.

---

## 🚀 Features

- 📦 In-memory orderbook
- 🟢 Buy (bid) & 🔴 Sell (ask) order handling
- 📊 Market depth aggregation (Binance-style format)
- 🔐 Thread-safe shared state using `Arc<Mutex<>>`
- ⚡ Built with `actix-web`
- 🧩 Clean modular architecture

---

## 🏗 Project Structure

```
src/
├── main.rs        # Application entry point
├── routes.rs      # HTTP API routes
├── orderbook.rs   # Core orderbook logic
├── inputs.rs      # Request DTOs
└── outputs.rs     # Response DTOs
```

---

## 🧠 Core Concepts Implemented

**Orderbook**
- Separate buy (bids) and sell (asks) sides
- Price-level grouping using `BTreeMap`
- FIFO order storage per price level

**Market Depth**
- Aggregated quantity per price level
- Sorted bids (highest first) and asks (lowest first)
- `lastUpdateId` for versioning

**Concurrency**
- Shared orderbook across requests
- Safe mutation with `Arc<Mutex<Orderbook>>`

---

## 📡 API Endpoints

### ➕ Create Order

`POST /order`

**Request**
```json
{
  "price": 100,
  "quantity": 2,
  "user_id": 1,
  "side": "Buy"
}
```

**Response**
```json
{
  "order_id": "a3f9c2d1-..."
}
```

---

### ❌ Delete Order *(Work in Progress)*

`DELETE /order`

> ⚠️ This endpoint is under active development and not yet fully functional.

**Request**
```json
{
  "order_id": "a3f9c2d1-..."
}
```

**Response**
```json
{
  "filled_qty": 0,
  "average_price": 100
}
```

---

### 📊 Get Market Depth

`GET /depth`

**Response**
```json
{
  "bids": [[100, 5], [99, 3]],
  "asks": [[101, 2]],
  "lastUpdateId": "10"
}
```

---

## 💡 Example Flow

Here's what happens when a buy and sell order match:

```
1. POST /order  →  Buy  100 @ $100  (queued as bid)
2. POST /order  →  Sell  50 @ $100  (matches bid, partial fill)
3. GET  /depth  →  bids: [[100, 50]], asks: []
```

> Matching engine is on the roadmap — currently orders are stored but not matched.

---

## ▶️ Running the Project

### Prerequisites

- Rust (stable) — [install here](https://rustup.rs)
- Cargo (comes with Rust)

### Run Locally

```bash
cargo run
```

Server starts at: `http://127.0.0.1:8080`

### Quick Test

```bash
# Create a buy order
curl -X POST http://127.0.0.1:8080/order \
  -H "Content-Type: application/json" \
  -d '{"price": 100, "quantity": 2, "user_id": 1, "side": "Buy"}'

# Check market depth
curl http://127.0.0.1:8080/depth
```

---

## 🛣 Roadmap

- [ ] Matching engine (buy ↔ sell)
- [ ] Partial order fills
- [ ] Order cancellation
- [ ] WebSocket-based depth updates
- [ ] Persistence layer

---

## 📚 Learning Goal

This project is built to understand:

- How exchanges manage orderbooks internally
- Backend system design in Rust
- Safe shared state in async web servers

Inspired by real-world trading systems and backend engineering principles.

---

## 🧑‍💻 Author

**RogueStrider (Satyam)**  
Built with ❤️ while learning Rust backend & systems programming.
