all: build test
install: setup build
.PHONY: build
build:
	pip install .
.PHONY: test
test:
	python tests/test.py

setup:
	python3 -m venv env && source env/bin/activate
	pip install maturin

.PHONY: bench
bench:
	python tests/performance.py
