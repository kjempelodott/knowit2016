%: %.rs
	rustc -O $<
	strip $@
all: $(basename $(wildcard *.rs))
