from typing import Optional, List, Tuple, NewType
from enum import Enum

Label = NewType("Label", List[Tuple[int, int, str]])

class Text:
    """
    Text object.

    Attributes:
        text (str): String object of the text.
    """
    def __init__(self, text: str) -> None: ...

class Entity:
    """
    Entity object.

    Attributes:
        name (str): Name of the entity.
        label (str): Label of the entity.
    """
    def __init__(self, name: str, label: str) -> None: ...

class Document:
    """
    Document object.

    Attributes:
        id (int): Id of the annotation.
        text (str): Text of the annotation.
        label (Label): Label of the annotation.
    """
    label: Label
    id : int
    text: str

    def __init__(self, id: int, text: str, label: Label) -> None: ...
    def __repr__(self) -> str: ...
    @staticmethod
    def from_string(text: str) -> Document: ...
    def annotate(self, text: str, entities: List[Entity], case_sensitive: bool = False) -> None: ...


class Input:
    """
    Input configuration object.

    Attributes:
        path (str): Path to the input file.
        filter (bool): Use filters. Default is False.
    """
    path: str
    filter: bool

class Filters:
    """
    Filters configuration object.

    Attributes:
        alphanumeric (bool): Filter alphanumeric characters. Default is False.
        case_sensitive (bool): Filter case sensitive characters. Default is False.
        min_length (int): Filter characters with a minimum length. Default is 0.
        max_length (int): Filter characters with a maximum length. Default is 1024.
        punctuation (bool): Filter punctuation characters. Default is False.
        numbers (bool): Filter tokens made exclusively of numbers. Default is False.
        special_characters (bool): Filter special characters. Default is False.
        accept_special_characters (str): Accept special characters. Default is None.
        list_of_special_characters (List[str]): List of special characters to accept.
        Default is a list of special characters.
    """
    alphanumeric: bool
    case_sensitive: bool
    min_length: int
    max_length: int
    punctuation: bool
    numbers: bool
    special_characters: bool
    accept_special_characters: Optional[str]
    list_of_special_characters: Optional[List[str]]

class Texts:
    """
    Texts configuration object.

    Attributes:
        input (Input): Input configuration.
        filters (Filters): Filters configuration.
    """
    input: Input
    filters: Filters

class Output:
    """
    Output configuration object.

    Attributes:
        path (str): Path to the output file.
    """
    path: str

class Format(Enum):
    """
    Format of the output file.
    """
    CONLL = "conll"
    JSON = "json"
    SPACY = "spacy"
    BRAT = "brat"
    JSONL = "jsonl"

class AnnotationsConfig:
    """
    Annotations configuration object.

    Attributes:
        output (Output): Output configuration.
        format (Format): Format of the output file. Default is "jsonl".
        Possible values are "conll", "json", "spacy", "brat", "jsonl".
    """
    output: Output
    format: Format

class Excludes:
    """
    Excludes configuration object.

    Attributes:
        path (str): Path to the file containing the entities to exclude.
    """
    path: str

class Entities:
    """
    Entities configuration object.

    Attributes:
        input (Input): Input configuration.
        excludes (Excludes): Excludes configuration.
        filters (Filters): Filters configuration.
    """
    input: Input
    excludes: Excludes
    filters: Filters

class Logging:
    """
    Logging configuration object.

    Attributes:
        level (str): Logging level. Default is "info".
        Possible values are "debug", "info", "warning", "error", "critical".
    """
    level: str

class Config:
    """
    Configuration object, parsed from a TOML file.

    Attributes:
        texts (Texts): Texts configuration.
        annotations (AnnotationsConfig): Annotations configuration.
        entities (Entities): Entities configuration.
        logging (Logging): Logging configuration.
    """
    texts: Texts
    annotations: AnnotationsConfig
    entities: Entities
    logging: Logging

class Quickner:
    """
    Quickner class to process texts and entities to generate annotations.

    Parameters:
        config_file (str): Path to the configuration file.
    
    Attributes:
        config_file (str): Path to the configuration file.
        config (Config): Configuration object.
        documents (List[Document]): List of documents.
        entities (List[Entity]): List of entities.

    Methods:
        process(save: bool = False): Process texts and entities to generate annotations.
        save_annotations(path: str = None, format: Format = Format.JSONL): Save annotations to a file.
    """
    config_file: str
    config: Config
    documents: List[Document]
    entities: List[Entity]

    def __init__(self, config_file: Optional[str] = None) -> None: ...
    def process(self, save: Optional[bool] = False) -> None: ...
    def save_annotations(self, path: Optional[str] = None, format: Optional[Format] = Format.JSONL) -> None: ...
    def to_jsonl(self, path: Optional[str] = None) -> None:
        """
        Save annotations to a JSONL file.

        Parameters:
            path (str): Path to the output file. Default is the path defined in the configuration file.

        Returns:
            None
        """
        ...
    def to_csv(self, path: Optional[str] = None) -> None:
        """
        Save annotations to a CSV file.

        Parameters:
            path (str): Path to the output file. Default is the path defined in the configuration file.

        Returns:
            None
        """
        ...
    
    def to_spacy(self, path: Optional[str] = None) -> None:
        """
        Save annotations to a Spacy file.

        Parameters:
            path (str): Path to the output file. Default is the path defined in the configuration file.

        Returns:
            None
        """
        ...

# Module functions

def from_jsonl(path: str) -> Quickner:
    """
    Create a Quickner object from a JSONL file.

    Parameters:
        path (str): Path to the JSONL file.

    Returns:
        Quickner: Quickner object with:
        - the annotations loaded from the JSONL file
        - the entities loaded from the JSONL file and infered from the annotations
        - the texts loaded from the JSONL file
        - A default configuration
    """
    ...

def from_spacy(path: str) -> Quickner:
    """
    Create a Quickner object from a Spacy file.

    Parameters:
        path (str): Path to the Spacy file.

    Returns:
        Quickner: Quickner object with:
        - the annotations loaded from the Spacy file
        - the entities loaded from the Spacy file and infered from the annotations
        - the texts loaded from the Spacy file
        - A default configuration
    """
    ...