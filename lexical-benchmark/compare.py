#!/usr/bin/env python
'''
    profiling
    =========

    Create a baseline for metrics between various tools.

    This finds all the results from criterion in the target directories,
    and then concatenates them and joins various tooling into a single
    file.

    The file will be output to `/target/profiling.json` (by default).
'''

import argparse
import json
from pathlib import Path

home = Path(__file__).absolute().parent
target = home / 'target'

parser = argparse.ArgumentParser(
    prog='compare',
    description='Compare profiling results between criterion runs.'
)
parser.add_argument(
    '-b',
    '--baseline',
    type=Path,
    help='the file to serve as the baseline for the comparison.',
    required=True,
)
parser.add_argument(
    '-n',
    '--new',
    type=Path,
    help='the file to serve as the new run for the comparison.',
    required=True,
)
parser.add_argument(
    '-o',
    '--output',
    type=Path,
    help='the file to serve as the new run for the comparison.',
    required=True,
)
args = parser.parse_args()

with args.baseline.open(encoding='utf-8') as fp:
    baseline = json.load(fp)
with args.new.open(encoding='utf-8') as fp:
    new = json.load(fp)

# grab our comparison names, so we know where to analyze
group = next(iter(baseline.values()))
tools = [i.rsplit('_')[-1] for i in group['mean']]
header = f'| | {" | ".join(tools)} |'
center = f'|:{"-:|:" * len(tools)}-:|'

# NOTE: These values are in nanoseconds. so, we want to
# then scale it to the appropriate value accordingly


def get_unit(x, y):
    '''Get the correct unit suffix and scaling factor.'''
    less = min(x, y)
    if less > 10**9:
        return ('ns', 1)
    elif less > 10**6:
        return ('us', 1/10**3)
    elif less > 10**3:
        return ('ms', 1/10**6)
    return ('s', 1/10**9)


output = 'Results\n=======\n\n' + header + '\n' + center + '\n'
for test, baseline_results in baseline.items():
    row = []
    for tool in tools:
        new_results = new[test]
        baseline_mean = [v for k, v in baseline_results['mean'].items() if k.endswith(f'_{tool}')][0]
        new_mean = [v for k, v in new_results['mean'].items() if k.endswith(f'_{tool}')][0]
        row.append(str(round(baseline_mean / new_mean * 100, 2)) + '%')
    output += f'| {test} | {" | ".join(row)} |\n'

with args.output.open('w', encoding='utf-8') as file:
    print(output, file=file)
