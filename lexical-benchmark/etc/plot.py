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

import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

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
    return parser.parse_args(argv)

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

def plot_write_integer_figure(data, path, title):
    '''Plot JSON data'''

    keys = [i.split('_')[1:] for i in data.keys()]
    xticks = sorted({i[0] for i in keys}, key=lambda x: (x[0], int(x[1:])))
    libraries = sorted({i[1] for i in keys})
    unsigned = [i for i in xticks if i.startswith('u')]
    signed = [i for i in xticks if i.startswith('i')]

    def plot_ax(ax, xticks):
        '''Plot an axis with various subfigures.'''

        for library in libraries:
            ys = [data[f'write_{i}_{library}'] for i in xticks]
            points = ax.semilogy(
                xticks, ys, '-o', mec='k', ms=15,
                mew=1, alpha=.8, label=library
            )
            labels = [format_time(i) for i in ys]

        ax.grid(color='white', linestyle='solid')
        ax.set_title(title)
        ax.set_xlabel('Integer Types')
        ax.set_ylabel('Time (Log)')
        ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: format_time(x)))
        ax.legend(libraries, fancybox=True, framealpha=1, shadow=True, borderpad=1)

    nrows = bool(signed) + bool(unsigned)
    height = 8 * nrows
    fig = plt.figure(figsize=(10, height))
    index = 1
    if unsigned:
        ax = fig.add_subplot(nrows, 1, index)
        plot_ax(ax, unsigned)
        index += 1
    if signed:
        ax = fig.add_subplot(nrows, 1, index)
        plot_ax(ax, signed)
        index += 1

    fig.savefig(path, format='svg')
    fig.clf()


def plot_write_integer():
    '''Plot the write integer dataset.'''

    assets = f'{home}/../lexical-write-integer/assets'
    with open(f'{home}/results/write-integer.json') as file:
        data = json.load(file)

    # First plot JSON data.
    plot_write_integer_figure(
        data['json:simple'],
        f'{assets}/json_simple.svg',
        'JSON Data: Simple',
    )
    plot_write_integer_figure(
        data['json:random'],
        f'{assets}/json_random.svg',
        'JSON Data: Random',
    )
    plot_write_integer_figure(
        data['json:chain_random'],
        f'{assets}/json_chain_random.svg',
        'JSON Data: Chained Random',
    )

    # First plot random data.
    plot_write_integer_figure(
        data['random:uniform'],
        f'{assets}/random_uniform.svg',
        'Random Data: Uniform',
    )
    plot_write_integer_figure(
        data['random:simple'],
        f'{assets}/random_simple.svg',
        'Random Data: Simple',
    )
    plot_write_integer_figure(
        data['random:large'],
        f'{assets}/random_large.svg',
        'Random Data: Large',
    )
    plot_write_integer_figure(
        data['random:simple_signed'],
        f'{assets}/random_simple_signed.svg',
        'Random Data: Simple Negative',
    )
    plot_write_integer_figure(
        data['random:large_signed'],
        f'{assets}/random_large_signed.svg',
        'Random Data: Large Negative',
    )

def main(argv=None):
    '''Entry point.'''

    args = parse_args(argv)
    benches = args.benches.split(',')
    for bench in benches:
        if bench == 'write-integer':
            plot_write_integer()
        elif bench == 'write-float':
            plot_write_float()
        elif bench == 'parse-integer':
            plot_parse_integer()
        elif bench == 'parse-float':
            plot_parse_float()
        else:
            raise NotImplementedError

if __name__ == '__main__':
    main()
