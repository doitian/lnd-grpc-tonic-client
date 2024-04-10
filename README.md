# lnd-grpc-tonic-client

Rust [lnd] client using [tonic] and [tonic-openssl].

[lnd]: https://github.com/lightningnetwork/lnd
[tonic]: https://github.com/hyperium/tonic
[tonic-openssl]: https://github.com/LucioFranco/tonic-openssl

Lnd requires TLS certificate to secure the communication and Macaroon for authorization (see [details](https://docs.lightning.engineering/lapps/guides/add-features/connect-to-lnd)). Lnd auto-generates a self-signed TLS certificate, however, tonic does not work well with self-signed certificate.

This crate is based on [this suggestion](https://github.com/hyperium/tonic/issues/459#issuecomment-696793643) and [this example](https://github.com/LucioFranco/tonic-openssl/blob/master/example/src/client2.rs).

Proto files are copied from lnd repository and the generated files can be found in the src directory.

See examples in the examples directory.
