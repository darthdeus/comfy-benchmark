.PHONY: default rectangles asset-benchmark bevymark comfymark comfymark-tracy

ENV_VARS=RUST_LOG=info,wgpu=warn,symphonia=warn,naga=warn RUST_BACKTRACE=1 COMFY_VSYNC_OVERRIDE=0

# default: rectangles
# default: bevymark
default: comfymark-tracy
# default: comfymark-tracy
# default: asset-benchmark

rectangles:
	cargo run --bin comfy-rectangles --release

bevymark:
	# cargo run --bin bevymark --release
	cargo run --bin bevymark --profile stress-test

comfymark:
	cargo run --bin comfymark --profile stress-test

comfymark-tracy:
	cargo run --bin comfymark --profile stress-test --features comfy/tracy

asset-benchmark:
	./generate-assets.sh
	$(ENV_VARS) cargo run --bin comfy-asset-benchmark --features comfy/tracy
