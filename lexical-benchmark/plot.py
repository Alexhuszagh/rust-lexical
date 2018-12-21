import json
import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
plt.style.use('ggplot')

HOME = os.path.dirname(os.path.realpath(__file__))

with open(os.path.join(HOME, "results", "double_conversion.json"), 'r') as f:
    DOUBLE_CONVERSION = json.load(f)

with open(os.path.join(HOME, "results", "golang.json"), 'r') as f:
    GOLANG = json.load(f)

with open(os.path.join(HOME, "results", "lexical.json"), 'r') as f:
    LEXICAL = json.load(f)

with open(os.path.join(HOME, "results", "libcore.json"), 'r') as f:
    LIBCORE = json.load(f)

with open(os.path.join(HOME, "results", "python.json"), 'r') as f:
    PYTHON = json.load(f)

with open(os.path.join(HOME, "results", "rapidjson.json"), 'r') as f:
    RAPIDJSON = json.load(f)

with open(os.path.join(HOME, "results", "strtod.json"), 'r') as f:
    STRTOD = json.load(f)

# Not supported with most C++17 compilers.
#with open(os.path.join(HOME, "results", "from_chars.json"), 'r') as f:
#    FROM_CHARS = json.load(f)

def plot_digits():
    """Plot data for a given digit series"""

    keys = [
        "digits2",
        "digits8",
        "digits16",
        "digits32",
        "digits64",
    ]
    double_conversion = np.array([DOUBLE_CONVERSION[k][1] for k in keys])
    golang = np.array([GOLANG[k][1] for k in keys])
    lexical = np.array([LEXICAL[k][1] for k in keys])
    libcore = np.array([LIBCORE[k][1] for k in keys])
    python = np.array([PYTHON[k][1] for k in keys])
    rapidjson = np.array([RAPIDJSON[k][1] for k in keys])
    strtod = np.array([STRTOD[k][1] for k in keys])

    index = [str(int(1.5*float(k.replace("digits", "")))) for k in keys]
    data = {
        'double_conversion': double_conversion,
        'golang': golang,
        'lexical': lexical,
        'libcore': libcore,
        'python': python,
        'rapidjson': rapidjson,
        'strtod': strtod,
    }
    df = pd.DataFrame(data, index=index)
    ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14)
    ax.set_yscale('log')
    ax.set_xlabel("digits")
    ax.set_ylabel("ms/iter")
    ax.figure.tight_layout()
    ax.legend(loc=2, prop={'size': 14})
    plt.show()


def plot_series(keys, prefix):
    """Plot data for digit series."""

    double_conversion = np.array([DOUBLE_CONVERSION[k][1] for k in keys])
    golang = np.array([GOLANG[k][1] for k in keys])
    lexical = np.array([LEXICAL[k][1] for k in keys])
    libcore = np.array([LIBCORE[k][1] for k in keys])
    python = np.array([PYTHON[k][1] for k in keys])
    rapidjson = np.array([RAPIDJSON[k][1] for k in keys])
    strtod = np.array([STRTOD[k][1] for k in keys])

    index = [k.replace(prefix, "") for k in keys]
    data = {
        'double_conversion': double_conversion,
        'golang': golang,
        'lexical': lexical,
        'libcore': libcore,
        'python': python,
        'rapidjson': rapidjson,
        'strtod': strtod,
    }
    df = pd.DataFrame(data, index=index)
    ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14)
    ax.set_yscale('log')
    ax.set_xlabel("digits")
    ax.set_ylabel("ns/iter")
    ax.figure.tight_layout()
    ax.legend(loc=2, prop={'size': 14})
    plt.show()


def plot_denormal():
    """Plot the denormalized data."""

    keys = [
        "denormal10",
        "denormal20",
        "denormal30",
        "denormal40",
        "denormal50",
        "denormal100",
        "denormal200",
        "denormal400",
        "denormal800",
        "denormal1600",
        "denormal3200",
        "denormal6400",
    ]
    plot_series(keys, "denormal")


def plot_large():
    """Plot the large data."""

    keys = [
        "large10",
        "large20",
        "large30",
        "large40",
        "large50",
        "large100",
        "large200",
        "large400",
        "large800",
        "large1600",
        "large3200",
        "large6400",
    ]
    plot_series(keys, "large")


def main():
    """Plot the core data"""

    plot_denormal()
    plot_large()
    plot_digits()

if __name__== '__main__':
    main()
