

BINARY=01-hello

RUSTC=rustc
RUST_OPTIONS=

all: 01-hello

%:%.rs
	$(RUSTC) $(RUST_OPTIONS) -o $@ $<


clean:
	rm -rf $(BINARY)
