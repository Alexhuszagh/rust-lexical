'''
    plot
    ====

    Create plots from the run benchmarks, as d3.js/HTML.
    This gives us beautiful, interactive plots that we
    can later save as an individual benchmark.
'''

import argparse
import json
import os
import re

import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

plt.style.use('ggplot')

etc = os.path.dirname(os.path.realpath(__file__))
home = os.path.dirname(etc)

def parse_args(argv=None):
    '''Create and parse our command line arguments.'''

    parser = argparse.ArgumentParser(description='Plot a benchmark.')
    parser.add_argument(
        '--benches',
        help='''name of benchmarks to run''',
        default='parse-float,parse-integer,write-float,write-integer',
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

def format_time(time):
    '''Format time to be a nice value.'''

    def strip_zero(value):
        if value.endswith('.0'):
            return value[:-2]
        return value

    if time < 1000:
        return f'{strip_zero(str(round(time, 3)))} ns'
    time /= 1000
    if time < 1000:
        return f'{strip_zero(str(round(time, 3)))} us'
    time /= 1000
    if time < 1000:
        return f'{strip_zero(str(round(time, 3)))} ms'
    time /= 1000
    return f'{strip_zero(str(round(time, 3)))} s'

def float_sort_key(x):
    '''Sort key for an float value.'''
    return (x[0], int(x[1:]))

def integer_sort_key(x):
    '''Sort key for an integral value.'''
    return (x[0], int(x[1:]))

def plot_bar(
    prefix=None,
    xlabel=None,
    data=None,
    path=None,
    title=None,
    key=None
):
    '''Plot a generic bar chart.'''

    keys = [i.split('_')[1:] for i in data.keys()]
    xticks = sorted({i[0] for i in keys})
    libraries = sorted({i[1] for i in keys})

    def plot_ax(ax, xticks):
        '''Plot an axis with various subfigures.'''

        length = len(xticks)
        width = 0.4 / len(libraries)
        x = np.arange(length)
        for index, library in enumerate(libraries):
            xi = x + width * index
            yi = [data[f'{prefix}_{i}_{library}'] for i in xticks]
            plt.bar(xi, yi, width, label=library, alpha=.7)

        ax.grid(color='white', linestyle='solid')
        ax.set_title(title)
        ax.set_xlabel(xlabel)
        ax.set_ylabel('Time (Log)')
        ax.set_yscale('log')
        ax.set_xticks(x + width * len(libraries) / 4)
        ax.set_xticklabels(xticks, rotation=-45)
        ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: format_time(x)))
        ax.legend(libraries, fancybox=True, framealpha=1, shadow=True, borderpad=1)

    fig = plt.figure(figsize=(10, 8))
    index = 1
    ax = fig.add_subplot(1, 1, 1)
    plot_ax(ax, xticks)

    fig.savefig(path, format='svg')
    fig.clf()

def plot_scatter(
    prefix=None,
    xlabel=None,
    data=None,
    path=None,
    title=None,
    rows=None,
    key=None
):
    '''Plot a generic scatter plot.'''

    keys = [i.split('_')[1:] for i in data.keys()]
    xticks = sorted({i[0] for i in keys}, key=key)
    libraries = sorted({i[1] for i in keys})
    row_data = [xticks]
    if rows is not None:
        row_data = [[i for i in xticks if i.startswith(r)] for r in rows]
        row_data = [i for i in row_data if i]

    def plot_ax(ax, xticks):
        '''Plot an axis with various subfigures.'''

        for library in libraries:
            ys = [data[f'{prefix}_{i}_{library}'] for i in xticks]
            points = ax.semilogy(
                xticks, ys, '-o', mec='k', ms=15,
                mew=1, alpha=.8, label=library
            )

        ax.grid(color='white', linestyle='solid')
        ax.set_title(title)
        ax.set_xlabel(xlabel)
        ax.set_ylabel('Time (Log)')
        ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: format_time(x)))
        ax.legend(libraries, fancybox=True, framealpha=1, shadow=True, borderpad=1)

    nrows = len(row_data)
    height = 8 * nrows
    fig = plt.figure(figsize=(10, height))
    index = 1
    for row in row_data:
        ax = fig.add_subplot(nrows, 1, index)
        plot_ax(ax, row)
        index += 1

    fig.savefig(path, format='svg')
    fig.clf()

