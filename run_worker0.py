import deep_inc
import torch

inc = deep_inc.IncHandle()
inc.init_process_group(0, 2)

data = [1]
x = torch.tensor(data)


inc._all_reduce(x, 'reduce_op_sum')

