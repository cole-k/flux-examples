import argparse
import os
import shutil
import subprocess
import json
from enum import Enum
import timeit

class Suites(Enum):
    LR = "lr"
    PRUSTI = "prusti"
    BOTH = "both"

benchmark_list_prusti = [
    # "lib/vecwrapper.rs",
    # "bcopy.rs",
    # "bsearch.rs",
    # "dotprod.rs",
    "fft.rs",
    # "heapsort.rs",
    # "kmp.rs",
    # "knuth_shuffle.rs",
    # "min_index.rs",
    # "lib/matwrapper.rs",
    # "simplex.rs",
    # "kmeans.rs",
]

benchmark_list_lr = [
    "lib/rvec.rs",
    "bcopy.rs",
    "bsearch.rs",
    "dotprod.rs",
    "fft.rs",
    "heapsort.rs",
    "kmp.rs",
    "knuth_shuffle.rs",
    "min_index.rs",
    "lib/rmat.rs",
    "simplex.rs",
    "kmeans.rs",
]

# def prusti_run():
#     print(current_benchmark)
#     print(r"c:\Users\adam_\AppData\Roaming\Code\User\globalStorage\viper-admin.prusti-assistant\prustiTools\LatestRelease\prusti\prusti-rustc.exe -Pserver_address=\"MOCK\" -Coverflow-checks=off --crate-type=lib " + current_benchmark)
#     proc = subprocess.run(
#         r"c:\Users\adam_\AppData\Roaming\Code\User\globalStorage\viper-admin.prusti-assistant\prustiTools\LatestRelease\prusti\prusti-rustc.exe -Pserver_address=\"MOCK\" -Coverflow-checks=off --crate-type=lib " + current_benchmark,
#         shell=True,
#         capture_output=True,
#     )
#     stdout = proc.stdout.decode("utf-8", "backslashreplace")
#     stderr = proc.stderr.decode("utf-8", "backslashreplace")
#     #print(stdout)
#     if proc.returncode != 0:
#         print("Verifying " + current_benchmark + " failed...")
#         print(stderr)

def run_benchmark(benchmark):
    proc = subprocess.run(
        "python count_lines.py " + benchmark,
        shell=True,
        capture_output=True,
    )
    stdout = proc.stdout.decode("utf-8", "backslashreplace")
    stderr = proc.stderr.decode("utf-8", "backslashreplace")
    #print(stdout)
    if proc.returncode != 0:
        print("Counting lines of " + benchmark + " failed...")
        print(stderr)
    
    print(stdout)
    counts = json.loads(stdout)

    print(benchmark)

    prusti_run = r"""
proc = subprocess.run(r'c:\Users\adam_\AppData\Roaming\Code\User\globalStorage\viper-admin.prusti-assistant\prustiTools\LatestRelease\prusti\prusti-rustc.exe -Pcheck_overflows=false -Coverflow-checks=off --crate-type=lib ' + "{}", shell=True, capture_output=True)
print(proc.stdout.decode("utf-8", "backslashreplace"))
if proc.returncode != 0:
    print("Verifying " + "{}" + " failed...")
    print(proc.stderr.decode("utf-8", "backslashreplace"))
""".format(benchmark, benchmark)

    print(prusti_run)


    t = timeit.Timer(stmt = prusti_run, setup = "import subprocess")
    print(t.repeat(repeat=3, number=1))

    return counts

def run_suite(suite, dir):
    stats = []
    if suite == Suites.LR:
        benchmark_list = benchmark_list_lr
        prefix = dir + "/lr/"
    elif suite == Suites.PRUSTI:
        benchmark_list = benchmark_list_prusti
        prefix = dir + "/prusti/"

    for benchmark in benchmark_list:
        benchmark_stats = run_benchmark(prefix + benchmark)
        stats.append((benchmark, benchmark_stats))

    return stats

def dump_csv(stats, suite, file):
    filename, ext = os.path.splitext(os.path.basename(file))
    if suite == Suites.LR:
        filename = filename + "_lr"
    elif suite == Suites.PRUSTI:
        filename = filename + "_prusti"
    
    output = os.path.dirname(file) + filename + ext

    print(output)

    with open(output, 'w') as file:
        print("Benchmark, LOC, Function Contracts, Contract Lines, Loop Invariants, Invariant Lines", file=file)

        for (benchmark, counts) in stats:
            print("{}, {}, {}, {}, {}, {}".format(benchmark, counts['lines'], counts['function_contracts'], counts['contract_lines'], counts['loop_invariants'], counts['invariant_lines']), file=file)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("directory", type=str, default=".")
    parser.add_argument("suites", type=Suites, default=Suites.BOTH, choices=list(Suites))
    parser.add_argument("output", type=str)
    args = parser.parse_args()

    if args.suites == Suites.BOTH or args.suites == Suites.LR:
        lr_stats = run_suite(Suites.LR, args.directory)
        dump_csv(lr_stats, Suites.LR, args.output)

    if args.suites == Suites.BOTH or args.suites == Suites.PRUSTI:
        prusti_stats = run_suite(Suites.PRUSTI, args.directory)
        dump_csv(prusti_stats, Suites.PRUSTI, args.output)