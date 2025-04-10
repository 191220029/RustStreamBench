# import pandas as pd
# import matplotlib.pyplot as plt
# import seaborn as sns
# import os

# def plot_performance(directory):
#     # 构建数据文件路径
#     data_file = os.path.join(directory, 'data.csv')
#     output_file = os.path.join(directory, f'{os.path.basename(directory)}_performance_comparison.png')
    
#     # 检查数据文件是否存在
#     if not os.path.exists(data_file):
#         print(f"警告: {data_file} 不存在，跳过该目录")
#         return
    
#     # 读取CSV文件
#     df = pd.read_csv(data_file)
    
#     # 计算每个框架在每个workload下的平均执行时间
#     avg_times = df.groupby(['Framework', 'Workload'])['ExecutionTime(s)'].mean().reset_index()
    
#     # 自定义Framework的顺序，将sequential放在最后
#     frameworks = avg_times['Framework'].unique()
#     custom_order = sorted([f for f in frameworks if f != 'sequential']) + ['sequential']
#     avg_times['Framework'] = pd.Categorical(avg_times['Framework'], categories=custom_order, ordered=True)
    
#     # 设置中文字体
#     plt.rcParams['font.sans-serif'] = ['Times New Roman']  # 用来正常显示中文标签
#     plt.rcParams['axes.unicode_minus'] = False  # 用来正常显示负号
    
#     # 设置图表大小
#     plt.figure(figsize=(15, 8))
    
#     # 创建并排条形图
#     sns.barplot(data=avg_times, x='Workload', y='ExecutionTime(s)', hue='Framework', width=0.6)
    
#     # 设置图表标题和标签
#     plt.title(f'Performance Comparison - {os.path.basename(directory)}')
#     plt.xlabel('Workload')
#     plt.ylabel('Execution Time (s)')
    
#     # 旋转x轴标签以防重叠
#     plt.xticks(rotation=45)
    
#     # 添加图例
#     plt.legend(title='Framework', bbox_to_anchor=(1.05, 1), loc='upper left')
    
#     # 调整布局
#     plt.tight_layout()
    
#     # 保存图表
#     plt.savefig(output_file, dpi=300, bbox_inches='tight')
#     print(f"图表已保存为 {output_file}")
#     plt.close()

# # 要处理的目录列表
# directories = ['bzip2', 'eye-detector', 'image-processing', 'micro-bench']

# # 为每个目录生成图表
# for directory in directories:
#     print(f"\n处理目录: {directory}")
#     plot_performance(directory)

# print("\n所有图表生成完成！")

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os

def load_and_process_data(directory):
    # 构建数据文件路径
    data_file = os.path.join(directory, 'data.csv')
    
    # 检查数据文件是否存在
    if not os.path.exists(data_file):
        print(f"警告: {data_file} 不存在，跳过该目录")
        return None
    
    # 读取CSV文件
    df = pd.read_csv(data_file)
    
    # 计算每个框架在每个workload下的平均执行时间
    avg_times = df.groupby(['Framework', 'Workload'])['ExecutionTime(s)'].mean().reset_index()
    
    # 自定义Framework的顺序，将sequential放在最后
    frameworks = avg_times['Framework'].unique()
    custom_order = sorted([f for f in frameworks if f != 'sequential']) + ['sequential']
    avg_times['Framework'] = pd.Categorical(avg_times['Framework'], categories=custom_order, ordered=True)
    
    return avg_times

# 要处理的目录列表
directories = ['bzip2', 'eye-detector', 'image-processing', 'micro-bench']

# 设置中文字体
plt.rcParams['font.sans-serif'] = ['Times New Roman']  # 用来正常显示中文标签
plt.rcParams['axes.unicode_minus'] = False  # 用来正常显示负号

# 创建子图
fig, axes = plt.subplots(2, 2, figsize=(20, 15))
fig.suptitle('Performance Comparison Across RustStream Benchmarks', fontsize=16)

# 为每个目录创建图表
for idx, directory in enumerate(directories):
    print(f"\n处理目录: {directory}")
    data = load_and_process_data(directory)
    if data is not None:
        row = idx // 2
        col = idx % 2
        ax = axes[row, col]
        
        # 创建并排条形图
        sns.barplot(data=data, x='Workload', y='ExecutionTime(s)', hue='Framework', width=0.6, ax=ax)
        
        # 设置子图标题和标签
        ax.set_title(directory)
        ax.set_xlabel('Workload')
        ax.set_ylabel('Execution Time (s)')
        
        # 旋转x轴标签以防重叠
        ax.tick_params(axis='x', rotation=45)
        
        # 添加图例
        ax.legend(title='Framework', bbox_to_anchor=(1.05, 1), loc='upper left')

# 调整布局
plt.tight_layout()

# 保存图表
output_file = 'ruststream_benchmarks_performance_comparison.png'
plt.savefig(output_file, dpi=300, bbox_inches='tight')
print(f"\n图表已保存为 {output_file}")
plt.close()