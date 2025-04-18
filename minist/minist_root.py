import numpy as np
import tensorflow as tf
import sys
import os
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

(train_images, train_labels), (test_images, test_labels) =load_hdf5_mnist('/home/xiaolongfu/dagrs-perf/mnist.hdf5')
def preprocess(images):
    images_flat = images.reshape(images.shape[0], -1)  # 展平为 (n, 784)
    return np.insert(images_flat, 0, 1, axis=1)       # 添加偏置项 (n, 785)

# 预处理训练集和测试集
X_train = preprocess(train_images)
y_train = train_labels.flatten()
X_test = preprocess(test_images)
y_test = test_labels.flatten()

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def predict(X,All_Theta):
    h=sigmoid(X@All_Theta.T)
    h_argmax=np.argmax(h,axis=1)
    h_argmax=h_argmax+1
    return h_argmax

def main():
    np.set_printoptions(threshold=sys.maxsize)

    argv = sys.argv
    X, y = X_train, y_train
    
    # Load theta values for each class from the provided files
    All_Theta = np.zeros((10, X.shape[1]))  # 10 classes, with the same number of features

    # for i in range(0, 10):
    #     # Load theta for class (i+1)
    #     s = argv[2+i].split(":")
    #     j, s = s[0], s[1]
    #     s = s.strip("\[\]\"'")
    #     All_Theta[int(j)] = np.fromstring(s, sep=" ", dtype=float)
    
    for i in range(0, 10):
        # Load theta for class (i+1) from the file
        file = f"theta_{i}.txt"
        with open(file, "r") as f:
            s = f.read().strip("[] ")
            All_Theta[i] = np.fromstring(s, sep=" ", dtype=float)
        os.remove(file)

    # Predict the class using the provided theta values
    y_predict = predict(X, All_Theta)

    # Calculate accuracy
    y_predict=predict(X, All_Theta)
    accuracy=np.mean(y_predict==y)
    print(f"train acc: {accuracy * 100:.2f}%\n")

    
    y_predict=predict(X_test, All_Theta)
    accuracy=np.mean(y_predict==y_test)
    print(f"test acc: {accuracy * 100:.2f}%\n")

if __name__ == "__main__":
    main()