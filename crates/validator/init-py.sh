#!/usr/bin/env bash

cd "$(dirname "$0")/src/shacl-validator"

source venv/bin/activate
python3 -m ensurepip --upgrade
pip install -r requirements.txt --upgrade
