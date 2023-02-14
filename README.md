
<div align="center">
  <h1>Quickner ⚡ </h1>
  <p>
    <strong>A simple, fast, and easy to use NER annotator for Python</strong>
  </p>
  <p>
    <a href="https://badge.fury.io/py/quickner"><img src="https://badge.fury.io/py/quickner.svg" alt="PyPI version" height="18"></a>
    <a href="https://pypi.org/project/quickner/"><img src="https://img.shields.io/pypi/l" alt="License" height="18"></a>
    <a href="https://pypi.org/project/quickner/"><img src="https://img.shields.io/pypi/dm/quickner" alt="PyPI - Downloads" height="18"></a>
    <a href="https://actions-badge.atrox.dev/omarmhaimdat/quickner/goto?ref=master"><img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fomarmhaimdat%2Fquickner%2Fbadge%3Fref%3Dmaster&style=flat" alt="Build Status" height="18"></a>
  </p>
  <p>
    <img src="quickner.gif" alt="Showcase">
  </p>
</div>

<!-- 
[![PyPI version](https://badge.fury.io/py/quickner.svg)](https://badge.fury.io/py/quickner)
![License](https://img.shields.io/pypi/l) ![PyPI - Downloads](https://img.shields.io/pypi/dm/quickner)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fomarmhaimdat%2Fquickner%2Fbadge%3Fref%3Dmaster&style=flat)](https://actions-badge.atrox.dev/omarmhaimdat/quickner/goto?ref=master)

![Showcase](showcase.gif) -->


Quickner is a new tool to quickly annotate texts for NER (Named Entity Recognition). It is written in Rust and accessible through a Python API.

Quickner is blazing fast, simple to use, and easy to configure using a TOML file.

## Installation

```bash
pip install quickner # or pip3 install quickner
```

## Usage

```python
from quickner import Quickner

# Initialize the annotator
quick = Quickner() # or Quickner("config.toml") if the config file is not in the current directory

# Annotate the texts using the config file
quick.process() # or annotator.process(True) to save the annotated data to a file

quick.to_jsonl("annotations.jsonl") # save the annotations to a jsonl file
quick.to_spacy("annotations.spacy") # save the annotations to a spaCy file
```

## Load from file

Initialize the Quickner object from a file containing existing annotations.

`Quickner.from_jsonl` and `Quickner.from_spacy` are class methods that return a Quickner object and are able to parse the annotations and entities from a jsonl or spaCy file.

```python
from quickner import Quickner

quick = Quickner.from_jsonl("annotations.jsonl") # load the annotations from a jsonl file
quick = Quickner.from_spacy("annotations.json") # load the annotations from a spaCy file
```


## Single text annotation

```python
from quickner import Document, Entity

# Create a document from a string
rust = Document.from_string("rust is made by Mozilla")
# Create a list of entities
entities = [Entity("Rust", "Programming Language"), Entity("Mozilla", "ORG")]
# Annotate the document with the entities, case_sensitive is set to False by default
rust.annotate(entities, case_sensitive=True)
>>> rust
Document(id=0, text=Rust is made by Mozilla, label=[(0, 4, Programming Language), (16, 23, ORG)])

## Configuration

The configuration file is a TOML file with the following structure:

```toml
# Configuration file for the NER tool

[general]
# Mode to run the tool, modes are:
# Annotation from the start
# Annotation from already annotated texts
# Load annotations and add new entities

[logging]
level = "debug" # level of logging (debug, info, warning, error, fatal)

[texts]

[texts.input]
filter = false     # if true, only texts in the filter list will be used
path = "texts.csv" # path to the texts file

[texts.filters]
accept_special_characters = ".,-" # list of special characters to accept in the text (if special_characters is true)
alphanumeric = false              # if true, only strictly alphanumeric texts will be used
case_sensitive = false            # if true, case sensitive search will be used
max_length = 1024                 # maximum length of the text
min_length = 0                    # minimum length of the text
numbers = false                   # if true, texts with numbers will not be used
punctuation = false               # if true, texts with punctuation will not be used
special_characters = false        # if true, texts with special characters will not be used

[annotations]
format = "spacy" # format of the output file (jsonl, spaCy, brat, conll)

[annotations.output]
path = "annotations.jsonl" # path to the output file

[entities]

[entities.input]
filter = true         # if true, only entities in the filter list will be used
path = "entities.csv" # path to the entities file
save = true           # if true, the entities found will be saved in the output file

[entities.filters]
accept_special_characters = ".-" # list of special characters to accept in the entity (if special_characters is true)
alphanumeric = false             # if true, only strictly alphanumeric entities will be used
case_sensitive = false           # if true, case sensitive search will be used
max_length = 20                  # maximum length of the entity
min_length = 0                   # minimum length of the entity
numbers = false                  # if true, entities with numbers will not be used
punctuation = false              # if true, entities with punctuation will not be used
special_characters = true        # if true, entities with special characters will not be used

[entities.excludes]
# path = "excludes.csv" # path to entities to exclude from the search

```

## License

[MOZILLA PUBLIC LICENSE Version 2.0](LICENSE)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Authors

- [**Omar MHAIMDAT**]
