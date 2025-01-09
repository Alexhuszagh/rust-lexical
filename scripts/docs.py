'''
    docs
    ====

    Script to validate the generated documentation

    This also validates the TOML files and any links inside the docs.
'''

import html.parser
import time
import urllib.error
import urllib.request
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


def main() -> None:
    '''Run our validation code.'''

    # get all our toml files
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

    # get all our links
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


if __name__ == '__main__':
    main()
