%: %.rs
	rustc -O $<
all: $(basename $(wildcard *.rs))
