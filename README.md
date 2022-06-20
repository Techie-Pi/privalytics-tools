# Privalytics Tools
> Tools to transform and analyze Privalytics data

## Usage
```
privalytics --input-folder <INPUT_FOLDER> --output-folder <OUTPUT_FOLDER> <TRANSFORMATION>
```

- ``<INPUT_FOLDER>`` Is the folder that contains the Privalytics data, e.g. ``./test_data/``
- ``<OUTPUT_FOLDER>`` Is the folder where transformed data will be saved, e.g. ``./output_test_data/``
- ``<TRANSFORMATION>`` Is the desired transformation to perform:
  - ``to-csv`` Converts the data to a ``.csv`` file
