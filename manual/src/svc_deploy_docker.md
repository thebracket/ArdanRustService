# Docker Deployment

> Make sure you have Docker and the Docker Desktop tools installed! A working example is in `/no_workspace/deploy_bookstore`. It's not in a workspace---but it could be, you'd have to change the path to `Cargo.lock`.

In the source folder for your project, type:

```bash
docker init
```

1. Select "Rust" (the default).
2. Select the default unless you need a specific version of Rust.
3. We listen on port 3001, but it's configurable. Let's choose 3002.

We still need to tell Docker that it should include the `static_html` content, and have a place to store the SqLite data files. We can edit `Dockerfile`, and change the second stage:

```
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
RUN mkdir -p /db && chown -R appuser /db
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
COPY static_html /bin/static_html

# Expose the port that the application listens on.
EXPOSE 3002

# What the container should run when it is started.
CMD ["/bin/server"]
```

Finally, we need to edit `compose.yaml` to include the environment variables and persist `/db` between runs:

```
services:
  server:
    build:
      context: .
      target: final
    ports:
      - 3002:3002
    environment:
      - APP_LISTEN_ADDRESS=0.0.0.0
      - APP_LISTEN_PORT=3002
      - APP_STATIC_CONTENT=/bin/static_html
      - AUTH_DB_FILENAME=/db/auth.db
      - BOOKSTORE_DB_FILENAME=/db/bookstore.db
    volumes:
      - db:/db
volumes:
  db:
```

Now you can run your program with `docker compose up`.