import numpy as np
from scipy.optimize import minimize
import sys
import tensorflow as tf
import h5py

def load_hdf5_mnist(file_path):
    with h5py.File(file_path, 'r') as hf:
        # 加载训练集
        train_images = hf['train/images'][:]
        train_labels = hf['train/labels'][:]
        
        # 加载测试集
        test_images = hf['test/images'][:]
        test_labels = hf['test/labels'][:]
        
    return (train_images, train_labels), (test_images, test_labels)

def preprocess(images):
    images_flat = images.reshape(images.shape[0], -1)  # 展平为 (n, 784)
    return np.insert(images_flat, 0, 1, axis=1)       # 添加偏置项 (n, 785)

(train_images, train_labels), (test_images, test_labels) =load_hdf5_mnist('/home/xiaolongfu/dagrs-perf/mnist.hdf5')
# 预处理训练集和测试集
X_train = preprocess(train_images)
y_train = train_labels.flatten()
X_test = preprocess(test_images)
y_test = test_labels.flatten()

def sigmoid(z):
    return 1/(1+np.exp(-z))

def regularized_cost(Theta,X,y,l):
    ThetaReg=Theta[1:]
    cost=(-y*np.log(sigmoid(X@Theta)))-(1-y)*np.log(1-sigmoid((X@Theta)))
    reg=(ThetaReg@ThetaReg)*l/(2*len(X))
    return np.mean(cost)+reg


def regularized_gradient(Theta,X,y,l):
    ThetaReg=Theta[1:]
    cost=(X.T@(sigmoid(X@Theta)-y))*(1/len(X))
    reg=np.concatenate([np.array([0]),(l/len(X))*ThetaReg])
    return cost+reg

def one_vs_all(X,y,l,i):
    Theta=np.zeros(X.shape[1])
    y_i=np.array([1 if labal==(i + 1) else 0 for labal in y])
    ret=minimize(fun=regularized_cost, x0=Theta,args=(X,y_i,l),method='TNC',jac=regularized_gradient)
    return ret.x

argc = len(sys.argv)
argv = sys.argv

X, y = X_train, y_train
i = int(argv[2])
theta = one_vs_all(X, y, 1, i)

s = np.array2string(theta, max_line_width=1 << 31)
with open(f"theta_{i}.txt", "w") as f:
    f.write(s)
print(f"theta_{i}.txt")