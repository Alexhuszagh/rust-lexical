#!/usr/bin/env python
'''
    compare
    =======

    Compare a baseline to new metrics from 2 criterion runs.

    This uses 2 previously exported entries from `profiling.py` and then
    exports them to a markdown file, of the following format, where the
    percentage is `new / baseline`, so a value below 100% is better.

        Results
        =======

        | | core | lexical |
        |:-:|:-:|:-:|
        | parse_i128 | 99.83% | 100.21% |
        | parse_i16 | 99.23% | 99.23% |
        | parse_i32 | 99.1% | 98.07% |
        | parse_i64 | 100.77% | 100.29% |
        | parse_i8 | 101.14% | 98.64% |
        | parse_u128 | 99.08% | 99.06% |
        | parse_u16 | 99.48% | 98.64% |
        | parse_u32 | 101.47% | 99.27% |
        | parse_u64 | 100.28% | 101.1% |
        | parse_u8 | 100.06% | 102.18% |
        | parse_u128 | 99.31% | 100.02% |
        | parse_u16 | 101.86% | 99.24% |
        | parse_u32 | 100.97% | 100.96% |
'''

import argparse
import json
import re
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


def numsort(value):
    '''We want to sort on the numbers, so say i8 < i32.'''
    match = re.search(r'[iuf](\d{1,3})', value)
    if match is None:
        return value
    numeric = match.group(1).rjust(3, '0')
    return value[:match.start() + 1] + numeric + value[match.end() + 1:]


# grab our comparison names, so we know where to analyze
group = next(iter(baseline.values()))
tools = sorted({i.rsplit('_')[-1] for i in group['mean']})

# NOTE: These values are in nanoseconds. so, we want to
# then scale it to the appropriate value accordingly


def get_unit(x):
    '''Get the correct unit suffix and scaling factor.'''
    if x < 10**3:
        return ('ns', 1)
    elif x < 10**6:
        return ('us', 1/10**3)
    elif x < 10**9:
        return ('ms', 1/10**6)
    return ('s', 1/10**9)


def get_compare_unit(x, y):
    '''Get the correct unit suffix and scaling factor.'''
    return get_unit(min(x, y))


ratios = []
raw_values = []
first_tool = tools[0]
for test_group, baseline_results in baseline.items():
    new_means = new[test_group]['mean']
    baseline_means = baseline_results['mean']
    tests = [i[: -len(first_tool) - 1] for i in baseline_means if i.endswith(f'_{first_tool}')]
    tests.sort(key=numsort)
    for test in tests:
        ratio_row = []
        value_row = []
        for tool in tools:
            test_name = f'{test_group}/{test}' if test_group not in test else test
            baseline_mean = baseline_means[f'{test}_{tool}']
            new_mean = new_means[f'{test}_{tool}']
            ratio_row.append(str(round(new_mean * 100 / baseline_mean, 2)) + '%')
            unit, scale = get_compare_unit(new_mean, baseline_mean)
            value_row.append(str(round(new_mean * scale, 4)) + unit)
            value_row.append(str(round(baseline_mean * scale, 4)) + unit)
        ratios.append(f'| {test_name} | {" | ".join(ratio_row)} |')
        raw_values.append(f'| {test_name} | {" | ".join(value_row)} |')

ratio_header = f'| | {" | ".join(tools)} |'
ratio_center = f'|:{"-:|:" * len(tools)}-:|'
pairs = [j for i in tools for j in (f'{i} - New', f'{i} - Base')]
value_header = f'| | {" | ".join(pairs)} |'
value_center = f'|:{"-:|:" * len(tools) * 2}-:|'

output = '# Results\n\n'
output += '## Ratios\n\n'
output += 'These results are calculated as `new / baseline`, so values below 100% are better.\n\n'
output += f'{ratio_header}\n{ratio_center}\n'
output += '\n'.join(ratios)
output += '\n\n## Raw Values\n\n'
output += f'{value_header}\n{value_center}\n'
output += '\n'.join(raw_values)


with args.output.open('w', encoding='utf-8') as file:
    print(output, file=file)
