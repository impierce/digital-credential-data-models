from rdflib import Graph
from pyshacl import validate
from os import path

g_elm = Graph()
g_elm.parse('EDC-generic-full.rdf')

q = """
PREFIX elm: <http://data.europa.eu/snb/model/elm/>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?entity WHERE {
        ?entity rdfs:subClassOf* elm:EuropeanDigitalCredential .
}
"""
x = g_elm.query(q)

print(list(x))

# g_elm.serialize(format="turtle", destination="shape.ttl")
g_elm.serialize(format="json-ld", destination="shape.json")


del g_elm

g_data = Graph()
g_data.parse("credential-sample.json", format="json-ld")
g_data.serialize(destination="data.ttl", format="turtle")

# Load the request
data = path.abspath('data.ttl')

# Load the SHACL shape
shape = path.abspath('shape.ttl')

# Validate the data against the shape
conforms, _, text = validate(
    data, shacl_graph=shape, inference='rdfs'
)

# Print validation results
if conforms:
    print("Data conforms to the SHACL shape.")
else:
    print("Data does not conform to the SHACL shape. Validation report:")
    print(text)
