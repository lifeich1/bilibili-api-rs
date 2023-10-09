#!/usr/bin/env -S make -f

MAJOR_VER = 0
MINOR_VER = 3
PATCH_VER = 3

vern = $(MAJOR_VER).$(MINOR_VER).$(PATCH_VER)

all:
	perl -p -i -e \
		's/\Aversion = "\d+\.\d+\.\d+"/version = "$(vern)"/' \
		./bilibili-api-rs/Cargo.toml
	perl -p -i -e \
		's/\ALatest released: \d+\.\d+\.\d+/Latest released: $(vern)/' \
		./README.md
	git commit -a -m ":bookmark: bump to v$(vern)" -e
	git tag -a -s -m "Auto tag by bump.mak" v$(vern)
	gh release create v$(vern) --generate-notes
