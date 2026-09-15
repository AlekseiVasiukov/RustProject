# Swagger Integration Guide

## What is Swagger/OpenAPI?

Swagger (now called OpenAPI) is a standard for documenting REST APIs.
It provides:
- **Auto-generated documentation** - Every endpoint is described
- **Interactive UI** - Test endpoints directly from your browser
- **Client generation** - Generate code for other languages

## How Swagger Works in Our Project

We use two crates:
- **`utoipa`**: Generates OpenAPI specs from Rust code annotations
- **`utoipa-swagger-ui`**: Serves the interactive Swagger UI in Axum

### How it Works (Step by Step)

1. We annotate our Rust types with `#[derive(ToSchema)]`
2. We annotate our handler functions with `#[utoipa::path(...)]`
3. We collect all annotations in the `ApiDoc` struct
4. Axum serves the OpenAPI JSON at `/api-docs/openapi.json`
5. Swagger UI at `/swagger-ui` reads the JSON and renders documentation

## Accessing Swagger UI

After starting the server (`cargo run`), open your browser to:

```
http://localhost:3000/swagger-ui
```

You'll see an interactive page with all endpoints listed.

## Adding a New Endpoint to Swagger

### Step 1: Add the handler to `#[utoipa::path]`

In [`src/server.rs`](src/server.rs), find the `ApiDoc` struct and add your handler:

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        game_query,
        your_new_handler,  // <-- Add your handler name here
    ),
    // ...
)]
pub struct ApiDoc;
```

### Step 2: Annotate your handler function

Add the `#[utoipa::path]` attribute above your function:

```rust
#[utoipa::path(
    get,                        // HTTP method
    path = "/your/endpoint",    // URL path
    tag = "YourTag",            // Group in Swagger
    responses(
        (status = 200, description = "Success", body = YourResponseType),
    )
)]
async fn your_new_handler() {
    // Your code here
}
```

### Step 3: Register the route in the router

In `create_router()`, add:

```rust
.route("/your/endpoint", get(your_new_handler))
```

## Customizing Request/Response Schemas

If you create a new response type, add it to:

1. Derive `ToSchema`:
```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourResponseType {
    pub field: String,
}
```

2. Add to `ApiDoc.components(schemas(...))`:
```rust
components(schemas(
    GameQueryRequest,
    GameQueryResponse,
    HealthStatus,
    YourResponseType,  // <-- Add here
))
```

## Versioning Your API

To version your API in Swagger, update the `version` field in `ApiDoc`:

```rust
#[openapi(
    info(title = "Game Engine API", version = "1.0.0"),
    // ...
)]
```

## Useful Links

- [utoipa Documentation](https://docs.rs/utoipa/)
- [OpenAPI Specification](https://swagger.io/specification/)
- [Axum + utoipa Examples](https://github.com/juhaku/utoipa/tree/main/examples)