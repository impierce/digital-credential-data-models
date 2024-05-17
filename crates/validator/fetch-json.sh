#!/usr/bin/env bash

mkdir "$(dirname $0)/elm-requests"
cd "$(dirname $0)/elm-requests"

wget -O bengales-highschool-diploma.json https://code.europa.eu/ebsi/json-schema/-/raw/main/schemas/vcdm2.0/edc/examples/Bengales_highSchoolDiploma.json?ref_type=heads
wget -O digicomp-generic.json https://code.europa.eu/ebsi/json-schema/-/raw/main/schemas/vcdm2.0/edc/examples/DigiComp%20Generic.json?ref_type=heads
wget -O diploma-rntuo-credential.json https://code.europa.eu/ebsi/json-schema/-/raw/main/schemas/vcdm2.0/edc/examples/Diploma%20rntuo%20credential.json?ref_type=heads
wget -O francisco-cruz.json https://code.europa.eu/ebsi/json-schema/-/raw/main/schemas/vcdm2.0/edc/examples/Francisco%20Cruz%20Argudo_Certificate%20of%20Completion.json?ref_type=heads
wget -O microcredential.json https://code.europa.eu/ebsi/json-schema/-/raw/main/schemas/vcdm2.0/edc/examples/Microcredential%20-%20Data%20and%20software%20business.json?ref_type=heads
wget -O transcript-of-records.json https://code.europa.eu/ebsi/json-schema/-/raw/main/schemas/vcdm2.0/edc/examples/Transcript%20of%20records%20-%20generic.json?ref_type=heads
