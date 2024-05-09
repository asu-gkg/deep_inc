import torch
import timeit
import deep_inc


inc = deep_inc.IncHandle()
inc.init_process_group(1, 2)
def perform_operation(dim):
    # 生成一个随机数tensor
    x = torch.randn(dim)
    # 执行一个简单的操作，这里是求和
    return inc._all_reduce(x, 'reduce_op_sum')

# 定义不同的tensor维度
dimensions = [64, 128, 256]

# 测量每个维度的平均执行时间
for dim in dimensions:
    time_taken = timeit.timeit('perform_operation(dim)', globals=globals(), number=100)
    print(f"Average time for dimension {dim}: {time_taken / 100:.6f} seconds")




