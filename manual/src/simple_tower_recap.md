# Quick Recap on State and Layers

So to recap what we've gone over:

* A `Router` provides a connection between an URL and HTTP method, and a handler function.
* Routers can hold `State` objects, which are available for injection into any handler function called by that router application.
* Routers can add `layer`s. The `Extension` layer serves as additional state, and can be used to share as many injectible dependencies as you want. State is *slightly* faster than layers, but less flexible.
* Shared data in both *state* and *extension layers* must still obey Rust's safety rules. No data-races here!
* Arc ensures that the object can be shared between threads/tasks, without duplication and with very low overhead.
* Interior mutability allows you to safely create mutable shared state.

We've already got a pretty powerful engine. Let's start making it more useful.