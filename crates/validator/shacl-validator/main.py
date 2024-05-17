from rdflib import Graph
from pyshacl import validate
from os import path
import sys
import getopt
import argparse


def file_path(string):
    if path.isfile(string) and ".json" in string:
        return string
    else:
        raise argparse.ArgumentTypeError(
            f"Input file:{path} is not a valid JSON file")


# Add to main if shacl shape needs to be regenerated.
def create_shacl_shape():
    g_shape = Graph()
    g_shape.parse('EDC-generic-full.rdf')
    g_shape.serialize(format="turtle", destination="shape.ttl")

    del g_shape

def main(inputfile):
    g_data = Graph()
    g_data.parse(inputfile, format="json-ld")

    # Load the request
    data = g_data.serialize(format="turtle")

    # Load the SHACL shape
    shape = path.abspath('shape.ttl')

    # Validate the data against the shape
    conforms, _, text = validate(data, shacl_graph=shape, inference='rdfs')

    # Print validation results
    if conforms:
        print("Data conforms to the SHACL shape.")
    else:
        print("Data does not conform to the SHACL shape. Validation report:")
        print(text)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-i', '--input-file', type=file_path)
    args = parser.parse_args()
    if args.input_file is None:
        print("main.py -i <inputfile>")
        sys.exit(2)
    print('Input file is:', args.input_file)
    main(args.input_file)
