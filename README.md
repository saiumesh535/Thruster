# Thruster [![Build Status](https://travis-ci.com/trezm/Thruster.svg?branch=master)](https://travis-ci.org/trezm/Thruster) ![Crates.io](https://img.shields.io/crates/v/thruster.svg) ![Crates.io](https://img.shields.io/crates/d/thruster.svg) [![Discord](https://img.shields.io/discord/658730946211610643)](https://discord.gg/m9JrPRd)

## An fast and intuitive rust web framework

✅ Runs in stable
✅ Runs fast
✅ Doesn't use unsafe

[Documentation](https://docs.rs/thruster)

## Features

- [built with async/await in mind](https://github.com/thruster-rs/Thruster/blob/master/thruster/examples/profiling.rs#L11)
- [hyper compatible](https://github.com/thruster-rs/Thruster/blob/master/thruster/examples/hyper_most_basic.rs)
- [ssl ready](https://github.com/thruster-rs/Thruster/tree/master/thruster/examples/hyper_most_basic_ssl)
- [testable](#testing)
- [static file serving](https://github.com/thruster-rs/Thruster/tree/master/thruster/examples/static_file)
- [socketio](https://github.com/thruster-rs/thruster-socketio)
- [gRPC](https://github.com/thruster-rs/Thruster/tree/master/thruster/examples/grpc), and more experimental [non-tonic based gRPC](https://github.com/thruster-rs/thruster-grpc)

## Motivation

Thruster is a web framework that aims for developers to be productive and consistent across projects and teams. Its goals are to be:
- Performant
- Simple
- Intuitive

Thruster also
- Does not use `unsafe`
- Works in stable rust

## Fast

Thruster can be run with different server backends and represents a nicely packaged layer over them. This means that it can keep up with the latest and greatest changes from the likes of Hyper, Actix, or even ThrusterServer, a home-grown http engine.

## Intuitive

Based on frameworks like Koa, and Express, thruster aims to be a pleasure to develop with.

## Example

To run the example `cargo run --example <example-name>`.
For example, `cargo run --example hello_world` and open [http://localhost:4321/](http://localhost:4321/)

### Middleware Based

The core parts that make the new async await code work is designating middleware functions with the `#[middleware_fn]` attribute (which marks the middleware so that it's compatible with the stable futures version that thruster is built on,) and then the `async_middleware!` macro in the actual routes.

A simple example for using async await is:

```rust
use std::boxed::Box;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

use thruster::{App, BasicContext as Ctx, Request};
use thruster::{m, middleware_fn, MiddlewareNext, MiddlewareResult, Server, ThrusterServer};

#[middleware_fn]
async fn profile(context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let start_time = Instant::now();

    context = next(context).await;

    let elapsed_time = start_time.elapsed();
    println!(
        "[{}μs] {} -- {}",
        elapsed_time.as_micros(),
        context.request.method(),
        context.request.path()
    );

    Ok(context)
}

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    Ok(context)
}

#[middleware_fn]
async fn four_oh_four(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.status(404);
    context.body("Whoops! That route doesn't exist!");
    Ok(context)
}

#[tokio::main]
fn main() {
    println!("Starting server...");

    let mut app = App::<Request, Ctx, ()>::new_basic();

    app.get("/plaintext", m![profile, plaintext]);
    app.set404(m![four_oh_four]);

    let server = Server::new(app);
    server.build("0.0.0.0", 4321).await;
}
```

### Error handling

It's recommended to use the `map_try!` macro from the main package. This has the same function as `try!`, but with the ability to properly map the error in a way that the compiler knows that execution ends (so there's no movement issues with `context`.)

This ends up looking like:

```rust
use thruster::errors::ThrusterError as Error;
use thruster::proc::{async_middleware, middleware_fn};
use thruster::{map_try, App, BasicContext as Ctx, Request};
use thruster::{MiddlewareNext, MiddlewareResult, MiddlewareReturnValue, Server, ThrusterServer};

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    Ok(context)
}

#[middleware_fn]
async fn error(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let res = "Hello, world".parse::<u32>();
    let non_existent_param = map_try!(res, Err(_) => {
        Error {
          context,
          message: "Parsing failure!".to_string(),
          status: 400
        }
      }
    );

    context.body(&format!("{}", non_existent_param));

    Ok(context)
}

#[middleware_fn]
async fn json_error_handler(context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let res = next(context).await;

    let ctx = match res {
        Ok(val) => val,
        Err(e) => {
            let mut context = e.context;
            context.body(&format!(
                "{{\"message\": \"{}\",\"success\":false}}",
                e.message
            ));
            context.status(e.status);
            context
        }
    };

    Ok(ctx)
}

#[tokio::main]
fn main() {
    println!("Starting server...");

    let mut app = App::<Request, Ctx, ()>::new_basic();

    app.use_middleware("/", m![json_error_handler]);

    app.get("/plaintext", m![plaintext]);
    app.get("/error", m![error]);

    let server = Server::new(app);
    server.build("0.0.0.0", 4321).await;
}
```

### Quick setup without a DB

The easiest way to get started is to just clone the [starter kit](https://github.com/trezm/thruster-starter-kit)

```bash
> git clone git@github.com:trezm/thruster-starter-kit.git
> cd thruster-starter-kit
> cargo run
```

The example provides a simple plaintext route, a route with JSON serialization, and the preferred way to organize sub routes using sub apps.

### Quick setup with postgres

The easiest way to get started with postgres is to install thruster-cli,

```bash
> cargo install thruster-cli
```

And then to run

```bash
> thruster-cli init MyAwesomeProject
> thruster-cli component Users
> thruster-cli migrate
```

Which will generate everything you need to get started! Note that this requires a running postgres connection and assumes the following connection string is valid:

```
postgres://postgres@localhost/<Your Project Name>
```

This is all configurable and none of it is hidden from the developer. It's like seeing the magic trick and learning how it's done! Check out the docs for [thruster-cli here](https://github.com/trezm/thruster-cli).

## Testing
Thruster provides an easy test suite to test your endpoints, simply include the `testing` module as below:

```rust
let mut app = App::<Request, Ctx, ()>::new_basic();

...

app.get("/plaintext", m![plaintext]);

...

let result = testing::get(app, "/plaintext");

assert!(result.body == "Hello, World!");
```

## Make your own middleware modules
Middleware is super easy to make! Simply create a function and export it at a module level. Below, you'll see a piece of middleware that allows profiling of requests:

```rust
#[middleware_fn]
async fn profiling<C: 'static + Context + Send + Sync>(
    mut context: C,
    next: MiddlewareNext<C>,
) -> MiddlewareResult<C> {
    let start_time = Instant::now();

    context = next(context).await?;

    let elapsed_time = start_time.elapsed();
    info!("[{}μs] {}", elapsed_time.as_micros(), context.route());

    Ok(context)
}
```

You might find that you want to allow for more specific data stored on the context, for example, perhaps you want to be able to hydrate query parameters into a hashmap for later use by other middlewares. In order to do this, you can create an additional trait for the context that middlewares downstream must adhere to. Check out the provided [query_params middleware](https://github.com/thruster-rs/Thruster/blob/master/thruster/src/middleware/query_params.rs) for an example.

## Other, or Custom Backends

Thruster is capable of just providing the routing layer on top of a server of some sort, for example, in the Hyper snippet above. This can be applied broadly to any backend, as long as the server implements `ThrusterServer`.

```rs
use async_trait::async_trait;

#[async_trait]
pub trait ThrusterServer {
    type Context: Context + Send;
    type Response: Send;
    type Request: RequestWithParams + Send;

    fn new(App<Self::Request, Self::Context>) -> Self;
    async fn build(self, host: &str, port: u16);
}
```

There needs to be:
- An easy way to create a server.
- A function to build the server into a future that could be loaded into an async runtime.

Within the `build` function, the server implementation should:
- Start up some sort of listener for connections
- Call `let matched = app.resolve_from_method_and_path(<some method>, <some path>);` (This is providing the actual routing.)
- Call `app.resolve(<incoming request>, matched)` (This runs the chained middleware.)

## Using cargo generate

_Note: This hasn't yet been updated for the latest version of thruster_

If you have `cargo generate` installed, you can simply run the [cargo generator](https://github.com/ami44/thruster-basic-template)

```
cargo generate --git https://github.com/ami44/thruster-basic-template.git --name myproject
```
