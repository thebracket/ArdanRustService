# Introduction

> All of this class material, including the code, is available at [this Github repo](https://github.com/thebracket/ArdanRustService)

This class will cover the ins and outs of integrating Rust into a Service-Oriented-Architecture. In this class, we'll go over:

* Building a REST service
    * Start with a minimal HTTP server.
    * Add state, immutable and mutable.
    * Add services with layers.
    * Nest multiple routers together.
    * Call other REST services.
    * Error Handling.
    * Utilizing Middleware.
    * Using Headers.
    * Additional Layer Services.
* Tracing
    * Logging
    * Timing with Spans
    * Timing Axum Requests from end-to-end
    * Logging targets: stdout, files, JSON
    * OpenTelemetry
* Documenting your REST API with OpenAPI
* Service Configuration
    * Environment Variables
    * Files
    * HTTP sources
    * The Command Line
* gRPC
    * Tonic - the gRPC server
    * Streaming
* Web Sockets
    * Web Socket server
    * Web Sockets and Serialization
    * Web Socket Client
* Service Deployment
* Service Design

## About Herbert Wolverson

Herbert has been developing software professionally for more than 20 years. Starting with BASIC, Pascal, and then moving onto C and C++, Herbert has developed custom applications ranging from web-server filters to games. Herbert is the author of Hands-on Rust and Rust Brain Teasers.

| Book | | Publisher E-Book | Amazon |
|------|-| -----------------|--------|
| Hands-on Rust | ![](./images/Hands-on-Rust.png) | [PragProg Page](https://pragprog.com/titles/hwrust/hands-on-rust/) | [Amazon Page](https://www.amazon.com/Hands-Rust-Effective-Learning-Development/dp/1680508164) |
| Rust Brain Teasers | ![](./images/Rust-Brain-Teasers.png) | [PragProg Page](https://pragprog.com/titles/hwrustbrain/rust-brain-teasers/) | [Amazon Page](https://www.amazon.com/Rust-Brain-Teasers-Pragmatic-Programmers/dp/1680509179) |

## Resources

I recommend bookmarking the following resources:

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
