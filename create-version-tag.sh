#!/bin/bash
version=$(sed -n -E 's/version\s*=\s*\"([^\"]+)\"/\1/p' Cargo.toml | head -n 1)
branch=$(git branch | grep \* | cut -d ' ' -f2)
git tag -a "$version" -m "website version $version" -s && git push origin "$branch" --follow-tags