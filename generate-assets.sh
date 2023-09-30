#!/bin/bash
set -euxo pipefail

dir="comfy-asset-benchmark/assets"

if [ -f "$dir/comfy-50.png" ]; then
  exit 0
fi

for i in $(seq 1 50); do
  cp "$dir/_base-comfy.png" "$dir/comfy-$i.png"
  cp "$dir/_base-wall.jpg" "$dir/wall-$i.jpg"
  cp "$dir/_base-music.wav" "$dir/music-$i.wav"
done
