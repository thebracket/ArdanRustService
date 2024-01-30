# gRPC

Many REST APIs look a lot like remote procedure calls (RPC): you call a function remotely, process the response, and treat it like a regular function call. The only difference being that the function executes elsewhere.

Writing a full `Reqwest` handler and sharing data types works---but it's laborious. It's even more laborious when you want to handle clients running other languages.

Google invented `gRPC` for this use-case. `gRPC` uses `protobuf` to define protocols, and provides some automatic framework creation. Rust isn't on the officially blessed list of languages yet, but the Tokio team have built `tonic` to help.