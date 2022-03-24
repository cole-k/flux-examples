import argparse
import os

def is_prusti_annotation(annotation):
    if annotation[2:6] == "pure":
        return True
    elif annotation[2:9] == "trusted":
        return True
    elif annotation[2:9] == "ensures":
        return True
    elif annotation[2:10] == "requires":
        return True
    return False

def is_rustc_annotation(annotation):
    if annotation[1:2] == "lr":
        return True
    return False

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=str)
    args = parser.parse_args()
    file = open(args.file, 'r')
    lines = file.readlines()

    benchmark, _ = os.path.splitext(os.path.basename(args.file))
    print(benchmark)

    counts = {
        "lines": 0,
        "loop_invariants": 0,
        "invariant_lines": 0,
        "function_contracts": 0,
        "contract_lines": 0,
    }

    line_number = 0
    # Strips the newline character
    for line in lines:
        line_number += 1
        stripped = line.strip()
        if stripped:
            if (stripped[0] != "/" or stripped[1] != "/") and stripped[:16] != "pub fn main() {}":
                if stripped[0] == "#":
                    if is_prusti_annotation(stripped):
                        print("Line {} is a prusti function contract".format(line_number))
                        counts['contract_lines'] = counts['contract_lines'] + 1
                        if not in_contract:
                            counts['function_contracts'] = counts['function_contracts'] + 1
                        in_contract = True

                    elif is_rustc_annotation(stripped):
                        print("Line {} is a liquid-rust function contract".format(line_number))
                        counts['contract_lines'] = counts['contract_lines'] + 1
                        if not in_contract:
                            counts['function_contracts'] = counts['function_contracts'] + 1
                        in_contract = True

                elif stripped[:14] == "body_invariant":
                    print("Line {} is a body_invariant".format(line_number))
                    counts['invariant_lines'] = counts['invariant_lines'] + 1
                    if not in_invariant:
                        counts['loop_invariants'] = counts['loop_invariants'] + 1
                    in_invariant = True

                else:
                    print("Line {} is a line of code".format(line_number))
                    in_contract = False
                    in_invariant = False
                    counts['lines'] = counts['lines'] + 1

    print("{}, {}, {}, {}, {}, {}".format(benchmark, counts['lines'], counts['function_contracts'], counts['contract_lines'], counts['loop_invariants'], counts['invariant_lines']))