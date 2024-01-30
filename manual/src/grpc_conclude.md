# When to use gRPC

gRPC is a powerful alternative to regular REST (you can even have both in the same server). gRPC makes the most sense when your remote function calls look like a program: you initiate one connection, ask the other side to do things - acting as if the remote party is local - and eventually conclude the conversation.

REST meanwhile, is more request oriented. You send a request of some sort, and receive a reply. You can do this over and over. REST is also more suited to browser-based endpoints.

gRPC is very fast, and the protobuf system makes for nice protocol documentation. It's also not as widely adopted as REST.