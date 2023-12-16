.PHONY: default rectangles asset-benchmark bevymark comfymark comfymark-tracy

ENV_VARS=RUST_LOG=info,wgpu=warn,symphonia=warn,naga=warn RUST_BACKTRACE=1

# default: rectangles
# default: bevymark
default: comfymark
# default: comfymark-tracy
# default: comfymark-tracy
# default: asset-benchmark

rectangles:
	cargo run --bin comfy-rectangles --release

bevymark:
	# cargo run --bin bevymark --release
	cargo run --bin bevymark --profile stress-test

comfymark:
	# $(ENV_VARS) cargo run --bin comfymark --profile stress-test
	$(ENV_VARS) cargo run --bin comfymark

comfymark-tracy:
	$(ENV_VARS) cargo run --bin comfymark --features comfy/tracy

asset-benchmark:
	./generate-assets.sh
	$(ENV_VARS) cargo run --bin comfy-asset-benchmark --features comfy/tracy
