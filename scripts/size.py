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
        size /= multiple

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

    return {k: filesize(v - empty) for k, v in data.items()}

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

def print_report(data, level):
    '''Print markdown-based report for the file sizes.'''

    def sort_key(x):
        split = x.split('-')
        ctype = split[-1]
        return (split[0], split[1], ctype[0], int(ctype[1:]))

    print(f'**Optimization Level "{level}"**')
    print('')
    print(f'|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|')
    print(f'|:-:|:-:|:-:|:-:|:-:|')
    keys = sorted(data['core']['stripped'], key=sort_key)
    for key in keys:
        uc = data['core']['unstripped'][key]
        sc = data['core']['stripped'][key]
        ul = data['lexical']['unstripped'][key]
        sl = data['lexical']['stripped'][key]
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
        print_report(data, level)

if __name__ == '__main__':
    main()
