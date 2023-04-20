#!/bin/bash
for f in ./*.html; do
    echo Converting $f
    wkhtmltopdf --page-height 8in --page-width 5.375in "$f" pdf/$f.pdf
done
