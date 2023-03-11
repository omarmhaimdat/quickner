
<div align="center">
  <h1 style="font-size:40px;">Quickner ⚡ </h1>
  <p>
    <strong style="font-size:20px;">A simple, fast, and easy to use NER annotator for Python</strong>
  </p>
  <p>
    <a href="https://badge.fury.io/py/quickner"><img src="https://badge.fury.io/py/quickner.svg" alt="PyPI version" height="18"></a>
    <a href="https://pypi.org/project/quickner/"><img src="https://img.shields.io/badge/License-Mozilla%20Public%20License%20Version%202.0-orange" alt="License" height="18"></a>
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

# Create a virtual environment
python3 -m venv env
source env/bin/activate

# Install quickner
pip install quickner # or pip3 install quickner
```

## Usage

### Using the config file

```python
from quickner import Quickner, Config

config = Config(path="config.toml") # or Config() if the config file is in the current directory

# Initialize the annotator
quick = Quickner(config=config)

# Annotate the texts using the config file
quick.process() # or annotator.process(True) to save the annotated data to a file
```

### Using Documents

```python
from quickner import Quickner, Document

# Create documents
rust = Document("rust is made by Mozilla")
python = Document("Python was created by Guido van Rossum")
java = Document("Java was created by James Gosling")

# Documents can be added to a list
documents = [rust, python, java]

# Initialize the annotator

quick = Quickner(documents=documents)
quick
>>> Entities: 0 | Documents: 3 | Annotations:
>>> quick.documents
[Document(id="87e03d58b1ba4d72", text=rust is made by Mozilla, label=[]), Document(id="f1da5d23ef88f3dc", text=Python was created by Guido van Rossum, label=[]), Document(id="e4324f9818e7e598", text=Java was created by James Gosling, label=[])]
>>> quick.entities
[]
```

### Using Documents and Entities

```python
from quickner import Quickner, Document, Entity

# Create documents from texts
texts = (
  "rust is made by Mozilla",
  "Python was created by Guido van Rossum",
  "Java was created by James Gosling at Sun Microsystems",
  "Swift was created by Chris Lattner and Apple",
)
documents = [Document(text) for text in texts]

# Create entities
entities = (
  ("Rust", "PL"),
  ("Python", "PL"),
  ("Java", "PL"),
  ("Swift", "PL"),
  ("Mozilla", "ORG"),
  ("Apple", "ORG"),
  ("Sun Microsystems", "ORG"),
  ("Guido van Rossum", "PERSON"),
  ("James Gosling", "PERSON"),
  ("Chris Lattner", "PERSON"),
)
entities = [Entity(*(entity)) for entity in entities]

# Initialize the annotator
quick = Quickner(documents=documents, entities=entities)
quick.process()

>>> quick
Entities: 6 | Documents: 3 | Annotations: PERSON: 2, PL: 3, ORG: 1
>>> quick.documents 
[Document(id=87e03d58b1ba4d72, text=rust is made by Mozilla, label=[(0, 4, PL), (16, 23, ORG)]), Document(id=f1da5d23ef88f3dc, text=Python was created by Guido van Rossum, label=[(0, 6, PL), (22, 38, PERSON)]), Document(id=e4324f9818e7e598, text=Java was created by James Gosling, label=[(0, 4, PL), (20, 33, PERSON)])]
```

### Find documents by label or entity

When you have annotated your documents, you can use the `find_documents_by_label` and `find_documents_by_entity` methods to find documents by label or entity.

Both methods return a list of documents, and are not case sensitive.

Example:

```python

# Find documents by label
>>> quick.find_documents_by_label("PERSON")
[Document(id=f1da5d23ef88f3dc, text=Python was created by Guido van Rossum, label=[(0, 6, PL), (22, 38, PERSON)]), Document(id=e4324f9818e7e598, text=Java was created by James Gosling, label=[(0, 4, PL), (20, 33, PERSON)])]

