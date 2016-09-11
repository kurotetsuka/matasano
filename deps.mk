# dependencies

$(binaries): $(libraries)

bin/libmatasano.rlib: $(shell find src/matasano -name *.rs)
