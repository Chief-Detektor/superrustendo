
#!/usr/bin/env bash


cat `ls target/cov/superrustendo-* | head -n3 | tail -n1 | sed 's/.\{1\}$//'`/coverage.json | sed -n -r 's/\s{2}"(percent_covered)": "([0-9]*\.[0-9]*)",/\1: \2/p'
