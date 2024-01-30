# Hello Tonic

Tonic can automate a lot of the pain of building a service, and the provision of protobuf protocol files can serve as a level of automatic documentation. gRPC hasn't received as much attention as regular REST, but it's an option to consider.

Let's start by building a new project:

```
cargo new hello_tonic
cd hello_tonic
```

Next, we'll define our protocol:

```
mkdir proto
touch proto/hello.proto
```

Now edit `hello.proto`:

```proto
syntax = "proto3";
package hello;

service Greeter {
    rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
   string name = 1;
}

message HelloReply {
    string message = 1;
}
```

In this file, we're defining our *protocol*:

* `syntax = "proto3"` defines the file as using version 3 of the ProtoBuf format.
* `package hello` names your package. This name is important: the compiler will use it as the compiled output name.
* `service` allows you to define a service.
* `rpc SayHello` defines a remote procedure call named `SayHello`. We define that it requires a request, and returns a reply.
* The `message` sections allow us to define each of the data types we are using.

Congratulations - you've defined your first protocol.