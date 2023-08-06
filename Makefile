install:build
	cp ./target/release/mds /bin/mds

build:
	cargo build --release

uninstall:
	rm -f /bin/mds

clean:
	rm -rf target