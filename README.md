# Counting Animals

This was an interesting task I was given from one of my e-mentors in order to
test and improve my general programming and rust knowledge, this task also
contains parts about multi-threading and networks, so make sure to try it out
yourself in your preferred language.

You can check the content of the task in [this file](./task.md)

## My implementation

> ~~My implementation is not fully finished yet, I have not covered the part
> where~~ ~~I have to measure the time and write down the global state to a
> file, as the~~ ~~task says:~~
>
> ```
> The global state should be written to the file. Since writing to disk would be
> very expensive if it's done every time the state updates, try to only write to
> disk every 5s. Don't write to disk if nothing has changed.
> ```
>
> ~~I hope to implement in in the future.~~

Now there is a special thread for writing down the current state to a file
called `dummy`. However, it doesn't check if the state has been updated or not
_yet_.

#### 10 minutes later
It works now too.

- Client implementation is in [main.rs](./src/main.rs)
- Server implementation is in [server.rs](./src/server.rs)

- There is extra server implementation in TypeScript I wrote to test out the
  client with basic server, its content is in [serve.ts](./src/serve.ts)

## How to run.

### Building

You will need to compile the client using `cargo` or `nix`(just a wrapper)
because it has a dependency for serializing.

```bash
nix build
# or
cargo build
```

Just compile the `server.rs` with `rustc`.

```bash
rustc server.rs
```

Optionally, copy the executable files to the `bin/` folder:

```bash
cp result/bin/task-rs bin/ # built fix nix
cp ./server bin/
```

### Running

Client:

```bash
cat data.json | bin/client
```

Server:

```bash
bin/server 3000 3001 3002
```

### Have fun, make cool things.
