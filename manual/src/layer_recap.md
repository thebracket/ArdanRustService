# Layer Recap

Now you've got a very capable REST stack:

* You can build `Router` applications.
    * You can nest multiple applications into a single server with `nest`.
    * You can merge different application stacks together with `merge`.
* You can share state inside `Router`s with `State` and extension layers.
* You can call `reqwest` to call other REST services.
* You can use `impl IntoResponse` to provide a consistent handler interface.
* You can return `(StatusCode, String)` to provide detailed errors.
* You can create your own middleware layers to intercept requests and dynamically inject dependencies.
* You can utilize the `tower` and `tower_http` services to add compression, timeouts, CORS, rate limits, concurrency limits, etc.

In other words: you can build a very rich, powerful application with minimal boilerplate.