# Understanding the Service Stack

Despite being 14 lines of code (including whitespace), the `simple_http_server` example actually does quite a lot. Before we dive into expanding it, lets take a quick tour through *how* Axum is making your system function.

Axum is built on a stack of different services:

**Layer**|**Purpose**|
--|--|
**Tokio**|Provides an async runtime and executor. By default it runs one thread per core, with work-stealing. It can be configured to run single-threaded, or on a limited number of threads.|
**Hyper**|Provides HTTP services, including request/response formatting and parsing. You *can* provide services with plain hyper, but by default the full stack will be invoked.|
**Tower**|Provides a middleware service layer on top of Hyper and Tokio. Tower can be used for everything from providing timeout services to authentication layers. Tower layers and extensions can be provided to Axum applications through dependency injection.|

Keep this in mind as we progress. Services typically touch each layer, making use of that layer's speciality.