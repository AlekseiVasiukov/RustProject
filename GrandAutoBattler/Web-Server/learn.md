# Learning Asynchronous Rust for Web Development

To move from a synchronous console application to a modern web server, you need to understand three foundational pillars: **Tokio**, **Axum**, and **Serde**.

## 1. Tokio: The Asynchronous Runtime

In standard Rust, when you call a function, the thread waits for it to finish. In a web server, if one request takes 5 seconds to calculate a battle, you don't want the entire server to freeze for everyone else.

**Tokio** is an asynchronous runtime. Think of it as a "manager" for thousands of tiny tasks called **Futures**.

### Key Concepts:
* **`async` keyword**: When you mark a function as `async`, it doesn't execute immediately. Instead, it returns a `Future`—a recipe for work that will happen later.
* **`.await` keyword**: This is how you say, "Pause this specific task here until the work is done, but let the CPU go do other tasks in the meantime."
* **The Runtime**: You don't run `main()` normally; you run `#[tokio::main] async fn main()`. This starts the engine that manages all your tasks.

**Analogy**: 
* **Synchronous**: A chef who cooks one dish at a time. They boil water, and they stand there watching the pot until it bubbles, doing nothing else.
* **Asynchronous (Tokio)**: A chef who puts water on to boil, sets a timer, and immediately starts chopping onions. When the timer goes off, they return to the water.

---

## 2. Axum: The Web Framework

**Axum** is a library built on top of Tokio that handles the "HTTP" part. It's like a highly efficient receptionist for your server.

### How it works:
1. **Routing**: You tell Axum: "If someone sends a `GET` request to `/factions`, call this specific function."
2. **Extractors**: Axum can automatically pull data out of a request. If a client sends JSON, Axum can "extract" it directly into a Rust `struct`.
3. **Handlers**: These are the `async` functions you write that contain your business logic (e.g., asking the Game Engine for a result).

**Example Flow**:
`Client Request (HTTP POST)` $\rightarrow$ `Axum Router` $\rightarrow$ `Your Async Handler` $\rightarrow$ `Game Engine` $\rightarrow$ `Response`

---

## 3. Serde: The Data Translator

Computers talk in bits and bytes. Humans and Web Clients talk in **JSON** (JavaScript Object Notation). 

**Serde** (short for **Ser**ializing and **De**serializing) is the industry standard for converting Rust data structures into other formats and back.

* **Serialization**: Turning a Rust `struct` (like `Combatant`) into a JSON string to send to the client.
* **Deserialization**: Taking a JSON string received from a client and turning it back into a Rust `struct` so you can use it in your engine.

### Why do we need it?
Without Serde, you would have to manually write code to parse every single string and number in a JSON message. With Serde, you just add `#[derive(Serialize, Deserialize)]` to your struct, and it "just works."

---

## Summary Table

| Tool | Role | Analogy |
|---|---|---|
| **Tokio** | Runtime / Executor | The Chef (managing multiple tasks) |
| **Axum** | Web Framework | The Waiter (taking orders and delivering food) |
| **Serde** | Data Serializer | The Translator (converting Menu items to the kitchen) |

## Moving Forward
In our project, we will use:
1. **Tokio** to keep the server responsive.
2. **Axum** to create our endpoints (URLs).
3. **Serde** to allow the client to "read" our `Combatant` and `GameResult` objects.
