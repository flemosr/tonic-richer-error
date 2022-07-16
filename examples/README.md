# Examples

## Richer Error
Both clients and both servers do the same thing, but using the two different
approaches. Run **one of** the servers in one terminal, and then run the clients
in another.

### Client using ErrorDetails struct

```bash
$ cargo run --bin richer-error-client
```

### Client using standard messages vector

```bash
$ cargo run --bin richer-error-client-vec
```

### Server using ErrorDetails struct

```bash
$ cargo run --bin richer-error-server
```

### Server using standard messages vector

```bash
$ cargo run --bin richer-error-server-vec
```