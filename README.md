# OpenBadges
Rust library for the Open Badges data model
https://www.imsglobal.org/spec/ob/v3p0

Open Badges Specification
Candidate Final Public
Spec Version:       3.0
Document Version:   1.0.10
Date Issued:        September 22, 2023

# Summary
This library contains all structs and enums needed to (De)Serialize OpenBadges from and into JSON format. These structs and enums are enhanced with implementations and builders for more convenient usage.

# Index

- `src`: This folder contains all the structs, enums and their implementations and builders.
- `tests/obv3_json_examples`: This folder contains all JSON format examples specified on the website (https://www.imsglobal.org/spec/ob/v3p0#examples-0) as JSON files.
- `tests/tests`: This folder contains the tests indexed per example JSON file.

# Deviation
There is one deviation to the Json Schema as specified in the following issue:
https://github.com/1EdTech/openbadges-specification/issues/553
Changes suggested in this issue are accepted, but have not yet been made.
This library has already adopted this change.

