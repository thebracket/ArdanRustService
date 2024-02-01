# Service Exposure

You need to think about exposure on two levels:

* **In-code exposure**. Only expose the parts of your module that you are ok with being called from outside! You don't want to discover that another team was relying on what you thought was a private function, and suddenly your implementation detail is part of the API!
* **External exposure**. Not every service needs to be callable from outside. In the bookstore example, nothing is private to the outside world. If you have services that aren't available publically---don't include them in public APIs. Hide them on your local network, or require an access token that can't be generated from outside.

Rust offers three levels of privacy:
* `pub` - public. `pub` functions, fields and types are available to anyone who can access the module. Publicity is hierarchical---you can be public inside a module that isn't exposed from the outside. But be careful.
* `pub(crate)`, which is public---but only within the same crate.
* `private` (the default - no keyword needed). Only available inside the current module.

> A handy trick is to enable `![warn(missing_docs)]`. You'll now have warnings for every public function that doesn't have full Rust documentation provided---and none for your internal functions. That both tells you what to document, and gives a quick overview of what you've exposed to the world.
