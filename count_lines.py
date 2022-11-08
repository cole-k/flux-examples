import argparse
import glob
import json


def is_preamble(line):
    if line.startswith("pub mod"):
        return True
    if line.startswith("mod"):
        return True
    elif line.startswith("use"):
        return True
    elif line.startswith("extern"):
        return True
    elif line.startswith("pub(crate) mod"):
        return True
    elif line.startswith("pub(crate) use"):
        return True
    return False


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


def is_flux_annotation(annotation):
    return "flux::" in annotation


def count_file(path):
    file = open(path, 'r')
    lines = file.readlines()

    counts = {
        "lines": 0,
        "loop_invariants": 0,
        "invariant_lines": 0,
        "function_contracts": 0,
        "contract_lines": 0,
    }

    line_number = 0
    in_contract_block = False
    in_contract = False
    in_body_invariant_block = False
    in_body_invariant = False
    in_predicate = False
    predicate_ident = 0

    # Strips the newline character
    for line in lines:
        line_number += 1
        stripped = line.strip()
        ident = len(line.rstrip()) - len(stripped)
        if stripped:
            if (stripped[0] != "/" or stripped[1] != "/"
                ) and stripped[:16] != "pub fn main() {}" and not is_preamble(
                    stripped):
                if stripped[0] == "#":
                    if is_prusti_annotation(stripped) or is_flux_annotation(
                            stripped):
                        counts['contract_lines'] = counts['contract_lines'] + 1
                        if not in_contract_block:
                            counts['function_contracts'] = counts[
                                'function_contracts'] + 1
                        in_contract_block = True
                        in_contract = True

                        if stripped.endswith("]"):
                            in_contract = False

                elif in_contract:
                    counts['contract_lines'] += 1
                    if stripped.endswith(")]"):
                        in_contract = False

                elif stripped.startswith("predicate!"):
                    predicate_ident = ident
                    counts['contract_lines'] += 1
                    in_predicate = True
                    if stripped.endswith("}"):
                        in_predicate = False

                elif in_predicate:
                    counts['contract_lines'] += 1
                    if predicate_ident == ident:
                        in_predicate = False

                elif stripped.startswith("body_invariant!"):
                    counts['invariant_lines'] = counts['invariant_lines'] + 1
                    if not in_body_invariant_block:
                        counts[
                            'loop_invariants'] = counts['loop_invariants'] + 1
                    in_body_invariant = True
                    in_body_invariant_block = True

                    if stripped.endswith(");"):
                        in_body_invariant = False

                elif in_body_invariant:
                    counts['invariant_lines'] += 1
                    if stripped == ");":
                        in_body_invariant = False
                else:
                    in_body_invariant_block = False
                    in_contract_block = False
                    counts['lines'] = counts['lines'] + 1
    return counts


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=str)
    args = parser.parse_args()

    counts = {
        "lines": 0,
        "loop_invariants": 0,
        "invariant_lines": 0,
        "function_contracts": 0,
        "contract_lines": 0,
    }
    for file in glob.iglob(args.file, recursive=True):
        counts = {k: v + counts[k] for (k, v) in count_file(file).items()}

    print(json.dumps(counts))
