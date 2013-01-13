run-tests: tests
	./tests

tests: webencoding.rc webencoding.rs indexes.rs
	rustc --test $< -o $@

.PHONY: run-tests
