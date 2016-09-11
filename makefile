# globals
default: build
freshen: clean build
clean:
	rm -rf bin/*

# vars
options =
#options = -A dead_code
#options = -A dead_code -A unused_variables

# includes
include lists.mk
include deps.mk

# compilation definitions
$(dirs):
	mkdir -p $@

$(libraries): bin/lib%.rlib: src/%/lib.rs
	rustc --out-dir bin/ $<

$(binaries): bin/% : src/bin/%.rs
	rustc $(options) \
		-L crate=bin/ \
		-g $< -o $@

# commands
dirs: $(dirs)
build: dirs $(binaries)

# tests
test: dirs test-set-one

test-asdf: bin/asdf
	$<

test-s1c1: bin/set1/c1
	$<
test-s1c2: bin/set1/c2
	$<
test-s1c3: bin/set1/c3
	$<
test-s1c4: bin/set1/c4
	$<
test-s1c5: bin/set1/c5
	$<
test-s1c6: bin/set1/c6
	$<
test-s1c7: bin/set1/c7
	$<
test-s1c8: bin/set1/c8
	$<
