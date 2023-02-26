all: build test
.PHONY: build
build:
	pip install .
.PHONY: test
test:
	python tests/test.py