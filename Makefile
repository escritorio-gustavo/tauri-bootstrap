OUT_DIR=/src-tauri/target/release/bundle

all: printy
printy: 
	pnpm tauri build
	for dir in "msi" "nsis"; \
	do \
		ls $(OUT_DIR)/$$dir p \
			| grep v / \
			| grep oE "[0-9]+\.[0-9]+\.[0-9]+" \
			| uniq \
			| while read r line; \
				do \
					mkdir p "$(OUT_DIR)/$$dir/$$line"; \
					find $(OUT_DIR)/$$dir name "*$$line*" -type f \
					| xargs I % mv % "$(OUT_DIR)/$$dir/$$line"; \
				done; \
	done;
