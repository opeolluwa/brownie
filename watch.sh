#!/bin/bash/
echo "Watching scss files"
scss --watch public/index.scss:public/index.scss
echo "Watching js files"
tsc --watch 
