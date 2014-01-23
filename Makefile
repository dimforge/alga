build:
	rustpkg build math

test: build
	rustpkg test math

install: build
	rustpkg install math

clean:
	rustpkg clean
	rm -Rf ./build
