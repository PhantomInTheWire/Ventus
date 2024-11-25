#!/bin/bash

# Output filename
output_file="git_repo_stats.md"

# Clear or create the output file
echo "" > "$output_file"

# Header
echo "# Git Repository Stats" >> "$output_file"
echo "" >> "$output_file"

# Total number of commits
total_commits=$(git rev-list --count HEAD)
echo "## Total Commits" >> "$output_file"
echo "- **Total number of commits:** $total_commits" >> "$output_file"
echo "" >> "$output_file"

# Number of commits by each author and percentage
echo "## Commits by Author" >> "$output_file"
echo "| Author | Number of Commits | Percentage of Commits |" >> "$output_file"
echo "|--------|-------------------:|-----------------------:|" >> "$output_file"

git shortlog -sne | while read -r line; do
  commits=$(echo "$line" | awk '{print $1}')
  author=$(echo "$line" | cut -f2- -d' ')
  percentage=$(awk "BEGIN {printf \"%.2f\", ($commits / $total_commits) * 100}")
  echo "| $author | $commits | $percentage% |" >> "$output_file"
done
echo "" >> "$output_file"

# Current lines of code by each author
echo "## Lines of Code by Author" >> "$output_file"
echo "| Author | Lines of Code | Percentage of Code |" >> "$output_file"
echo "|--------|--------------:|-------------------:|" >> "$output_file"

git ls-files | grep -E '\.(c|cpp|h|py|js|ts|java|rb|go|rs|php|swift|sh)$' | xargs -n1 -I{} git blame --line-porcelain {} |\
awk '/^author /{a[$2]++} END {for (i in a) print a[i], i}' | sort -nr | while read -r line; do
  loc=$(echo "$line" | awk '{print $1}')
  author=$(echo "$line" | cut -f2- -d' ')
  total_loc=$(git ls-files | grep -E '\.(c|cpp|h|py|js|ts|java|rb|go|rs|php|swift|sh)$' | xargs cat | wc -l)
  percentage=$(awk "BEGIN {printf \"%.2f\", ($loc / $total_loc) * 100}")
  echo "| $author | $loc | $percentage% |" >> "$output_file"
done
echo "" >> "$output_file"

# Top 10 churned files by number of changes
echo "## Top 10 Most Changed Files" >> "$output_file"
echo "| File | Number of Changes |" >> "$output_file"
echo "|------|-------------------:|" >> "$output_file"

git log --name-only --pretty=format: | sort | uniq -c | sort -nr | head -n 10 | while read -r line; do
  changes=$(echo "$line" | awk '{print $1}')
  file=$(echo "$line" | cut -f2- -d' ')
  echo "| $file | $changes |" >> "$output_file"
done
echo "" >> "$output_file"

# File type summary
echo "## File Type Summary" >> "$output_file"
echo "| File Type | Number of Files |" >> "$output_file"
echo "|-----------|----------------:|" >> "$output_file"

git ls-files | sed 's/.*\.//' | sort | uniq -c | sort -nr | while read -r line; do
  count=$(echo "$line" | awk '{print $1}')
  extension=$(echo "$line" | awk '{print $2}')
  echo "| .$extension | $count |" >> "$output_file"
done
echo "" >> "$output_file"

echo "Git repository stats have been written to $output_file"