# Find documents by entity
>>> quick.find_documents_by_entity("Guido van Rossum")
[Document(id=f1da5d23ef88f3dc, text=Python was created by Guido van Rossum, label=[(0, 6, PL), (22, 38, PERSON)])]
>>> quick.find_documents_by_entity("rust")
[Document(id=87e03d58b1ba4d72, text=rust is made by Mozilla, label=[(0, 4, PL), (16, 23, ORG)])]
>>> quick.find_documents_by_entity("Chris Lattner")
[Document(id=3b0b3b5b0b5b0b5b, text=Swift was created by Chris Lattner and Apple, label=[(0, 5, PL), (21, 35, PERSON), (40, 45, ORG)])]
```

### Get a Spacy Compatible Generator Object

You can use the `spacy` method to get a spacy compatible generator object.

The generator object can be used to feed a spacy model with the annotated data, you still need to convert the data into DocBin format.

Example:

```python
# Get a spacy compatible generator object
>>> quick.spacy()
<builtins.SpacyGenerator object at 0x102311440>
# Divide the documents into chunks
>>> chunks = quick.spacy(chunks=2)
>>> for chunk in chunks:
...     print(chunk)
...
[('rust is made by Mozilla', {'entitiy': [(0, 4, 'PL'), (16, 23, 'ORG')]}), ('Python was created by Guido van Rossum', {'entitiy': [(0, 6, 'PL'), (22, 38, 'PERSON')]})]
[('Java was created by James Gosling at Sun Microsystems', {'entitiy': [(0, 4, 'PL'), (20, 33, 'PERSON'), (37, 53, 'ORG')]}), ('Swift was created by Chris Lattner and Apple', {'entitiy': [(0, 5, 'PL'), (21, 34, 'PERSON'), (39, 44, 'ORG')]})]
```

### Single document annotation

You can also annotate a single document with a list of entities.

This is useful when you want to annotate a document with a list of entities is not in the list of entities of the Quickner object.

Example:

```python
from quickner import Document, Entity

# Create a document from a string
# Method 1
rust = Document.from_string("rust is made by Mozilla")
# Method 2
rust = Document("rust is made by Mozilla")

# Create a list of entities
entities = [Entity("Rust", "PL"), Entity("Mozilla", "ORG")]
# Annotate the document with the entities, case_sensitive is set to False by default
>>> rust.annotate(entities, case_sensitive=True)
>>> rust
Document(id="87e03d58b1ba4d72", text=rust is made by Mozilla, label=[(16, 23, ORG)])
>>> rust.annotate(entities, case_sensitive=False)
>>> rust
Document(id="87e03d58b1ba4d72", text=rust is made by Mozilla, label=[(16, 23, ORG), (0, 4, PL)])
```

### Load from file

Initialize the Quickner object from a file containing existing annotations.

`Quickner.from_jsonl` and `Quickner.from_spacy` are class methods that return a Quickner object and are able to parse the annotations and entities from a jsonl or spaCy file.

```python
from quickner import Quickner

quick = Quickner.from_jsonl("annotations.jsonl") # load the annotations from a jsonl file
quick = Quickner.from_spacy("annotations.json") # load the annotations from a spaCy file
```

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

## Features Roadmap and TODO

- [x] Add support for spaCy format
- [x] Add support for brat format
- [x] Add support for conll format
- [x] Add support for jsonl format
- [x] Add support for loading annotations from a json spaCy file
- [x] Add support for loading annotations from a jsonl file
- [x] Find documents with a specific entity/entities and return the documents
- [ ] Add support for loading annotations from a brat file
- [ ] Substring search for entities in the text (case sensitive and insensitive)
- [ ] Partial match for entities, e.g. "Rust" will match "Rustlang"
- [ ] Pattern/regex based entites, e.g. "Rustlang" will match "Rustlang 1.0"
- [ ] Fuzzy match for entities with levenstein distance, e.g. "Rustlang" will match "Rust"
- [ ] Add support for jupyter notebook

## License

[MOZILLA PUBLIC LICENSE Version 2.0](LICENSE)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Authors

- [**Omar MHAIMDAT**]
