import subprocess

output = subprocess.check_output(
            ["go", "run", "sklearn.go"],
            universal_newlines=True,
            cwd="/home/xiaolongfu/dagrs-perf/go-perf"
        )

print(output)