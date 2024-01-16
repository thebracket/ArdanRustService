# Merging Routers

If you want to provide a `Router` that is a combination of several `Routers`, but shares the same base URL, you can use the `merge` function. This allows you to build different middleware stacks per router, and then merge them into a coherent external API without needing to nest your URLs.

