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

``` 
export LIBTORCH=/mnt/c/Users/asu/RustroverProjects/libtorch/linux/libtorch-cxx11-abi-shared-with-deps-2.2.0+cu118/libtorch
export LIBTORCH_INCLUDE=/mnt/c/Users/asu/RustroverProjects/libtorch/linux/libtorch-cxx11-abi-shared-with-deps-2.2.0+cu118/libtorch
export LIBTORCH_LIB=/mnt/c/Users/asu/RustroverProjects/libtorch/linux/libtorch-cxx11-abi-shared-with-deps-2.2.0+cu118/libtorch
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH
```

But if you have libtorch 2.2.0+cu118, your pytorch version need to be same as libtorch, use this:
``` 
conda install pytorch==2.2.0 torchvision==0.17.0 torchaudio==2.2.0 pytorch-cuda=12.1 -c pytorch -c nvidia
```

2. Add tch

``` 
cargo add tch
```


3. Install etcd
之后把etcd搞到一个公开的地址上，后面就写死这个地址好了。
``` 
ETCD_VER=v3.4.32

# choose either URL
GOOGLE_URL=https://storage.googleapis.com/etcd
GITHUB_URL=https://github.com/etcd-io/etcd/releases/download
DOWNLOAD_URL=${GOOGLE_URL}

rm -f /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz
rm -rf /tmp/etcd-download-test && mkdir -p /tmp/etcd-download-test

curl -L ${DOWNLOAD_URL}/${ETCD_VER}/etcd-${ETCD_VER}-linux-amd64.tar.gz -o /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz
tar xzvf /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz -C /tmp/etcd-download-test --strip-components=1
rm -f /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz

/tmp/etcd-download-test/etcd --version
/tmp/etcd-download-test/etcdctl version
```

#### start a local etcd server
``` 
/tmp/etcd-download-test/etcd
```


#### set etcd key
``` 
/tmp/etcd-download-test/etcdctl --endpoints=localhost:2379 put foo bar
/tmp/etcd-download-test/etcdctl --endpoints=localhost:2379 get foo
```

#### view key, value
``` 
/tmp/etcd-download-test/etcdctl get "" --prefix               # 显示所有键和值
```

#### etcd clear
``` 
/tmp/etcd-download-test/etcdctl del --from-key ''
```



#### windows wsl
``` 
source iso_env/bin/activate
export LIBTORCH_USE_PYTORCH=1
export PYO3_PYTHON=/usr/bin/python3
export LD_LIBRARY_PATH=/mnt/c/Users/asu/RustroverProjects/inc/iso_env/lib/python3.12/site-packages/torch/lib/:$LD_LIBRARY_PATH
```