#!/bin/bash

# Read the content of 'embed-widget.js'
file_content=$(cat assets/og-embed-widget.js)

# Send a POST request to the JavaScript minifier API with the content as a string
response=$(curl -X POST -s --data-urlencode "input=$file_content" https://www.toptal.com/developers/javascript-minifier/api/raw)

# Save the response to a new file
echo "$response" > assets/embed-widget.js
