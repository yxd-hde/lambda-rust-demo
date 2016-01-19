COMPONENT=$(shell basename $$(pwd))

.PHONY: clean dist zip unit-test deps

default: clean deps dist zip;

deps:
	test -d lib || mkdir lib

# Install python's dependencies.
	pip install -r requirements.txt -t lib/

dist: deps
	test -d dist || mkdir dist

# Build rust library
	cd handler && cargo build --release

# Move the library to CWD
	cp handler/target/release/libhandler.so .

# Copy the source and library to dist
	cp *.py *.so dist/
	cp -r lib/* dist/

unit-test: ;

zip: dist
# Package the dist folder into a zip to deploy
	cd dist && zip -q -r ../$(COMPONENT).zip .

clean:
	cd handler && cargo clean
	rm -rf *.so $(COMPONENT).zip lib/* dist/*
