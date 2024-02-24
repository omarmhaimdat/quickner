from ast import Dict
from typing import Iterator, Optional, List, Tuple, NewType, overload
from enum import Enum

Label = NewType("Label", List[Tuple[int, int, str]])

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
    id: int
    text: str

    def __init__(self, text: str, label: Optional[Label]) -> None: ...
    def __repr__(self) -> str: ...
    @staticmethod
    def from_string(text: str) -> Document: ...
    def annotate(self, entities: List[Entity], case_sensitive: bool = False) -> None:
        """
        Annotate a text with entities.

        Parameters:
            entities (List[Entity]): List of entities to use for annotation.
            case_sensitive (bool): Case sensitive annotation. Default is False.

        Returns:
            None
        """
        ...

    def pretty(self) -> str:
        """
        Pretty print the document.

        Returns:
            str: Pretty print of the document.
        """
        ...

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

    def __init__(self, config_file: str) -> None: ...

class Quickner:
    """
    Quickner class to process texts and entities to generate annotations.

    Parameters:
        documents (List[Document]): List of documents.
        entities (List[Entity]): List of entities.
        config (Config): Configuration object.

    Attributes:
        documents (List[Document]): List of documents.
        entities (List[Entity]): List of entities.
        config (Config): Configuration object.

    Methods:
        process(save: bool = False): Process texts and entities to generate annotations.
        save_annotations(path: str = None, format: Format = Format.JSONL): Save annotations to a file.
    """

    config_file: str
    config: Config
    documents: List[Document]
    entities: List[Entity]

    @overload
    def __init__(self) -> None: ...
    @overload
    def __init__(self, documents: List[Document]) -> None: ...
    @overload
    def __init__(self, entities: List[Entity]) -> None: ...
    @overload
    def __init__(self, documents: List[Document], entities: List[Entity]) -> None: ...
    @overload
    def __init__(
        self, documents: List[Document], entities: List[Entity], config: Config
    ) -> None: ...
    @overload
    def __init__(self, documents: List[Document], config: Config) -> None: ...
    def process(self, save: Optional[bool] = False) -> None: ...
    def save_annotations(
        self, path: Optional[str] = None, format: Optional[Format] = Format.JSONL
    ) -> None: ...
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

    def spacy(
        self, chunks: Optional[int] = None
    ) -> Iterator[List[Dict["Entity", List[Tuple[int, int, str]]]]]:
        """
        Generate Spacy documents.

        Parameters:
            chunks (int): Number of documents to split the list of documents into. Default is None.

        Returns:
            Iterator[List[Dict["entity", List[Tuple[int, int, str]]]]]: Iterator of List of Spacy Format.
        """
        ...

    def add_document(self, document: Document) -> None:
        """
        Add a document to the list of documents.

        Parameters:
            document (Document): Document to add.

        Returns:
            None
        """
        ...

    def add_entity(self, entity: Entity) -> None:
        """
        Add an entity to the list of entities. If the entity already exists, it will be ignored.

        Parameters:
            entity (Entity): Entity to add.

        Returns:
            None
        """
        ...

    def find_documents_by_label(self, label: str) -> List[Document]:
        """
        Find documents with a specific label.

        Parameters:
            label (str): Label to search.

        Returns:
            List[Document]: List of documents with the label.
        """
        ...

    def find_documents_by_entity(self, name: str) -> List[Document]:
        """
        Find documents with a specific entity.
        >>> quickner.find_documents_by_entity("John")
        [Document(id="f9c68f53ee5319c8", text=John is a person., [[0, 4, "PERSON"]])]

        Parameters:
            name (str): Name of the entity to find.

        Returns:
            List[Document]: List of documents with the entity.
        """
        ...

    def numpy(self) -> NDArray:  # noqa: F821
        """
        Convert the list of documents to a Numpy array.

        Returns:
            NDArray: Numpy array of documents.
        """
        ...
