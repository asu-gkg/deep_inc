import os
import torch
import torch.distributed as dist
import timeit


os.environ['MASTER_ADDR'] = '127.0.0.1'
os.environ['MASTER_PORT'] = '29500'

dist.init_process_group("gloo", rank=1, world_size=2)


def perform_operation(dim):
    # 生成一个随机数tensor
    x = torch.randn(dim)
    # 执行一个简单的操作，这里是求和
    return dist.all_reduce(x, op=dist.ReduceOp.SUM)

# 定义不同的tensor维度
dimensions = [64, 128, 256, 512]

# 测量每个维度的平均执行时间
for dim in dimensions:
    time_taken = timeit.timeit('perform_operation(dim)', globals=globals(), number=100)
    print(f"Average time for dimension {dim}: {time_taken / 100:.6f} seconds")
