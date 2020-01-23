"""
    runtests
    ========

    Run the C, C++, and Python test suites for the C-API.
"""

import itertools as it
import os
import shutil
import subprocess
import sys

# DEFAULT TOOLS
PYTHON = sys.executable
CARGO = os.environ.get('CARGO', 'cargo')
CMAKE = os.environ.get('CMAKE', 'cmake')
CTEST = os.environ.get('CTEST', 'ctest')
BUILD_TYPE = 'Debug'

# PATHS
PROJECT_DIR = os.path.dirname(os.path.realpath(__file__))
TARGET_DIR = os.path.join(PROJECT_DIR, "target")
RELEASE_DIR = os.path.join(PROJECT_DIR, "release")
TESTS_DIR = os.path.join(PROJECT_DIR, 'tests')
BUILD_DIR = os.path.join(TESTS_DIR, 'build')
if not os.path.exists(BUILD_DIR):
    os.mkdir(BUILD_DIR)


def build_lexical(features):
    '''Build the lexical shared library.'''

    os.chdir(PROJECT_DIR)
    args = [CARGO, 'build', '--release']
    if features:
        args.append('--features=' + ','.join(features))
    subprocess.check_call(args)

def run_python_tests():
    '''Run the Python unittest suite.'''

    os.chdir(TESTS_DIR)
    subprocess.check_call([PYTHON, 'test_py.py'])

def run_c_tests():
    '''Run the C/C++ unittest suite.'''

    os.chdir(BUILD_DIR)

    # Clean up previous builds.
    try:
        os.remove(os.path.join(BUILD_DIR, 'CMakeCache.txt'))
        shutil.rmtree(os.path.join(BUILD_DIR, 'CMakeFiles'))
    except OSError:
        pass
    subprocess.check_call([CMAKE, TESTS_DIR, '-DCMAKE_BUILD_TYPE={}'.format(BUILD_TYPE)])

    # Build and test.
    subprocess.check_call([CMAKE, '--build', BUILD_DIR, '--config', BUILD_TYPE])
    subprocess.check_call([CTEST, '-C', BUILD_TYPE, '-V'])

def run_suite(features):
    '''Run suite of tests for a given set of features.'''

    print("------------------------------------------------------------")
    print("Running lexical test suite with features={}".format(features))
    print("------------------------------------------------------------")
    build_lexical(features)
    run_python_tests()
    run_c_tests()

def main():
    '''Run the main code block.'''

    # Only test combinations of format and radix.
    feature_list = ['format', 'radix']
    for length in range(len(feature_list)+1):
        for features in it.combinations(feature_list, length):
            run_suite(features)

    # After, test other features.
    run_suite(('rounding',))

if __name__ == '__main__':
    main()
