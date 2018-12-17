#!/usr/bin/env python3

import json
import os

HOME = os.path.dirname(os.path.realpath(__file__))
DATA = os.path.join(HOME, "..", "data")

with open(os.path.join(DATA, "denormal_halfway.json")) as f:
    DENORMAL_DATA = json.load(f)

with open(os.path.join(DATA, "large_halfway.json")) as f:
    LARGE_DATA = json.load(f)

with open(os.path.join(DATA, "digits2.json")) as f:
    DIGITS2_DATA = json.load(f)

with open(os.path.join(DATA, "digits8.json")) as f:
    DIGITS8_DATA = json.load(f)

with open(os.path.join(DATA, "digits16.json")) as f:
    DIGITS16_DATA = json.load(f)

with open(os.path.join(DATA, "digits32.json")) as f:
    DIGITS32_DATA = json.load(f)

with open(os.path.join(DATA, "digits64.json")) as f:
    DIGITS64_DATA = json.load(f)
