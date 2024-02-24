import unittest

from quickner import Quickner, Document, Entity


# TODO(Omar): Significantly improve tests with pytest
class TestQuickner(unittest.TestCase):
    texts = (
        "rust is made by Mozilla",
        "Python was created by Guido van Rossum",
        "Java was created by James Gosling at Sun Microsystems",
        "Swift was created by Chris Lattner and Apple",
        "You can find more information about Rust at https://www.rust-lang.org/",
    )

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

    annotations = (
        ("rust is made by Mozilla", [(0, 4, "PL"), (16, 23, "ORG")]),
        ("Python was created by Guido van Rossum", [(0, 6, "PL"), (22, 38, "PERSON")]),
        (
            "Java was created by James Gosling at Sun Microsystems",
            [(0, 4, "PL"), (20, 33, "PERSON"), (37, 53, "ORG")],
        ),
        (
            "Swift was created by Chris Lattner and Apple",
            [(0, 5, "PL"), (21, 34, "PERSON"), (39, 44, "ORG")],
        ),
    )

    def _test_correct(self, documents: list):
        for document in documents:
            for annotation in self.annotations:
                if document.text == annotation[0]:
                    self.assertEqual(document.label, annotation[1])

    def test_quickner_from_documents(self):
        documents = [Document(text) for text in self.texts]
        entities = [Entity(*(entity)) for entity in self.entities]
        quick = Quickner(documents=documents)
        self.assertEqual(len(quick.documents), 5)
        self.assertEqual(len(quick.entities), 0)
        quick.entities = entities
        quick.process()
        # Check if all entities are labeled correctly
        labels_count = sum(len(document.label) for document in quick.documents)
        self.assertEqual(labels_count, 12)
        self._test_correct(quick.documents)

    def test_quickner_from_documents_and_entities(self):
        entities = [Entity(*(entity)) for entity in self.entities]
        documents = [Document(text) for text in self.texts]
        quick = Quickner(documents=documents, entities=entities)
        self.assertEqual(len(quick.documents), 5)
        self.assertEqual(len(quick.entities), 10)
        quick.process()
        labels_count = sum(len(document.label) for document in quick.documents)
        self.assertEqual(labels_count, 12)
        self._test_correct(quick.documents)

    def test_find_document_by_label(self):
        entities = [Entity(*(entity)) for entity in self.entities]
        documents = [Document(text) for text in self.texts]
        quick = Quickner(documents=documents, entities=entities)
        quick.process()
        documents = quick.find_documents_by_label("PL")
        self.assertEqual(len(documents), 5)
        documents = quick.find_documents_by_label("ORG")
        self.assertEqual(len(documents), 3)
        documents = quick.find_documents_by_label("PERSON")
        self.assertEqual(len(documents), 3)
        self._test_correct(quick.documents)

    def test_setting_documents(self):
        entities = [Entity(*(entity)) for entity in self.entities]
        documents = [Document(text) for text in self.texts]
        quick = Quickner(documents=documents, entities=entities)
        quick.process()
        quick.documents = []
        self.assertEqual(len(quick.documents), 0)
        quick.documents = documents
        quick.process()
        self.assertEqual(len(quick.documents), 5)
        docs = quick.find_documents_by_entity("Rust")
        self.assertEqual(len(docs), 2)
        docs = quick.find_documents_by_entity("Python")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("Java")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("Swift")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("Mozilla")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("Apple")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("Sun Microsystems")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("Guido van Rossum")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("James Gosling")
        self.assertEqual(len(docs), 1)
        docs = quick.find_documents_by_entity("Chris Lattner")
        self.assertEqual(len(docs), 1)

    def test_find_document_by_entity(self):
        entities = [Entity(*(entity)) for entity in self.entities]
        all_documents = [Document(text) for text in self.texts]
        quick = Quickner(documents=all_documents, entities=entities)
        quick.process()
        documents = quick.find_documents_by_entity("Rust")
        self.assertEqual(len(documents), 2)
        documents = quick.find_documents_by_entity("Python")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("Java")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("Swift")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("Mozilla")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("Apple")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("Sun Microsystems")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("Guido van Rossum")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("James Gosling")
        self.assertEqual(len(documents), 1)
        documents = quick.find_documents_by_entity("Chris Lattner")
        self.assertEqual(len(documents), 1)

    def test_get_spacy_generator(self):
        entities = [Entity(*(entity)) for entity in self.entities]
        documents = [Document(text) for text in self.texts]
        quick = Quickner(documents=documents, entities=entities)
        quick.process()
        generator = quick.spacy()
        self.assertEqual(len(list(generator)), 1)
        generator = quick.spacy(chunks=2)
        self.assertEqual(len(list(generator)), 3)
        generator = quick.spacy(chunks=3)
        self.assertEqual(len(list(generator)), 2)
        generator = quick.spacy(chunks=5)
        self.assertEqual(len(list(generator)), 1)

    def test_single_document_annotation(self):
        rust = Document.from_string("rust is made by Mozilla")
        entities = [Entity("Rust", "PL"), Entity("Mozilla", "ORG")]
        rust.annotate(entities, case_sensitive=True)
        self.assertEqual(len(rust.label), 1)
        rust.annotate(entities, case_sensitive=False)
        self.assertEqual(len(rust.label), 2)
        self.assertEqual(rust.label[0][2], "ORG")
        self.assertEqual(rust.label[1][2], "PL")

    def test_character_level_slicing(self):
        entity = Entity("Python", "PL")
        document = Document("Indizes auf Zeichenebene anstelle von Indizes auf Byteebene, um Python-Slicing zu unterstützen")
        document.annotate([entity], case_sensitive=False)
        print(document.label[0][0], document.label[0][1])
        label = document.text[document.label[0][0]:document.label[0][1]]
        self.assertEqual(len(document.label), 1)
        self.assertEqual(label, "Python")


if __name__ == "__main__":
    unittest.main()
