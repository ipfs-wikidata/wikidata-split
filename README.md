# Wikidata-split

Small hacky tool to split up the weekly Wikidata json export into one entity per file and place them in sharded directories.

Expects a file `latest-all.json` as input and will output into a directory `split` into files and diretories like `split/Q12/144/12.json`
