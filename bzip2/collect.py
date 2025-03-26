import csv
import os
import re

LOG_DIR = "logs"
OUTPUT_CSV = "data.csv"

filename_pattern = re.compile(
    r"^(?P<framework>\w+!_)_"
    r"(?P<operation_workload>[\w\.-]+)"
    r"_iter(?P<iteration>\d+)"
)

# 存储所有数据行的临时列表
rows = []

for filename in os.listdir(LOG_DIR):
    if not filename.endswith('.log'):
        continue
    
    match = filename_pattern.search(filename)
    if not match:
        continue
    
    framework = match.group('framework').upper()
    operation_workload = match.group('operation_workload')
    iteration = int(match.group('iteration'))  # 转换为整数
    
    # 处理特殊文件名格式
    if ".log_" in operation_workload:
        operation_workload = operation_workload.replace(".log_", "_")
    
    # 读取执行时间
    with open(os.path.join(LOG_DIR, filename), 'r') as f:
        lines = f.readlines()
        if not lines:
            continue
            
        last_line = lines[-1].strip()
        if "Execution time:" not in last_line:
            continue
        
        try:
            time = float(last_line.split()[-2])
        except (IndexError, ValueError):
            continue
    
    # 收集数据（保持列顺序：Framework, Workload, Iteration, Time）
    rows.append((
        framework,
        operation_workload.replace('_', ' ').title(),
        iteration,
        f"{time:.6f}"
    ))

# 自定义排序函数
def sort_key(row):
    """ 
    排序优先级：
    1. 框架名称（字母顺序）
    2. 工作负载名称（字母顺序） 
    3. 迭代次数（数字大小）
    """
    return (row[0], row[1], row[2])

# 执行多级排序
sorted_rows = sorted(rows, key=sort_key)

# 写入排序后的CSV
with open(OUTPUT_CSV, 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['Framework', 'Workload', 'Iteration', 'ExecutionTime(s)'])
    
    for row in sorted_rows:
        # 将迭代次数转换回字符串写入（如果需要保持原始格式）
        modified_row = list(row)
        modified_row[2] = str(modified_row[2])  # 转换迭代次数为字符串
        writer.writerow(modified_row)

print(f"saving data to {OUTPUT_CSV}")