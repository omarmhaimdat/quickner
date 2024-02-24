import time
import csv

from quickner import Entity, Document, Quickner


def get_entities():
    with open("data/entities.csv", "r") as f:
        reader = csv.reader(f)
        entities = [Entity(*row) for row in reader]
    return entities


def get_documents():
    with open("data/texts.csv", "r") as f:
        reader = csv.reader(f)
        documents = [Document(row[0]) for row in reader]
    return documents


def main():
    start = time.perf_counter()
    documents = get_documents()
    entities = get_entities()
    quick = Quickner(documents=documents, entities=entities)
    quick.process()
    end = time.perf_counter()
    quick.to_jsonl("data/output.jsonl")
    print(quick)
    docs = quick.find_documents_by_entity("Twitter")
    print(f"Time elapsed: {end - start} seconds")


if __name__ == "__main__":
    main()
