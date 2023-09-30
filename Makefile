.PHONY: default rectangles bevymark comfymark

# default: rectangles
default: bevymark

rectangles:
	cargo run --bin comfy-rectangles --release

bevymark:
	cargo run --bin bevymark --release
