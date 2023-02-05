# Quickner

A simple, fast, and easy to use NER annotator for Python.

## Installation

```bash
pip install quickner
```

## Usage

```python
from quickner import Quickner

# Initialize the annotator
annotator = Quickner() # or Quickner("config.toml")

# Annotate the texts using the config file
annotator.process() # or annotator.process(True) to save the annotated data to a file
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

## License

[MOZILLA PUBLIC LICENSE Version 2.0](LICENSE)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Authors

- [**Omar MHAIMDAT**]
