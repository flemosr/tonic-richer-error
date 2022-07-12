# Examples

## BadRequest 
Both clients and both servers do the same thing, but using the two different
approaches. Run **one of** the servers in one terminal, and then run the clients
in another.

### Client using ErrorDetails struct

```
$   cargo run --bin badreq-client
```

### Client using standard messages vector

```
$   cargo run --bin badreq-client-vec
```

### Server using ErrorDetails struct

```
$   cargo run --bin badreq-server
```

### Server using standard messages vector

```
$   cargo run --bin badreq-server-vec
```