# DeepINC

## functions
1. The most important collective communication operation in distributed deep learning is all-reduce. 
Need to support cloud-scale all-reduce operation.

## potential optimization
1. Use **ebpf**/container/virtualization technology to support multiple tenants scenario.
2. Use rdma/erpc/zero-copy Serialization to speedup 