def plot_write_float(args):
    '''Plot the write float dataset.'''

    assets = f'{home}/../lexical-write-float/assets'
    with open(f'{home}/results/{filename("write-float", args)}.json') as file:
        data = json.load(file)

    bar_kwds = {
        'prefix': 'write',
        'xlabel': 'Float Types',
        'key': float_sort_key,
    }

    # First plot JSON data.
    plot_bar(
        **bar_kwds,
        data=data['json'],
        path=f'{assets}/{filename("json", args)}.svg',
        title='JSON Data',
    )

    # Plot random data.
    plot_bar(
        **bar_kwds,
        data=data['random:uniform'],
        path=f'{assets}/{filename("random_uniform", args)}.svg',
        title='Random Data: Uniform',
    )
    plot_bar(
        **bar_kwds,
        data=data['random:one_over_rand32'],
        path=f'{assets}/{filename("random_one_over_rand32", args)}.svg',
        title='Random Data: One Over Rand32',
    )
    plot_bar(
        **bar_kwds,
        data=data['random:simple_uniform32'],
        path=f'{assets}/{filename("random_simple_uniform32", args)}.svg',
        title='Random Data: Simple Uniform32',
    )
    plot_bar(
        **bar_kwds,
        data=data['random:simple_int32'],
        path=f'{assets}/{filename("random_simple_int32", args)}.svg',
        title='Random Data: Simple Int32',
    )
    plot_bar(
        **bar_kwds,
        data=data['random:simple_int64'],
        path=f'{assets}/{filename("random_simple_int64", args)}.svg',
        title='Random Data: Simple Int64',
    )
    plot_bar(
        **bar_kwds,
        data=data['random:big_int_dot_int'],
        path=f'{assets}/{filename("random_big_int_dot_int", args)}.svg',
        title='Random Data: BigInt.Int',
    )
    plot_bar(
        **bar_kwds,
        data=data['random:big_ints'],
        path=f'{assets}/{filename("random_big_ints", args)}.svg',
        title='Random Data: BigInts',
    )

def plot_write_integer(args):
    '''Plot the write integer dataset.'''

    assets = f'{home}/../lexical-write-integer/assets'
    with open(f'{home}/results/{filename("write-integer", args)}.json') as file:
        data = json.load(file)

    scatter_kwds = {
        'prefix': 'write',
        'xlabel': 'Integer Types',
        'rows': ['u', 'i'],
        'key': integer_sort_key,
    }

    # Plot JSON data.
    plot_scatter(
        **scatter_kwds,
        data=data['json:simple'],
        path=f'{assets}/{filename("json_simple", args)}.svg',
        title='JSON Data: Simple',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['json:random'],
        path=f'{assets}/{filename("json_random", args)}.svg',
        title='JSON Data: Random',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['json:chain_random'],
        path=f'{assets}/{filename("json_chain_random", args)}.svg',
        title='JSON Data: Chained Random',
    )

    # Plot random data.
    plot_scatter(
        **scatter_kwds,
        data=data['random:uniform'],
        path=f'{assets}/{filename("random_uniform", args)}.svg',
        title='Random Data: Uniform',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:simple'],
        path=f'{assets}/{filename("random_simple", args)}.svg',
        title='Random Data: Simple',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:large'],
        path=f'{assets}/{filename("random_large", args)}.svg',
        title='Random Data: Large',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:simple_signed'],
        path=f'{assets}/{filename("random_simple_signed", args)}.svg',
        title='Random Data: Simple Negative',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:large_signed'],
        path=f'{assets}/{filename("random_large_signed", args)}.svg',
        title='Random Data: Large Negative',
    )

