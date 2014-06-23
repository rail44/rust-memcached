all: src
	mkdir -p target
	rustc --out-dir target src/memcached.rs

test: target/test
	./target/test

target/test: src
	mkdir -p target
	rustc -o target/test src/memcached.rs --test

clean:
	rm -rf target
