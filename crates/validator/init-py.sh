#!/usr/bin/env bash

cd "$(dirname "$0")/shacl-validator"

python3 -m venv "venv"
source venv/bin/activate
venv/bin/python -m ensurepip --upgrade
pip install -r requirements.txt --upgrade
