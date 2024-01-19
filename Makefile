OUT_DIR=./src-tauri/target/release/bundle

all: build organize

build:
	pnpm tauri build

organize: 
	for dir in $(OUT_DIR)/*; \
	do \
		ls $$dir -p \
			| grep -v / \
			| grep -oE "[0-9]+\.[0-9]+\.[0-9]+" \
			| uniq \
			| while read -r line; \
				do \
					mkdir -p "$$dir/$$line"; \
					find $$dir -name "*$$line*" -type f \
					| xargs -I % mv % "$$dir/$$line"; \
				done; \
	done;
