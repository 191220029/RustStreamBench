import luigi
import subprocess
import time
import os

class TrainOVAModel(luigi.Task):
    """训练单个OVA模型"""
    model_id = luigi.IntParameter()  # 模型ID (1-based)
    script_path = luigi.Parameter(default='./minist_i.py')

    def requires(self):
        return [] 

    def run(self):
        # 调用外部脚本
        subprocess.check_output(
            ['python', '-B', self.script_path, "", str(self.model_id)], 
            universal_newlines=True,
            # cwd="/home/xiaolongfu/dagrs-perf/liugi-perf"
        )
        print(f"Trainer {self.model_id} finished.")
        # 假设脚本输出模型参数到文件（需与实际脚本逻辑一致）
        with open(self.output().path, 'w') as f:
            f.write(f"Model {self.model_id} trained")

    def output(self):
        return luigi.LocalTarget(f'model_{self.model_id}.done')  # 标记文件


class AggregateResults(luigi.Task):
    """汇总所有模型结果"""
    script_path = luigi.Parameter(default='./minist_root.py')

    def requires(self):
        # 依赖所有OVA模型任务
        return [TrainOVAModel(model_id=i) for i in range(0, 10)]
        # return [TrainOVAModel(model_id=i) for i in range(0, 10)]

    def run(self):
        # 执行汇总脚本
        print(f"Start verifying.")
        output = subprocess.check_output(
            ['python', '-B', self.script_path, ""],
            universal_newlines=True,
            # cwd="/home/xiaolongfu/dagrs-perf/liugi-perf"
        )
        print(output)
    
if __name__ == '__main__':
    
    os.system("rm *.done")

    start_time = time.time()

    # 运行Luigi任务链
    luigi.build([AggregateResults()], local_scheduler=True, workers=10)

    # 性能监控
    print(f'Total time: {time.time() - start_time} s')