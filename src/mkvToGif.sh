#!/bin/bash
scale="400"
fps="10"
file=$1
path=$2

# convert mkv to frames
ffmpeg -sseof -10 -t 10 -i ${file} -vf scale=${scale}:-1:flags=lanczos,fps=${fps} ${path}ffout%03d.png

# convert frames to gif
convert -loop 0 ${path}ffout*.png ${path}output.gif

echo "gif created at ${path}output.gif!"

# cleanup
rm ${path}*.png
rm ${file}

echo "all done!"
