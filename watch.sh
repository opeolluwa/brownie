#!/bin/bash/
echo "Watching scss files"
sass --watch public/stylesheets/index.scss:public/stylesheets/index.css
echo "Watching js files"
tsc --watch 
