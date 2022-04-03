#!/bin/bash
# pass arg of video to dump frames from video into current dir
ffmpeg -i $1 frames/f%04d.jpg -hide_banner