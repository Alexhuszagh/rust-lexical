#!/usr/bin/env python
'''
    size
    ====

    Get the binary sizes from executables using lexical.
'''

import argparse
import magic
import mimetypes
import subprocess
import os

import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

plt.style.use('ggplot')

scripts = os.path.dirname(os.path.realpath(__file__))
home = os.path.dirname(scripts)

LEVELS = {
    '0': 'debug',
    '1': 'debug',
    '2': 'release',
    '3': 'release',
    's': 'release',
    'z': 'release',
}

DEBUG = '''
[profile.dev]
opt-level = {level}
debug = true
debug-assertions = true
lto = false
'''

RELEASE = '''
[profile.release]
opt-level = {level}
debug = false
debug-assertions = false
lto = true
'''

def parse_args(argv=None):
    '''Create and parse our command line arguments.'''

    parser = argparse.ArgumentParser(description='Get lexical binary sizes.')
    parser.add_argument(
        '--opt-levels',
        help='''optimization levels to test''',
        default='0,1,2,3,s,z',
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

def plot_bar(
    xlabel=None,
    data=None,
    path=None,
    title=None,
    key=None
):
    '''Plot a generic bar chart.'''

    keys = [i.split('_') for i in data.keys()]
    xticks = sorted({i[1] for i in keys})
    libraries = sorted({i[0] for i in keys})

    def plot_ax(ax, xticks):
        '''Plot an axis with various subfigures.'''

        length = len(xticks)
        width = 0.4 / len(libraries)
        x = np.arange(length)
        for index, library in enumerate(libraries):
            xi = x + width * index
            yi = [data[f'{library}_{i}'] for i in xticks]
            plt.bar(xi, yi, width, label=library, alpha=.7)

        ax.grid(color='white', linestyle='solid')
        ax.set_title(title)
        ax.set_xlabel(xlabel)
        ax.set_ylabel('Size (B)')
        ax.set_yscale('log')
        ax.set_xticks(x + width * len(libraries) / 4)
        ax.set_xticklabels(xticks, rotation=-45)
        ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: filesize(x)))
        ax.legend(libraries, fancybox=True, framealpha=1, shadow=True, borderpad=1)

    fig = plt.figure(figsize=(10, 8))
    index = 1
    ax = fig.add_subplot(1, 1, 1)
    plot_ax(ax, xticks)

    fig.savefig(path, format='svg')
    fig.clf()

