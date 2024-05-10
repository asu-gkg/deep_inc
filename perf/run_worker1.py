import torch
import timeit
import deep_inc


inc = deep_inc.IncHandle()
inc.init_process_group(1, 2)
def perform_operation(dim):
    x = torch.randn(dim)
    return inc._all_reduce(x, 'reduce_op_sum')

dimensions = [64, 128, 256, 512]

for dim in dimensions:
    time_taken = timeit.timeit('perform_operation(dim)', globals=globals(), number=100)
    print(f"Average time for dimension {dim}: {time_taken / 100:.6f} seconds")