def plot_parse_float(args):
    '''Plot the parse float dataset.'''

    assets = f'{home}/../lexical-parse-float/assets'
    with open(f'{home}/results/{filename("parse-float", args)}.json') as file:
        data = json.load(file)

    # Need to plot the real data sets.
    real_data = {
        **data['canada'],
        **data['earth'],
        **data['mesh'],
    }
    plot_bar(
        prefix='parse',
        xlabel='Dataset',
        data=real_data,
        path=f'{assets}/{filename("real", args)}.svg',
        title='Real Datasets',
    )

    # Parse the random data
    random_keys = [k for k in data.keys() if k.startswith('random')]
    random_data = {}
    for key in random_keys:
        name = key.split(':')[1].replace('_', '-')
        subkeys = [i for i in data[key].keys() if 'f64' in i]
        for subkey in subkeys:
            keyname = subkey.replace('f64', name)
            random_data[keyname] = data[key][subkey]
    plot_bar(
        prefix='parse',
        xlabel='Generator',
        data=random_data,
        path=f'{assets}/{filename("random", args)}.svg',
        title='Random Data',
    )

    # Plot the contrived data.
    contrived_map = {
        "fast": data["contrived:fast"],
        "disguised": data["contrived:disguised"],
        "moderate": data["contrived:moderate"],
        "halfway": data["contrived:halfway"],
        "large": data["contrived:large30"],
        "denormal": data["contrived:denormal30"],
    }
    contrived_sort = {k: i for i, k in enumerate(contrived_map.keys())}
    contrived_data = {}
    for key, values in contrived_map.items():
        contrived_data.update({k.replace('f64', key): v for k, v in values.items()})
    plot_scatter(
        prefix='parse',
        xlabel='Float Type',
        data=contrived_data,
        path=f'{assets}/{filename("contrived", args)}.svg',
        title='Contrived Data',
        key=lambda x: contrived_sort[x],
    )

    # Plot the denormal and large data
    large_keys = [i for i in data.keys() if 'large' in i]
    large_data = {}
    for key in large_keys:
        name = key[len('contrived:large'):]
        values = data[key]
        large_data.update({k.replace('f64', name): v for k, v in values.items()})
    plot_scatter(
        prefix='parse',
        xlabel='Digit Count',
        data=large_data,
        path=f'{assets}/{filename("large", args)}.svg',
        title='Large, Near-Halfway Floats',
        key=lambda x: int(x),
    )

    denormal_keys = [i for i in data.keys() if 'denormal' in i]
    denormal_data = {}
    for key in denormal_keys:
        name = key[len('contrived:denormal'):]
        values = data[key]
        denormal_data.update({k.replace('f64', name): v for k, v in values.items()})
    plot_scatter(
        prefix='parse',
        xlabel='Digit Count',
        data=denormal_data,
        path=f'{assets}/{filename("denormal", args)}.svg',
        title='Denormal, Near-Halfway Floats',
        key=lambda x: int(x),
    )

def plot_parse_integer(args):
    '''Plot the parse integer dataset.'''

    assets = f'{home}/../lexical-parse-integer/assets'
    with open(f'{home}/results/{filename("parse-integer", args)}.json') as file:
        data = json.load(file)

    scatter_kwds = {
        'prefix': 'parse',
        'xlabel': 'Integer Types',
        'rows': ['u', 'i'],
        'key': integer_sort_key,
    }

    # First plot JSON data.
    plot_scatter(
        **scatter_kwds,
        data=data['json:simple'],
        path=f'{assets}/{filename("json_simple", args)}.svg',
        title='JSON Data: Simple',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['json:random'],
        path=f'{assets}/{filename("json_random", args)}.svg',
        title='JSON Data: Random',
    )

    # First plot random data.
    plot_scatter(
        **scatter_kwds,
        data=data['random:uniform'],
        path=f'{assets}/{filename("random_uniform", args)}.svg',
        title='Random Data: Uniform',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:simple'],
        path=f'{assets}/{filename("random_simple", args)}.svg',
        title='Random Data: Simple',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:large'],
        path=f'{assets}/{filename("random_large", args)}.svg',
        title='Random Data: Large',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:simple_signed'],
        path=f'{assets}/{filename("random_simple_signed", args)}.svg',
        title='Random Data: Simple Negative',
    )
    plot_scatter(
        **scatter_kwds,
        data=data['random:large_signed'],
        path=f'{assets}/{filename("random_large_signed", args)}.svg',
        title='Random Data: Large Negative',
    )

def main(argv=None):
    '''Entry point.'''

    args = parse_args(argv)
    benches = args.benches.split(',')
    for bench in benches:
        if bench == 'write-integer':
            plot_write_integer(args)
        elif bench == 'write-float':
            plot_write_float(args)
        elif bench == 'parse-integer':
            plot_parse_integer(args)
        elif bench == 'parse-float':
            plot_parse_float(args)
        else:
            raise NotImplementedError

if __name__ == '__main__':
    main()
