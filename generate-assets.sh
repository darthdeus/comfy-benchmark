#!/bin/bash
set -euxo pipefail

dir="comfy-asset-benchmark/assets"

for i in $(seq 1 100); do
  cp "$dir/_base-comfy.png" "$dir/comfy-$i.png"
  cp "$dir/_base-music.wav" "$dir/music-$i.wav"
done
