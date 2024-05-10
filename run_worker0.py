import deep_inc
import torch

inc = deep_inc.IncHandle()
inc.init_process_group(0, 2)

data = [1]
x = torch.tensor(data)

inc._all_reduce(x, 'reduce_op_sum')

import os
import torch
import torch.distributed as dist
import timeit

os.environ['MASTER_ADDR'] = '127.0.0.1'
os.environ['MASTER_PORT'] = '29500'

dist.init_process_group("gloo", rank=0, world_size=2)


def perform_operation(dim):
    x = torch.randn(dim)
    y = inc._all_reduce(x, 'reduce_op_sum')
    dist.all_reduce(x, op=dist.ReduceOp.SUM)
    # if y != x:
    print(f"y: {y}, x: {x}")
    return x


# 定义不同的tensor维度
# dimensions = [64, 128, 256, 512]
dimensions = [1, 2, 4, 8]


for dim in dimensions:
    time_taken = timeit.timeit('perform_operation(dim)', globals=globals(), number=100)
    print(f"Average time for dimension {dim}: {time_taken / 100:.6f} seconds")
