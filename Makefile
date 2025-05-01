minigrep_test:
	cargo test -p minigrep
minigrep_proj:
    IGNORE_CASE=1 cargo run -p minigrep -- genghis genghis-khan.txt
blockchain:
	cargo run -p blockchain_p
blockchain_test:
	cargo test -p blockchain_p
websocket:
	cargo run -p webbest_socket
sha:
	cargo run -p sha1
b_math:
	cargo run -p brain_math
num_regn:
	cargo run -p number_regn
pg:
	cargo run -p playground
bt:
	cargo run -p bit_tor