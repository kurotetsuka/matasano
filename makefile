#globals
default: build
freshen: clean build
clean:
	rm -rf bin/*

#vars


#includes
include lists.mk

#compilation definitions
$(bin_dirs):
	mkdir -p $@
$(binaries): bin/% : src/%.rs $(bin_dirs)
	rustc -g $< -o $@

#commands
build: $(binaries)

#tests
test: test-set1-one

test-pg: bin/test
	bin/test

test-set1-one: bin/set_one/one
	bin/set_one/one
