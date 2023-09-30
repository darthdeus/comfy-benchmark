.PHONY: default rectangles bevymark comfymark

# default: rectangles
# default: bevymark
default: comfymark

rectangles:
	cargo run --bin comfy-rectangles --release

bevymark:
	# cargo run --bin bevymark --release
	cargo run --bin bevymark --profile stress-test

comfymark:
	# cargo run --bin bevymark --release
	cargo run --bin comfymark --profile stress-test
