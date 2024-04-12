# DeepINC

## functions
1. The most important collective communication operation in distributed deep learning is all-reduce. 
Need to support cloud-scale all-reduce operation.

## potential optimization
1. Use **ebpf**/container/virtualization technology to support multiple tenants scenario.
2. Use rdma/erpc/zero-copy Serialization to speedup 

## Build Environment

TODO: Add description here.

## TODO

1. Implement add rpc
```
    make test_sim
```

Now log shows:
```
---- config::lib::tests::test_add_rpc stdout ----
I'm No. 0 server. About me: Mutex { data: <locked> }
Client sends a msg to server
recv add req, a: 1, b: 0 # wrong, actually a=1, b=1.
thread 'config::lib::tests::test_add_rpc' panicked at 'called `Option::unwrap()` on a `None` value', src/server/server.rs:33:52
stack backtrace:
```

Please fix that. Let client receive right result: sum = 2.