#!/bin/bash

while read dir; do
	echo "Target: ${dir}"
	(
		cd "${dir}"
		cargo clean
	)
done < <(find . -type d -name target -exec dirname {} \;)

exit 0
