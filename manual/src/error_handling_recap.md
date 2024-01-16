# Quick Recap on Nesting, Making Calls and Responses

We're pretty deep into understanding the Axum/Hyper/Tower/Tokio stack now --- and you've got the tools required to:

* Build `Router` applications to offer content.
* `Router` applications can offer state and `Extension` layers.
* Shared data with dependency injection and Rust's data-race protection.
* Nesting `Routers` to serve multi-tenant services.
* Calling `reqwest` to access other services.
* Using `IntoResponse` to clean up your handler return signatures.
* Combining `StatusCode`, `IntoResponse` and `map_err` for ergonomic error handling that hands out as much information as you want to the calling client as to why their call failed.

That gives you a *lot* of flexibility. But we're not done yet. Let's dig into middleware and what it can do for us.
