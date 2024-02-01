# Scaling Out

When it comes time to scale out (Rust services are fast and small, so in most cases you've got time!), there are a few areas that frequently rear their ugly heads and bite you:

* State.
* Latency.
* Accidental Coupling.
* Do I Really Need Microservices?

## State

The `auth` module wouldn't scale out well at all, as-is. All of the authentication data is stored in a single, private SQLite database. That's easy to fix: use a "real" database (or cluster), and all instances can share this as the "source of truth".

You *can* safely cache valid tokens for a short amount of time to reduce API calls.

It's purely a design issue: whatever shared state you are relying on *must* be available to other instances of a service, or generatable by retrying a call.

## Latency

Breaking your service into many smaller services can do wonders for overall throughput, but it can also hurt your latency. Every RPC call to another service takes time. Imagine you have 5 layers of services. Each of them calls `auth` to validate a token. That's 5 times the load on the `auth` service---not great. It's also five times your application has to *wait* for an external service to reply. Even on localhost, that can be a few microseconds. Async is great at maintaining throughput while things wait---but if your service is really latency sensitive, you are adding a lot of delays to the individual request.

In this example, if possible you'd authenticate at the gateway---the first service. And then include a private "this guy is already authenticated" token in your subsequent request. You just shaved 5 API calls off your program at the expense of adding an internal token.

> Aside. I once helped a fellow who had a large service architecture. Every API call required a service locator call, and re-authenticated. It was possible to be 20 API calls deep for some operations, and performance was terrible!

