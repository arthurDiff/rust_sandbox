minigrep-test:
	cargo test -p minigrep
minigrep_proj:
    IGNORE_CASE=1 cargo run -p minigrep -- genghis genghis-khan.txt
blockchain:
	cargo run -p blockchain_p