#!/bin/bash

rustc parse.rs --codegen opt-level=3 -o parse
