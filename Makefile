.PHONY: default rectangles bevymark comfymark comfymark-tracy

# default: rectangles
# default: bevymark
# default: comfymark
default: comfymark-tracy

rectangles:
	cargo run --bin comfy-rectangles --release

bevymark:
	# cargo run --bin bevymark --release
	cargo run --bin bevymark --profile stress-test

comfymark:
	cargo run --bin comfymark --profile stress-test

comfymark-tracy:
	cargo run --bin comfymark --profile stress-test --features comfy/tracy
