target:=levelspec


build:
	cargo build --release

install:
	cp target/release/${target} ~/bin/.

.PHONY: all
all: build install