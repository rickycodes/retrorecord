#!/bin/bash
file=$1
output=${file/.mkv/.mp4}

# -sseof -20 -t 20
ffmpeg -y -t 10 -sseof -12 -i "${file}" -an -vcodec copy -strict -2 "${output}"

echo "file created: ${output}"
