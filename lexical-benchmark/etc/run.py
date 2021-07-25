'''
    run
    ===

    Run benchmarks and save the data to JSON.
'''

import argparse
import collections
import contextlib
import json
import re
import os
import subprocess

etc = os.path.dirname(os.path.realpath(__file__))
home = os.path.dirname(etc)

def parse_args(argv=None):
    '''Create and parse our command line arguments.'''

    parser = argparse.ArgumentParser(description='Run a benchmark.')
    parser.add_argument(
        '--benches',
        help='''name of benchmarks to run''',
        default='parse-float,parse-integer,write-float,write-integer',
    )
    parser.add_argument(
        '--filter',
        help='''filter benchmarks to run''',
        default='',
    )
    parser.add_argument(
        '--features',
        help='''optional features to add''',
        default='',
    )
    parser.add_argument(
        '--no-default-features',
        help='''disable default features''',
        action='store_true',
    )
    return parser.parse_args(argv)

def filename(basename, args):
    '''Get a resilient name for the benchmark data.'''

    name = basename
    if args.no_default_features:
        name = f'{name}_nodefault'
    if args.features:
        name = f'{name}_features={args.features}'
    return name

@contextlib.contextmanager
def change_directory(path):
    '''Change directory and return to the original directory afterwards.'''

    cwd = os.getcwd()
    try:
        os.chdir(path)
        yield
    finally:
        os.chdir(cwd)

def process_rust_benchmark(line):
    '''Process the result of an individual Rust benchmark.'''

    pattern = r'test ([A-Za-z0-9_:]+)/([A-Za-z0-9_:]+) \.\.\. bench:\s+ (\d+) (\w+)/iter'
    match = re.match(pattern, line)
    group = match.group(1)
    name = match.group(2)
    speed = float(match.group(3))
    unit = match.group(4)
    if unit == 'ns':
        pass
    elif unit == 'us':
        speed *= 1e3
    elif unit == 'ms':
        speed *= 1e6
    else:
        raise ValueError('Unknown unit: ' + unit)

    return group, name, speed

def run_benchmark(args):
    '''Run a single benchmark.'''

    command = ['cargo', 'bench']
    if args.filter:
        command.append(args.filter)
    if args.no_default_features:
        command.append('--no-default-features')
    if args.features:
        command.append(f'--features={args.features}')
    # Use the bencher output since it's consistent and easy to parse.
    command += ['--', '--output-format', 'bencher']
    process = subprocess.Popen(command, stdout=subprocess.PIPE)
    data = collections.defaultdict(dict)
    for line in iter(process.stdout.readline, b''):
        line = line.decode('utf-8')
        print(line)
        if line.startswith('test'):
            group, name, speed = process_rust_benchmark(line)
            data[group][name] = speed

    process.stdout.close()
    process.wait()

    return data

def main(argv=None):
    '''Entry point.'''

    args = parse_args(argv)
    benches = args.benches.split(',')
    os.makedirs(f'{home}/results', exist_ok=True)
    for bench in benches:
        with change_directory(f'{home}/{bench}'):
            data = run_benchmark(args)
            with open(f'{home}/results/{filename(bench, args)}.json', 'w') as file:
                json.dump(data, file)

if __name__ == '__main__':
    main()
