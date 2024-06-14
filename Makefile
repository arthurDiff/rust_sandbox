intro_proj:
	cargo run -p intro
guess:
	cargo run -p guess_this
common: 
	cargo run -p common_concept
owner:
	cargo run -p ownership
struct:
	cargo run -p struct_proj
enum:
	cargo run -p enum_proj
module:
	cargo run -p module_sys
collection:
	cargo run -p collection_proj
generic:
	cargo run -p generic_proj
testing:
	cargo test -p test_proj -- --include-ignored
minigrep-test:
	cargo test -p minigrep
minigrep_proj:
    IGNORE_CASE=1 cargo run -p minigrep -- genghis genghis-khan.txt
closure_iter_proj:
	cargo run -p closure_iter
cargo_play_proj:
	cargo test -p cargo_play
smart_ptr_p:
	cargo run -p smart_ptr
fear_the_conc_p:
	cargo run -p fear_the_conr
oop:
	cargo run -p oop_proj
adv:
	cargo run -p adv_proj