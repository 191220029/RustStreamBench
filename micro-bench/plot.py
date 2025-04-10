import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# 读取CSV文件
df = pd.read_csv('data.csv')

# 计算每个框架在每个workload下的平均执行时间
avg_times = df.groupby(['Framework', 'Workload'])['ExecutionTime(s)'].mean().reset_index()

# 设置中文字体
plt.rcParams['font.sans-serif'] = ['Times New Roman']  # 用来正常显示中文标签
plt.rcParams['axes.unicode_minus'] = False  # 用来正常显示负号

# 为每个workload创建单独的子图
workloads = avg_times['Workload'].unique()
n_workloads = len(workloads)
fig, axes = plt.subplots(n_workloads, 1, figsize=(15, 5*n_workloads))
# fig.suptitle('execution time comparison', fontsize=16)

for idx, workload in enumerate(workloads):
    workload_data = avg_times[avg_times['Workload'] == workload]
    
    # 创建条形图
    if n_workloads == 1:
        sns.barplot(data=workload_data, x='Framework', y='ExecutionTime(s)', ax=axes)
        axes.set_xlabel(f'{workload}')
    else:
        sns.barplot(data=workload_data, x='Framework', y='ExecutionTime(s)', ax=axes[idx])
        # axes[idx].set_title(f'{workload} average execution time')
        axes[idx].set_xlabel(f'{workload}')
        # 旋转x轴标签以防重叠
        axes[idx].tick_params(axis='x', rotation=45)
    # axes[idx].set_ylabel('execution time (s)')
    
    
    # 添加数值标签
    for i, v in enumerate(workload_data['ExecutionTime(s)']):
        if n_workloads != 1:
            axes[idx].text(i, v, f'{v:.2f}s', ha='center', va='bottom')
        else:
            axes.text(i, v, f'{v:.2f}s', ha='center', va='bottom')

# 调整子图之间的间距
plt.tight_layout()

# 保存图表
plt.savefig('micro-bench_performance_comparison.png', dpi=300, bbox_inches='tight')
print("图表已保存为 micro-bench_performance_comparison.png") 