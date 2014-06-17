all: src
	mkdir -p target
	rustc --out-dir target src/lib.rs

test: target/test
	./target/test

target/test: src
	mkdir -p target
	rustc -o target/test src/lib.rs --test

clean:
	rm -rf target
