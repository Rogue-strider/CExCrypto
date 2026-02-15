# 🧾 In-Memory Orderbook (Rust + Actix Web)

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

src/
├── main.rs # Application entry point
├── routes.rs # HTTP API routes
├── orderbook.rs # Core orderbook logic
├── inputs.rs # Request DTOs
└── outputs.rs # Response DTOs

---

## 🧠 Core Concepts Implemented

- **Orderbook**
  - Separate buy (bids) and sell (asks) sides
  - Price-level grouping using `BTreeMap`
  - FIFO order storage per price level

- **Market Depth**
  - Aggregated quantity per price level
  - Sorted bids (highest first) and asks (lowest first)
  - `lastUpdateId` for versioning

- **Concurrency**
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
```{
  "order_id": "ads"
}
```

### ❌ Delete Order (WIP)
`DELETE /order`

**Request**
```
{
  "order_id": "ads"
}
```

**Response**
```
{
  "filled_qty": 0,
  "average_price": 100
}
```

### 📊 Get Market Depth
`GET /depth`

**Response**
```
{
  "bids": [[100, 5], [99, 3]],
  "asks": [[101, 2]],
  "lastUpdateId": "10"
}
```

## ▶️ Running the Project

### Prerequisites
- Rust (stable)
- Cargo

### Run Locally
```bash
cargo run
```
- The server will start at: http://127.0.0.1:8080

### 🛣 Roadmap
- Matching engine (buy ↔ sell)
- Partial order fills
- Order cancellation
- WebSocket-based depth updates
- Persistence layer

### 📚 Learning Goal
This project is built to understand:
- How exchanges manage orderbooks
- Backend system design in Rust
- Safe shared state in async web servers
- Inspired by real-world trading systems and backend engineering principles


.

### 🧑‍💻 Author (RogueStrider - Satyam)
Built with ❤️ while learning Rust backend & systems programming.
