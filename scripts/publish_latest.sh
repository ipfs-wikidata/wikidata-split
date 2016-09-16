#! /bin/bash
set -euf -o pipefail

version="20160912"

echo "$(date): Starting logging" >> publish_latest.log

echo "$(date): Starting download of version ${version}" >> publish_latest.log
wget --no-clobber -c --output-file download.log --output-document latest-all.json.gz https://dumps.wikimedia.org/wikidatawiki/entities/${version}/wikidata-${version}-all.json.gz
echo "$(date): Finished download of version ${version}" >> publish_latest.log

echo "$(date): Starting unzip" >> publish_latest.log
gunzip latest-all.json.gz
echo "$(date): Finished unzip" >> publish_latest.log

echo "$(date): Starting split" >> publish_latest.log
../wikidata-split > split.log
echo "$(date): Finished split" >> publish_latest.log

mv split2 split

echo "$(date): Starting publish to IPFS of version ${version}" >> publish_latest.log
ipfs add -r split > ipfs_add.log
echo "$(date): Finished publish to IPFS of version ${version}" >> publish_latest.log
