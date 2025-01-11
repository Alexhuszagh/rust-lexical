'''
    docs
    ====

    Script to validate the generated documentation

    This also validates the TOML files and any links inside the docs.
'''

import html.parser
import os
import re
import sys
import shutil
import time
import urllib.error
import urllib.request
import subprocess
from pathlib import Path

# This is a hack for older Python versions
# Remove once gh actions drops support for Python < 3.11
try:
    import tomllib
except ImportError:
    import pip._vendor.tomli as tomllib

home_dir = Path(__file__).absolute().parent.parent
target_dir = home_dir / 'target' / 'doc'


class LinkParser(html.parser.HTMLParser):
    '''Custom parser that looks for links within HTML.'''

    links: set[str]
    processed: int
    file: str

    def __init__(self) -> None:
        super().__init__()
        self.links = set()
        self.processed = 0
        self.file = ''

    def feed(self, file: str, data: str) -> None:
        '''Feed data to the underlying file.'''
        self.file = file
        super().feed(data)

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str]]) -> None:
        '''Find if it's a link and if so add the data.'''

        # these are all programmatically blocked, likely cause of our user agent
        # they're stable links so it's fine
        blocked = (
            'https://sagemath.org/',
            'https://www.sagemath.org/',
            'https://www.java.com/en/',
            'https://coffeescript.org/',
        )
        # skip our non-link tags and our local/script links
        if tag != 'a':
            return
        attributes = dict(attrs)
        assert 'href' in attributes
        href = attributes['href']
        if not href.startswith(('https://', 'http://')):
            return
        if href.startswith(blocked):
            return

        # try to avoid getting deny-listed or rate limited
        self.processed += 1
        if href not in self.links:
            try:
                request = urllib.request.Request(href)
                # spoof this to avoid getting blocked
                request.add_header('User-Agent', 'Mozilla/5.0 (X11; U; Linux i686)')
                # NOTE: `crates.io` requires an `Accept: text/html`
                #   https://github.com/rust-lang/crates.io/issues/788
                if href.startswith(('https://crates.io', 'https://www.crates.io')):
                    request.add_header('Accept', 'text/html')
                response = urllib.request.urlopen(request)
            except urllib.error.HTTPError as error:
                if error.code in (401, 403):
                    return
                msg = f'Got an invalid href "{href}" with code "{error.code}" for file "{self.file}".'
                raise ValueError(msg)
            time.sleep(0.2)
            code = response.code
            if response.code != 200:
                raise ValueError(f'Got an invalid href "{href}" with code "{code}" for file "{self.file}".')
            self.links.add(href)

        if self.processed > 1 and self.processed % 200 == 0:
            print(f'Processing link {self.processed}...')

    def handle_endtag(self, tag: str) -> None:
        '''Handle the closing of a tag (ignored).'''
        _ = tag

    def handle_data(self, data: str):
        '''Handle any raw data (we ignore this).'''
        _ = data


def validate_toml() -> None:
    '''Validate our TOML files.'''

    for path in home_dir.rglob('**/*.toml'):
        # Bug fixes for Docker on Windows. We don't want dups anyway.
        if path.is_symlink():
            continue
        print(f'Processing TOML file "{path.relative_to(home_dir)}"...')
        # NOTE: This is a workaround for Python < 3.11, since `tomli`
        # expects a `str` and `tomllib` expects `bytes` for `load`.
        # Also, `resolve` is a bug fix for symlinks prior to 3.11.
        # `as_posix()` is a bug fix for symbolic links on Windows docker.
        with open(path.absolute().as_posix(), encoding='utf-8') as file:
            data = file.read()
        _ = tomllib.loads(data)


def validate_links() -> None:
    '''Validate all the links inside our build documentation.'''

    parser = LinkParser()
    for path in target_dir.rglob('**/*.html'):
        # Bug fixes for Docker on Windows. We don't want dups anyway.
        if path.is_symlink():
            continue
        with path.open(encoding='utf-8') as file:
            data = file.read()
        parser.feed(path.name, data)

    # deduplicate and validate all our links
    print(f'Processed and validated {len(parser.links)} links...')


cargo_toml = '''
[package]
authors = ["Alex Huszagh <ahuszagh@gmail.com>"]
edition = "2021"
name = "lexical-format-doctests"
publish = false

[workspace]
members = []

[dependencies.lexical-core]
path = "../../lexical-core"
features = ["format", "radix"]
'''

test_prefix = '''
#![allow(unused, dead_code)]

use core::num;

use lexical_core::*;

const PF_OPTS: ParseFloatOptions = ParseFloatOptions::new();
const PI_OPTS: ParseIntegerOptions = ParseIntegerOptions::new();
const WF_OPTS: WriteFloatOptions = WriteFloatOptions::new();
const WI_OPTS: WriteIntegerOptions = WriteIntegerOptions::new();
'''

test_rs = '''
#[test]
pub fn test{index}() {{
    {test}
}}
'''


def validate_format() -> int:
    '''Validate all the format features inside our docs.'''

    # read all our tests
    with (home_dir / 'lexical-util' / 'src' / 'format_builder.rs').open(encoding='utf-8') as file:
        data = file.read()
    tests = [i.group(1) for i in re.finditer(r'<!--\s*TEST\s*(.*?)-->', data, re.DOTALL)]
    tests = [re.sub(r'(?:\A|[\r\n]+)\s*///?', '\n', i) for i in tests]
    tests = [i.strip().removeprefix('```rust').removesuffix('```') for i in tests]

    # create a fake project inside target
    proj_dir = home_dir / 'target' / 'format-doctest'
    src_dir = proj_dir / 'src'
    tests_dir = proj_dir / 'tests'
    shutil.rmtree(proj_dir, ignore_errors=True)
    proj_dir.mkdir(parents=True)
    src_dir.mkdir()
    tests_dir.mkdir()

    # create basic project
    with (proj_dir / 'Cargo.toml').open(encoding='utf-8', mode='w') as file:
        print(cargo_toml, file=file)
    with (src_dir / 'lib.rs').open(encoding='utf-8', mode='w'):
        pass
    with (tests_dir / 'test.rs').open(encoding='utf-8', mode='w') as file:
        print(test_prefix, file=file)
        for index, test in enumerate(tests):
            print(test_rs.format(index=index, test=test), file=file)

    # build our tests
    cargo = os.environ.get('CARGO', 'cargo')
    return subprocess.call(f'{cargo} test', cwd=proj_dir, shell=True)


def main() -> None:
    '''Run our validation code.'''

    if 'SKIP_TOML' not in os.environ:
        validate_toml()
    if 'SKIP_LINKS' not in os.environ:
        validate_links()
    if 'SKIP_FORMAT' not in os.environ:
        sys.exit(validate_format())


if __name__ == '__main__':
    main()
