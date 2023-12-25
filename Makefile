TARGET			= better-tree
ALIAS_1			= btree

all: install

$(TARGET):
	cargo build --release

install: $(TARGET)
	sudo cp target/release/$(TARGET) /usr/local/bin/$(TARGET)
	sudo chmod +x /usr/local/bin/$(TARGET)
	sudo ln -s /usr/local/bin/$(TARGET) /usr/local/bin/$(ALIAS_1)

uninstall:
	sudo rm -f /usr/local/bin/$(TARGET)
	sudo rm -f /usr/local/bin/$(ALIAS_1)

.PHONY: install uninstall