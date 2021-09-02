#!/usr/bin/env python
'''
    size
    ====

    Get the binary sizes from executables using lexical. By default,
    this uses binutils `size` command to probe binary sizes: if `size`
    is not on the path, like on Windows, you can set the `SIZE` environment
    variable to manually specify the `size` executable. Likewise, the `strip`
    command can be overrided by the `STRIP` environment variable.
'''

import argparse
import json
import mimetypes
import subprocess
import os

import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

plt.style.use('ggplot')

if os.name == 'nt':
    from winmagic import magic
else:
    import magic

scripts = os.path.dirname(os.path.realpath(__file__))
home = os.path.dirname(scripts)
size_command = os.environ.get('SIZE', 'size')
strip_command = os.environ.get('STRIP', 'strip')

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
    parser.add_argument(
        '--plot',
        help='''plot graphs''',
        action='store_true',
    )
    parser.add_argument(
        '--run',
        help='''generate size data''',
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
        ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: prettyify(x)))
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
    if os.name == 'nt':
        return magic.from_file(path, mime=True) == 'application/x-dosexec'
    else:
        return magic.from_file(path, mime=True) == 'application/x-pie-executable'

def prettyify(size):
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

def get_file_size(path):
    '''Read the file size of a given binary.'''

    # Use the size utility, and grep for 2 sections.
    # We can't use `stat`, or `file`, or any other
    # utility that isn't aware of padding. We have
    # 3 sections that matter: .rodata, .text, and .data.
    #   .text: Compiled code
    #   .rodata: Read-only data (on Windows, this is `.rdata`)
    #   .data: Other data (often empty).
    cmd = [size_command, '-A', '-d', path]
    stdout = subprocess.run(cmd, check=True, stdout=subprocess.PIPE).stdout
    stdout = stdout.decode('utf-8')
    lines = [i.strip() for i in stdout.splitlines()[2:] if i.strip()]
    sections = dict([i.split()[:2] for i in lines])
    text = int(sections['.text'])
    data = int(sections['.data'])
    if os.name == 'nt':
        rodata = int(sections['.rdata'])
    else:
        rodata = int(sections['.rodata'])

    return text + data + rodata

def get_sizes(level):
    '''Get the binary sizes for all targets.'''

    data = {}
    build_type = LEVELS[level]
    target = f'{home}/lexical-size/target/{build_type}'
    for filename in os.listdir(target):
        path = os.path.join(target, filename)
        if os.path.isfile(path) and is_executable(path):
            exe_name = filename
            if os.name == 'nt':
                exe_name = filename[:-len('.exe')]
            data[exe_name] = get_file_size(path)

    empty = data.pop('empty')

    return {k: v - empty for k, v in data.items()}

def strip(level):
    '''Strip all the binaries'''

    if os.name == 'nt':
        # The Portable Executable format uses PDB for debugging info.
        return

    build_type = LEVELS[level]
    target = f'{home}/lexical-size/target/{build_type}'
    for filename in os.listdir(target):
        path = os.path.join(target, filename)
        if os.path.isfile(path) and is_executable(path):
            subprocess.check_call(
                [strip_command, path],
                stderr=subprocess.DEVNULL,
                stdout=subprocess.DEVNULL,
            )

def plot_level(args, data, level):
    '''Print markdown-based report for the file sizes.'''

    print(f'Plotting binary sizes for optimization level {level}.')
    def sort_key(x):
        split = x.split('-')
        ctype = split[-1]
        return (split[0], split[1], ctype[0], int(ctype[1:]))

    def flatten(lib, key, filter):
        subdata = {k: v for k, v in data[lib][key].items() if filter in k}
        return {f'{lib}_{k.split("-")[2]}': v for k, v in subdata.items()}

    # Create the bar graphs
    assets = f'{home}/assets'
    bar_kwds = {
        'xlabel': 'Binary Sizes',
        'key': sort_key,
    }

    if os.name == 'nt':
        pe = {
            **flatten('core', 'pe', 'parse'),
            **flatten('lexical', 'pe', 'parse'),
        }
        file = filename(f'size_parse_pe_opt{level}_{os.name}', args)
        plot_bar(
            **bar_kwds,
            data=pe,
            path=f'{assets}/{file}.svg',
            title=f'Parse Data -- Optimization Level "{level}"',
        )

        pe = {
            **flatten('core', 'pe', 'write'),
            **flatten('lexical', 'pe', 'write'),
        }
        file = filename(f'size_write_pe_opt{level}_{os.name}', args)
        plot_bar(
            **bar_kwds,
            data=pe,
            path=f'{assets}/{file}.svg',
            title=f'Write Data -- Optimization Level "{level}"',
        )
    else:
        unstripped = {
            **flatten('core', 'unstripped', 'parse'),
            **flatten('lexical', 'unstripped', 'parse'),
        }
        file = filename(f'size_parse_unstripped_opt{level}_{os.name}', args)
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
        file = filename(f'size_write_unstripped_opt{level}_{os.name}', args)
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
        file = filename(f'size_parse_stripped_opt{level}_{os.name}', args)
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
        file = filename(f'size_write_stripped_opt{level}_{os.name}', args)
        plot_bar(
            **bar_kwds,
            data=stripped,
            path=f'{assets}/{file}.svg',
            title=f'Write Stripped Data -- Optimization Level "{level}"',
        )

def run_level(args, level, is_lexical):
    '''Generate the size data for a given build configuration.'''

    print(f'Calculating binary sizes for optimization level {level}.')
    write_manifest(level)
    clean()
    build(args, level, is_lexical)
    data = {}
    if os.name == 'nt':
        data['pe'] = get_sizes(level)
    else:
        data['unstripped'] = get_sizes(level)
        strip(level)
        data['stripped'] = get_sizes(level)

    return data

def run(args):
    '''Run the size calculations.'''

    assets = f'{home}/assets'
    opt_levels = args.opt_levels.split(',')
    for level in opt_levels:
        data = {}
        data['core'] = run_level(args, level, False)
        data['lexical'] = run_level(args, level, True)
        file = filename(f'size{level}_{os.name}', args)
        with open(f'{assets}/{file}.json', 'w') as file:
            json.dump(data, file)

def plot(args):
    '''Plot the size calculations.'''

    assets = f'{home}/assets'
    opt_levels = args.opt_levels.split(',')
    for level in opt_levels:
        file = filename(f'size{level}_{os.name}', args)
        with open(f'{assets}/{file}.json', 'r') as file:
            data = json.load(file)
        plot_level(args, data, level)

def main(argv=None):
    '''Entry point.'''

    args = parse_args(argv)
    if args.run:
        run(args)
    if args.plot:
        plot(args)

if __name__ == '__main__':
    main()
