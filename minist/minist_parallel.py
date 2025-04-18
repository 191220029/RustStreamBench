from joblib import Parallel, delayed
import h5py
import subprocess
import numpy as np
import time
import tracemalloc

def load_hdf5_mnist(file_path):
    with h5py.File(file_path, 'r') as hf:
        train_images = hf['train/images'][:]
        train_labels = hf['train/labels'][:]
        test_images = hf['test/images'][:]
        test_labels = hf['test/labels'][:]
    return (train_images, train_labels), (test_images, test_labels)

def one_vs_all_parallel(K, feature_dim, n_jobs=-1):
    """并行调用外部脚本训练所有OVA模型"""
    All_Theta = np.zeros((K, feature_dim))
    
    def run_external_script(i):
        i = i - 1
        """执行minist_i.py并解析输出"""
        output = subprocess.check_output(
            ['python', "-B", '/home/xiaolongfu/dagrs-perf/dagrs-NJU-fxl/examples/dagrs-sklearn/examples/minist_i.py', "", str(i)],
            universal_newlines=True
        )
        # print("MINIST_" + str(i) +  "finished")
        return i, output.strip()  # 转换为0-based索引

    # 并行执行所有任务
    Parallel(n_jobs=n_jobs, backend='threading')(
        delayed(run_external_script)(i) for i in range(1, K+1)
    )

# 数据预处理函数
def preprocess(images):
    images_flat = images.reshape(images.shape[0], -1)
    return np.insert(images_flat, 0, 1, axis=1)  # 添加偏置项

# 预测函数
def predict(X, All_Theta):
    h = 1 / (1 + np.exp(-X @ All_Theta.T))
    return np.argmax(h, axis=1) + 1  # 1-based类别

if __name__ == "__main__":
    tracemalloc.start()  # 启动内存跟踪
    start = time.time()
    # 加载数据并预处理
    (train_images, train_labels), (test_images, test_labels) = load_hdf5_mnist('mnist.hdf5')
    X_train = preprocess(train_images)
    y_train = train_labels.flatten()
    X_test = preprocess(test_images)
    y_test = test_labels.flatten()

    # 训练模型
    All_Theta = one_vs_all_parallel(
        K=10,
        feature_dim=X_train.shape[1],
        n_jobs=-1
    )
    output = subprocess.check_output(
            ['python', "-B", '/home/xiaolongfu/dagrs-perf/dagrs-NJU-fxl/examples/dagrs-sklearn/examples/minist_root.py', ""],
            universal_newlines=True
        )
    
    print(output)
    
    time_end=time.time()
    print('time cost',time_end-start,'s')
    
    # 获取并打印内存峰值
    _current, peak = tracemalloc.get_traced_memory()
    print(f"Peak memory usage: {peak / 1048576} MB")
    tracemalloc.stop()  # 停止跟踪

    