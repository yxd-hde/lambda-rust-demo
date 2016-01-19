COMPONENT=$(shell basename $$(pwd))

.PHONY: clean dist zip unit-test deps

default: clean deps dist zip;

deps:
	test -d lib || mkdir lib
	pip install -r requirements.txt -t lib/

dist: deps
	test -d dist || mkdir dist
	cd handler && cargo build --release
	cp handler/target/release/libhandler.so .
	cp *.py *.so dist/
	cp -r lib/* dist/

unit-test: ;

zip: dist
	cd dist && zip -q -r ../$(COMPONENT).zip .

clean:
	cd handler && cargo clean
	rm -rf *.so $(COMPONENT).zip lib/* dist/*
