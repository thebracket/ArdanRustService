# Understanding the Service Stack

Despite being 14 lines of code (including whitespace), the `simple_http_server` example actually does quite a lot. Before we dive into expanding it, lets take a quick tour through *how* Axum is making your system function.

Axum is built on a stack of different services:

**Layer**|**Purpose**|
--|--|
**Tokio**|Provides an async runtime and executor. By default it runs one thread per core, with work-stealing. It can be configured to run single-threaded, or on a limited number of threads.|
**Hyper**|Provides HTTP services, including request/response formatting and parsing. You *can* provide services with plain hyper, but by default the full stack will be invoked.|
**Tower**|Provides a middleware service layer on top of Hyper and Tokio. Tower can be used for everything from providing timeout services to authentication layers. Tower layers and extensions can be provided to Axum applications through dependency injection.|

> Keep this in mind as we progress. Services typically touch each layer, making use of that layer's speciality.

# Web Request Lifecycle

You open a browser (or CLI client) and go to "http://example.com/my_page". What is actually happening?

## 1. Your Browser sends a GET request

Your browser opens a TCP connection to the desired endpoint. It then sends a complete request:

```
GET /axum_hyper_tower_tokio.html HTTP/1.1
Host: localhost:3000
User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:122.0) Gecko/20100101 Firefox/122.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Referer: http://localhost:3000/rest_service.html
Connection: keep-alive
Cookie: Rustrover-e66a9a18=970c1180-bdea-4b4b-a21e-e657e49936b3; sysauth_localhost:9000_80=0ab787d88dc1e09f564d5ccc229bf9d0; User-Token=377b8330-6625-470c-9b73-5f569b130dfa
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: same-origin
Pragma: no-cache
Cache-Control: no-cache
```

The request includes the URL you want, the host you called, encodings you'll accept, your preferred language, the previous referral page, any cookies, token headers, cache-control - a *lot* of data. If you're writing the client, you can add more headers.

## 2. The Server Receives your Request

All of the header information is accepted by your server, and encoded. `Hyper` takes care of this. `Hyper` creates a `Request` object.

## 3. The Method and URL are Matched Against a Router

Given `GET` and `/test_page`, Axum scans the active `Router` object to find a router that matches the request.

## 4. Layers are Invoked

Any layers that are attached to your `Router` and operate on `Request` objects are called. They are called in turn, one at a time---passing the modified `Request` on to the next one.

## 5. Handler Matching and Dependency Injection

Once layers are done, the `Request` is passed to the `Router`. Your handler's function signature has been built to include requests for any dependencies that are required. These are collected.

## 6. Your Handler Runs

After all this, your handler finally runs! It receives any dependencies given to it by previous layers. Your Handler returns a `Response` object. `Hyper` and `Axum` collectively encode it to include not just your content but all headers that are required.

## 7. Layers are Invoked

Any layers that operate on `Response` objects are invoked. `tower_http` provides a lot of these, for example to compress your data.

## 8. The Response is Sent

Finally, the webserver sends your response to the client.

So HTTP is deceptively simple: there's a lot going on here! The Axum/Tower/Hyper/Tokio stack gives you a great deal of control over each step.