def clean():
    '''Clean the project'''

    os.chdir(f'{home}/lexical-size')
    subprocess.check_call(
        'cargo +nightly clean',
        shell=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

def write_manifest(level):
    '''Write the manifest for the given optimization level.'''

    manifest = f'{home}/lexical-size/Cargo.toml'
    with open(f'{manifest}.in') as file:
        contents = file.read()

    toml_level = level
    if toml_level.isalpha():
        toml_level = f'"{level}"'
    if LEVELS[level] == 'debug':
        contents += DEBUG.format(level=toml_level)
    else:
        contents += RELEASE.format(level=toml_level)

    with open(manifest, 'w') as file:
        file.write(contents)

def build(args, level, is_lexical):
    '''Build the project.'''

    os.chdir(f'{home}/lexical-size')
    command = f'cargo +nightly build'
    if args.no_default_features:
        command = f'{command} --no-default-features'
    features = args.features
    if is_lexical:
        if features:
            features = f'{features},lexical'
        else:
            features = 'lexical'
    if features:
        command = f'{command} --features={features}'
    if LEVELS[level] == 'release':
        command = f'{command} --release'
    subprocess.check_call(
        # Use shell for faster performance.
        # Spawning a new process is a **lot** slower, gives misleading info.
        command,
        shell=True,
        stderr=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
    )

def is_executable(path):
    '''Determine if a file is a binary executable.'''
    return magic.from_file(path, mime=True) == 'application/x-pie-executable'

def filesize(size):
    '''Get the human readable filesize from bytes.'''

    suffixes = ['KB', 'MB', 'GB', 'TB']
    if size < 1024:
        return f'{size}B'
    size /= 1024
    for suffix in suffixes:
        if size < 1024:
            return f'{size:0.1f}{suffix}'
        size /= 1024

    return f'{size:0.1f}PB'

def get_sizes(level):
    '''Get the binary sizes for all targets.'''

    data = {}
    build_type = LEVELS[level]
    target = f'{home}/lexical-size/target/{build_type}'
    for filename in os.listdir(target):
        path = os.path.join(target, filename)
        if os.path.isfile(path) and is_executable(path):
            st = os.stat(path)
            data[filename] = st.st_size

    empty = data.pop('empty')

    return {k: v - empty for k, v in data.items()}

def strip(level):
    '''Strip all the binaries'''

    build_type = LEVELS[level]
    target = f'{home}/lexical-size/target/{build_type}'
    for filename in os.listdir(target):
        path = os.path.join(target, filename)
        if os.path.isfile(path) and is_executable(path):
            subprocess.check_call(
                f'strip "{path}"',
                shell=True,
                stderr=subprocess.DEVNULL,
                stdout=subprocess.DEVNULL,
            )

def print_report(args, data, level):
    '''Print markdown-based report for the file sizes.'''

    def sort_key(x):
        split = x.split('-')
        ctype = split[-1]
        return (split[0], split[1], ctype[0], int(ctype[1:]))

    def flatten(lib, key, filter):
        subdata = {k: v for k, v in data[lib][key].items() if filter in k}
        return {f'{lib}_{k.split("-")[2]}': v for k, v in data[lib][key].items()}

    # Create the bar graphs
    assets = f'{home}/assets'
    bar_kwds = {
        'xlabel': 'Binary Sizes',
        'key': sort_key,
    }

    unstripped = {
        **flatten('core', 'unstripped', 'parse'),
        **flatten('lexical', 'unstripped', 'parse'),
    }
    file = filename(f'parse_unstripped_opt{level}', args)
    plot_bar(
        **bar_kwds,
        data=unstripped,
        path=f'{assets}/{file}.svg',
        title=f'Parse Unstripped Data -- Optimization Level "{level}"',
    )

    unstripped = {
        **flatten('core', 'unstripped', 'write'),
        **flatten('lexical', 'unstripped', 'write'),
    }
    file = filename(f'write_unstripped_opt{level}', args)
    plot_bar(
        **bar_kwds,
        data=unstripped,
        path=f'{assets}/{file}.svg',
        title=f'Write Unstripped Data -- Optimization Level "{level}"',
    )

    stripped = {
        **flatten('core', 'stripped', 'parse'),
        **flatten('lexical', 'stripped', 'parse'),
    }
    file = filename(f'parse_stripped_opt{level}', args)
    plot_bar(
        **bar_kwds,
        data=stripped,
        path=f'{assets}/{file}.svg',
        title=f'Parse Stripped Data -- Optimization Level "{level}"',
    )

    stripped = {
        **flatten('core', 'stripped', 'write'),
        **flatten('lexical', 'stripped', 'write'),
    }
    file = filename(f'write_stripped_opt{level}', args)
    plot_bar(
        **bar_kwds,
        data=stripped,
        path=f'{assets}/{file}.svg',
        title=f'Write Stripped Data -- Optimization Level "{level}"',
    )

    # Print the report
    print(f'**Optimization Level "{level}"**')
    print('')
    print(f'|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|')
    print(f'|:-:|:-:|:-:|:-:|:-:|')
    keys = sorted(data['core']['stripped'], key=sort_key)
    for key in keys:
        uc = filesize(data['core']['unstripped'][key])
        sc = filesize(data['core']['stripped'][key])
        ul = filesize(data['lexical']['unstripped'][key])
        sl = filesize(data['lexical']['stripped'][key])
        print(f'|{key}|{ul}|{sl}|{uc}|{sc}|')
    print('', flush=True)

def generate_size_data(args, level, is_lexical):
    '''Generate the size data for a given build configuration.'''

    write_manifest(level)
    clean()
    build(args, level, is_lexical)
    unstripped = get_sizes(level)
    strip(level)
    stripped = get_sizes(level)

    return {
        'unstripped': unstripped,
        'stripped': stripped,
    }

def main(argv=None):
    '''Entry point.'''

    args = parse_args(argv)
    opt_levels = args.opt_levels.split(',')
    for level in opt_levels:
        data = {}
        data['core'] = generate_size_data(args, level, False)
        data['lexical'] = generate_size_data(args, level, True)
        print_report(args, data, level)

if __name__ == '__main__':
    main()
