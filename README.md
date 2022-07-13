# Tonic Richer Error
Assets for implementation of the gRPC Richer Error Model with tonic.

This crate introduces the `WithErrorDetails` trait and implements it in
`tonic::Status`, allowing the implementation of the [gRPC Richer Error Model]
with [tonic] in a convenient way.

[![Crates.io](https://img.shields.io/crates/v/tonic-richer-error)](https://crates.io/crates/tonic-richer-error)
[![Documentation](https://docs.rs/tonic-richer-error/badge.svg)](https://docs.rs/tonic-richer-error)
[![Crates.io](https://img.shields.io/crates/l/tonic-richer-error)](LICENSE)

[Examples] | [Website] | [Docs] 

## Usage
The `WithErrorDetails` trait adds associated functions to `tonic::Status` that
can be used on the server side to create a status with error details, that can
then be returned to the gRPC client. Moreover, the trait also adds methods
to `tonic::Status` that can be used by a tonic client to extract error details,
and handle them with ease.

## Getting Started
```
[dependencies]
tonic = "0.7"
tonic-richer-error = "0.2"
```

## Examples

The examples bellow cover a basic use case. More complete server and client
implementations can be found at the [examples] directory.

### Server Side: Generating `tonic::Status` with an `ErrorDetails` struct
```rust
use tonic::{Code, Status};
use tonic_richer_error::{ErrorDetails, WithErrorDetails};

// ...
// (inside a gRPC server endpoint that returns Result<Response<PbRes>, Status>)

// Create empty ErrorDetails struct
let mut err_details = ErrorDetails::new();

// Add error details conditionally
if some_condition {
    err_details.add_bad_request_violation(
        "field_a",
        "description of why the field_a is invalid"
    );
}

if other_condition {
    err_details.add_bad_request_violation(
        "field_b",
        "description of why the field_b is invalid",
    );
}

// Check if any error details were set and return error status if so
if err_details.has_bad_request_violations() {

    // Add additional error details if necessary
    err_details
        .add_help_link("description of link", "https://resource.example.local")
        .set_localized_message("en-US", "message for the user");

    let status = Status::with_error_details(
        Code::InvalidArgument,
        "bad request",
        err_details,
    )
    .unwrap();

    return Err(status);
}

// Handle valid request

// ...
```

### Client Side: Extracting an `ErrorDetails` struct from `tonic::Status`
```rust
use tonic::{Response, Status};
use tonic_richer_error::{WithErrorDetails};

// ...

// Where req_result was returned by a tonic::Client endpoint method
fn handle_request_result<T>(req_result: Result<Response<T>, Status>) {
    match req_result {
        Ok(_) => {
            // Handle successful response
        },
        Err(status) => {
            let err_details = status.get_error_details().unwrap();
            if let Some(bad_request) = err_details.bad_request {
                // Handle bad_request details
            }
            if let Some(help) = err_details.help {
                // Handle help details
            }
            if let Some(localized_message) = err_details.localized_message {
                // Handle localized_message details
            }
        }
    };
}
```

### Setup different standard error messages
Multiple examples are provided at the [ErrorDetails section] of the docs.
Instructions about how to setup the messages fields correctly are provided at
[standard error messages].

### Alternative `tonic::Status` associated functions and methods
In the [WithErrorDetails section] of the docs, an alternative way of interacting
with `tonic::Status` is presented, using vectors of standard error messages
directly ([::with_error_details_vec], [.get_error_details_vec]). This approach
can provide more control over the final error details vector if necessary, and
it is also presented at the [examples] directory.
Besides that, multiple examples with alternative error detail extration methods
are provided in the docs, which can be useful if only one kind of error detail
is being used, for example: [.get_details_bad_request].

## License

This project is licensed under the [MIT license](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, shall be licensed as MIT, without any additional terms or
conditions.

[gRPC Richer Error Model]: https://www.grpc.io/docs/guides/error/
[tonic]: https://github.com/hyperium/tonic
[Examples]: https://github.com/flemosr/tonic-richer-error/tree/main/examples
[Website]: https://github.com/flemosr/tonic-richer-error
[Docs]: https://docs.rs/tonic-richer-error/0.2.1/tonic_richer_error/
[examples]: https://github.com/flemosr/tonic-richer-error/tree/main/examples
[standard error messages]: https://github.com/googleapis/googleapis/blob/master/google/rpc/error_details.proto
[ErrorDetails section]: https://docs.rs/tonic-richer-error/0.2.1/tonic_richer_error/struct.ErrorDetails.html
[WithErrorDetails section]: https://docs.rs/tonic-richer-error/0.2.1/tonic_richer_error/trait.WithErrorDetails.html
[::with_error_details_vec]: https://docs.rs/tonic-richer-error/0.2.1/tonic_richer_error/trait.WithErrorDetails.html#tymethod.with_error_details_vec
[.get_error_details_vec]: https://docs.rs/tonic-richer-error/0.2.1/tonic_richer_error/trait.WithErrorDetails.html#tymethod.get_error_details_vec
[.get_details_bad_request]: https://docs.rs/tonic-richer-error/0.2.1/tonic_richer_error/trait.WithErrorDetails.html#tymethod.get_details_bad_request