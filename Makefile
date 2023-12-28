SHELL := /bin/bash

run:
	python3 -m venv .env
	source .env/bin/activate
	pip install numpy; pip install opencv-python; pip install maturin
	maturin develop
	python example.py