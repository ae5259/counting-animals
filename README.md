# Counting Animals

This was an interesting task I was given from one of my e-mentors in order to
test and improve my general programming and rust knowledge, this task also
contains parts about multi-threading and networks, so make sure to try it out
yourself in your preferred language.

You can check the content of the task in [this file](./task.md)

## My implementation

My implementation is not fully finished yet, I have not covered the part where I
have to measure the time and write down the global state to a file, as the task
says:

```
The global state should be written to the file. Since writing to disk would be
very expensive if it's done every time the state updates, try to only write to
disk every 5s. Don't write to disk if nothing has changed.
```

I hope to implement in in the future.

- Client implementation is in [main.rs](./src/main.rs)
- Server implementation is in [server.rs](./src/server.rs)

- There is extra server implementation in TypeScript I wrote to test out the
  client with basic server, its content is in [serve.ts](./src/serve.ts)

### Have fun, make cool things.
