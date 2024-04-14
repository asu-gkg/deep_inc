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

## Add rs-tch

1. Get libtorch from the PyTorch website download section and extract the content of the zip file. Set the path.

``` 
export LIBTORCH=/path/to/libtorch/
export LIBTORCH_INCLUDE=/path/to/libtorch/
export LIBTORCH_LIB=/path/to/libtorch/
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PAT
```

My solution is 

``` 
export LIBTORCH=/mnt/c/Users/asu/RustroverProjects/libtorch/linux/libtorch-cxx11-abi-shared-with-deps-2.2.0+cu118/libtorch
export LIBTORCH_INCLUDE=/mnt/c/Users/asu/RustroverProjects/libtorch/linux/libtorch-cxx11-abi-shared-with-deps-2.2.0+cu118/libtorch
export LIBTORCH_LIB=/mnt/c/Users/asu/RustroverProjects/libtorch/linux/libtorch-cxx11-abi-shared-with-deps-2.2.0+cu118/libtorch
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH
```


2. Add tch

``` 
cargo add tch
```