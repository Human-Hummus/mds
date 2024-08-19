install:build
	cp ./target/release/mds /bin/mds
qinstall:qbuild
	cp ./target/debug/mds /bin/mds

build:
	cargo build --release

qbuild:
	cargo build

uninstall:
	rm -f /bin/mds

clean:
	rm -rf target

