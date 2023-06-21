#!/bin/bash

# Redis configuration
REDIS_HOST="172.17.0.2"
REDIS_PORT="6379"
REDIS_PASSWORD=""  # Set your Redis password if applicable

# Function to save company information to Redis
save_company() {
    # Generate a random number between 0 and 2000
    local random_number=$((RANDOM % 2001))

    # Subtract 1000 to shift the range from 0-2000 to -1000-1000
    local shifted_number=$((random_number - 1000))

    # Convert the shifted number to a floating-point value by dividing it by 1000
    local random_float=$(awk "BEGIN {printf \"%.3f\", $shifted_number/1000}")

    local id="$1"
    local ticker="$2"
    local sector="$3"
    local industry="$4"
    local exchange="$5"
    local country="$6"
    local adj=("${@:7}")  # Remaining arguments are adjectives
    local growth="$random_float"

    # Prepare Redis command
    local redis_command="HSET c:$id ticker \"$ticker\" sector \"$sector\" industry \"$industry\" \
        exchange \"$exchange\" country \"$country\" growth \"$growth\" adj \"${adj[*]}\""

    # Execute Redis command
    redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" eval "$redis_command"

    echo "Saved company: $ticker"
}

# Create and save companies

# Company 1
save_company 1 "AAPL" "Technology" "Healthcare" "NASDAQ" "USA" "growth" "divs"

# Company 2
save_company 2 "GOOGL" "Technology" "Telecommunications" "NASDAQ" "USA" "value" "growth" "divs"

# Company 3
save_company 3 "MSFT" "Technology" "Finance" "NASDAQ" "USA" "growth" "zombie"

# Company 4
save_company 4 "AMZN" "Technology" "Retail" "NASDAQ" "USA" "value" "growth" "divs" "zombie"

# Company 5
save_company 5 "FB" "Technology" "Media" "NASDAQ" "USA" "growth" "divs" "zombie"

# Company 6
save_company 6 "TSLA" "Automotive" "Unknown" "NASDAQ" "USA" "growth"

# Company 7
save_company 7 "JPM" "Financial Services" "Unknown" "NYSE" "USA" "value" "zombie"

# Company 8
save_company 8 "BAC" "Financial Services" "Unknown" "NYSE" "USA" "growth" "zombie"

# Company 9
save_company 9 "WMT" "Consumer Cyclical" "Unknown" "NYSE" "USA" "value" "divs"

# Company 10
save_company 10 "GE" "Unknown" "Unknown" "NYSE" "USA" "growth" "zombie"

echo "Companies saved to Redis."
