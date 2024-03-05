BIN_DIR			:= $(shell pwd)/target/release
TARGET			:= $(BIN_DIR)/better-tree
SHRC_CFG		:= $(shell find $(HOME) -maxdepth 1 -type f -name ".*shrc")
ALIAS_CMD		:= alias btree="$(BIN_DIR)/better-tree"
RUST_CRGO		:= $(shell command -v cargo 2> /dev/null)


# Text color variables
BLACK			:= $(shell echo "\033[30m")
RED				:= $(shell echo "\033[31m")
GREEN			:= $(shell echo "\033[32m")
YELLOW			:= $(shell echo "\033[33m")
BLUE			:= $(shell echo "\033[34m")
MAGENTA			:= $(shell echo "\033[35m")
CYAN			:= $(shell echo "\033[36m")
LIGHT_GRAY		:= $(shell echo "\033[37m")
DARK_GRAY		:= $(shell echo "\033[90m")
LIGHT_RED		:= $(shell echo "\033[91m")
LIGHT_GREEN		:= $(shell echo "\033[92m")
LIGHT_YELLOW	:= $(shell echo "\033[93m")
LIGHT_BLUE		:= $(shell echo "\033[94m")
LIGHT_MAGENTA	:= $(shell echo "\033[95m")
LIGHT_CYAN		:= $(shell echo "\033[96m")
WHITE			:= $(shell echo "\033[97m")

# Background color variables
BG_BLACK		:= $(shell echo "\033[40m")
BG_RED			:= $(shell echo "\033[41m")
BG_GREEN		:= $(shell echo "\033[42m")
BG_YELLOW		:= $(shell echo "\033[43m")
BG_BLUE			:= $(shell echo "\033[44m")
BG_MAGENTA		:= $(shell echo "\033[45m")
BG_CYAN			:= $(shell echo "\033[46m")
BG_LIGHT_GRAY	:= $(shell echo "\033[47m")
BG_DARK_GRAY	:= $(shell echo "\033[100m")
BG_LIGHT_RED	:= $(shell echo "\033[101m")
BG_LIGHT_GREEN	:= $(shell echo "\033[102m")
BG_LIGHT_YELLOW	:= $(shell echo "\033[103m")
BG_LIGHT_BLUE	:= $(shell echo "\033[104m")
BG_LIGHT_MAGENTA:= $(shell echo "\033[105m")
BG_LIGHT_CYAN	:= $(shell echo "\033[106m")
BG_WHITE		:= $(shell echo "\033[107m")

# Reset text color and background
RESET			:= $(shell echo "\033[0m")



all: install

$(TARGET):
	cargo build --release

install: $(TARGET)
	@echo "\n"
	@if ! grep -q 'alias btree=' $(SHRC_CFG); then \
		echo "\n\n# better-tree alias" >> $(SHRC_CFG); \
		echo "$(ALIAS_CMD)" >> $(SHRC_CFG); \
		echo "$(GREEN)Alias added successfully. Please run \
'$(YELLOW)source $(SHRC_CFG)$(GREEN)' to apply.\
\n$(GREEN)Use \`$(YELLOW)btree -h$(GREEN)\` for help.$(RESET)"; \
	else \
		echo "$(RED)Alias already exists in $(YELLOW)$(SHRC_CFG)$(RESET)"; \
	fi

uninstall:
	@if grep -q '# better-tree alias' $(SHRC_CFG); then \
		sed -i '/# better-tree alias/{N;d;}' $(SHRC_CFG); \
		echo "$(GREEN)Alias removed successfully. Please run \
'$(YELLOW)source $(SHRC_CFG)$(GREEN)' to reapply changes.$(RESET)"; \
	else \
		echo "$(RED)Alias not found in $(YELLOW)$(SHRC_CFG)$(RED). Nothing to remove.$(RESET)"; \
	fi
	cargo clean

remove: uninstall
delete: uninstall

re: uninstall install remove delete

.PHONY: install uninstall re