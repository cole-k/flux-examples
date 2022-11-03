import argparse
import os
import shutil
import subprocess
import json
from enum import Enum
import timeit
from statistics import mean

class Suites(Enum):
    FLUX = "flux"
    PRUSTI = "prusti"
    BOTH = "both"

benchmark_list_prusti = [
    "lib/vecwrapper.rs",
    "bcopy.rs",
    "bsearch.rs",
    "dotprod.rs",
    "fft.rs",
    "heapsort.rs",
    "kmp.rs",
    "knuth_shuffle.rs",
    "min_index.rs",
    "lib/matwrapper.rs",
    "simplex.rs",
    "kmeans.rs",
]

benchmark_list_flux = [
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

def run_benchmark(benchmark, suite, prusti_server_address, flux_path, prusti_rustc):
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
    
    #print(stdout)
    counts = json.loads(stdout)
    path = os.getcwd()

    print(benchmark)

    if suite == Suites.FLUX:
        
        benchmark_path = os.path.join(path, benchmark)
        verify = r"""
proc = subprocess.run(r'cargo run --release -- --crate-type=lib {}', shell=True, capture_output=True)
if proc.returncode != 0:
    print("Verifying {} with FLUX failed...")
    print(proc.stderr.decode("utf-8", "backslashreplace"))
""".format(benchmark_path, benchmark)
    elif suite == Suites.PRUSTI:
        verify = r"""
proc = subprocess.run(r'{} -Pcheck_overflows=false -Coverflow-checks=off --crate-type=lib -Pserver_address={} {}', shell=True, capture_output=True)
if proc.returncode != 0:
    print("Verifying {} with Prusti failed...")
    print(proc.stderr.decode("utf-8", "backslashreplace"))
""".format(prusti_rustc, prusti_server_address, benchmark, benchmark)

    if suite == Suites.FLUX:
        os.chdir(flux_path)

    t = timeit.Timer(stmt = verify, setup = "import subprocess")
    times = t.repeat(repeat=5, number=1)
    print(times)
    counts['time'] = round(mean(times),2)

    if suite == Suites.FLUX:
        os.chdir(path)

    return counts

def run_suite(suite, dir, prusti_server_address, flux_path, prusti_rustc):
    stats = []
    if suite == Suites.FLUX:
        benchmark_list = benchmark_list_flux
        prefix = dir + "/flux/"
    elif suite == Suites.PRUSTI:
        benchmark_list = benchmark_list_prusti
        prefix = dir + "/prusti/"

    for benchmark in benchmark_list:
        benchmark_stats = run_benchmark(prefix + benchmark, suite, prusti_server_address, flux_path, prusti_rustc)
        stats.append((benchmark, benchmark_stats))

    return stats

def dump_csv(stats, suite, file):
    filename, ext = os.path.splitext(os.path.basename(file))
    if suite == Suites.FLUX:
        filename = filename + "_flux"
    elif suite == Suites.PRUSTI:
        filename = filename + "_prusti"
    
    output = os.path.dirname(file) + filename + ext

    print(output)

    with open(output, 'w') as file:
        print("Benchmark, LOC, Function Contracts, Contract Lines, Loop Invariants, Invariant Lines, Verification Time", file=file)

        for (benchmark, counts) in stats:
            print("{}, {}, {}, {}, {}, {}, {}".format(benchmark, counts['lines'], counts['function_contracts'], counts['contract_lines'], counts['loop_invariants'], counts['invariant_lines'], counts['time']), file=file)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("directory", type=str, default=".")
    parser.add_argument("suites", type=Suites, default=Suites.BOTH, choices=list(Suites))
    parser.add_argument("output", type=str)
    parser.add_argument("--prusti_server_address", type=str, default='"MOCK"')
    parser.add_argument("--flux_path", type=str, default='.')
    parser.add_argument("--prusti_rustc", type=str, default='./prusti-rustc')
    args = parser.parse_args()

    if args.suites == Suites.BOTH or args.suites == Suites.FLUX:
        flux_stats = run_suite(Suites.FLUX, args.directory, args.prusti_server_address, args.flux_path, args.prusti_rustc)
        dump_csv(flux_stats, Suites.FLUX, args.output)

    if args.suites == Suites.BOTH or args.suites == Suites.PRUSTI:
        prusti_stats = run_suite(Suites.PRUSTI, args.directory, args.prusti_server_address, args.flux_path, args.prusti_rustc)
        dump_csv(prusti_stats, Suites.PRUSTI, args.output)

