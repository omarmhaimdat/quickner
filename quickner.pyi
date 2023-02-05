from typing import Optional, List, Tuple, Set
from enum import Enum

LABEL = List[Tuple[int, int, str]]

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

class Annotation:
    """
    Annotation object.

    Attributes:
        id (int): Id of the annotation.
        text (str): Text of the annotation.
        label (LABEL): Label of the annotation.
    """
    def __init__(self, id: int, text: str, label: LABEL) -> None: ...
    def __repr__(self) -> str: ...

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
        annotations (List[Annotation]): List of annotations.
        entities (List[Entity]): List of entities.
        texts (List[Text]): List of texts.

    Methods:
        process(save: bool = False): Process texts and entities to generate annotations.
        save_annotations(path: str = None, format: Format = Format.JSONL): Save annotations to a file.
    """
    config_file: str
    config: Config
    annotations: List[Annotation]
    entities: List[Entity]
    texts: List[Text]

    def __init__(self, config_file: Optional[str] = None) -> None: ...
    def process(self, save: Optional[bool] = False) -> List[Annotation]: ...
    def save_annotations(self, path: Optional[str] = None, format: Optional[Format] = Format.JSONL) -> None: ...