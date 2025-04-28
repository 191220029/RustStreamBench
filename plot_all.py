import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os
import math
import numpy as np

# 设置绘图风格
sns.set_style("whitegrid")

# 读取所有CSV文件
benchmarks = ['bzip2', 'eye-detector', 'image-processing', 'micro-bench']
data_frames = []

for bench in benchmarks:
    csv_path = os.path.join(bench, 'data.csv')
    if os.path.exists(csv_path):
        df = pd.read_csv(csv_path)
        df['Benchmark'] = bench
        data_frames.append(df)

# 合并所有数据
all_data = pd.concat(data_frames, ignore_index=True)

# 获取所有workload
workloads = all_data['Workload'].unique()
n_workloads = len(workloads)

# 计算子图的行数和列数
n_cols = 3  # 每行3个子图
n_rows = math.ceil(n_workloads / n_cols)

# 创建主图
fig, axes = plt.subplots(n_rows, n_cols, figsize=(20, 5*n_rows))
fig.suptitle('Performance Comparison Across Different Thread Counts', fontsize=16, y=1.02)

# 如果只有一个子图，将axes转换为二维数组
if n_workloads == 1:
    axes = np.array([axes]).reshape(1, 1)

# 定义颜色方案
colors = {
    'DAGRS': '#1f77b4',      # 蓝色
    'PIPELINER': '#ff7f0e',  # 橙色
    'RAYON': '#2ca02c',      # 绿色
    'SEQUENTIAL': '#d62728', # 红色
    'TOKIO': '#9467bd'       # 紫色
}

# 为每个workload创建子图
for idx, workload in enumerate(workloads):
    row = idx // n_cols
    col = idx % n_cols
    ax = axes[row, col]
    
    # 筛选当前workload的数据
    workload_data = all_data[all_data['Workload'] == workload]
    
    # 获取对应的benchmark名称
    benchmark = workload_data['Benchmark'].iloc[0]
    
    # 获取所有框架
    frameworks = workload_data['Framework'].unique()
    
    # 为每个框架绘制折线图
    for framework in frameworks:
        framework_data = workload_data[workload_data['Framework'] == framework]
        # 按NThread排序
        framework_data = framework_data.sort_values('NThread')
        # 计算每个线程数的平均执行时间
        avg_times = framework_data.groupby('NThread')['ExecutionTime(s)'].mean()
        # 过滤掉执行时间小于0的数据点
        valid_data = avg_times[avg_times > 0.1]
        if not valid_data.empty:  # 只有当有有效数据时才绘制
            color = colors.get(framework, None)  # 获取框架对应的颜色
            ax.plot(valid_data.index, valid_data.values, marker='o', label=framework, 
                    linewidth=2, color=color)
    
    # 设置子图属性
    ax.set_title(f'{benchmark} - {workload}', fontsize=12)
    ax.set_xlabel('Thread Count (NThread)', fontsize=10)
    ax.set_ylabel('Average Execution Time (s)', fontsize=10)
    ax.grid(True, linestyle='--', alpha=0.7)
    
    # 设置x轴刻度为整数
    ax.set_xticks(workload_data['NThread'].unique())
    
    # 添加图例
    ax.legend(fontsize=8)

# 如果子图数量不是3的倍数，隐藏最后一个空子图
if n_workloads % n_cols != 0:
    for idx in range(n_workloads, n_rows * n_cols):
        row = idx // n_cols
        col = idx % n_cols
        axes[row, col].axis('off')

# 调整布局
plt.tight_layout()

# 保存图表
output_dir = 'plots'
os.makedirs(output_dir, exist_ok=True)
plt.savefig(os.path.join(output_dir, 'all_workloads_performance.png'), dpi=300, bbox_inches='tight')
plt.close()

print("Chart has been generated and saved in the plots directory")