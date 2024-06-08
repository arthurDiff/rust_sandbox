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