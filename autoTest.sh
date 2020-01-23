inotifywait -r src -m -e attrib |
while read dir ev; do
	echo "Change detected, testing... ($ev)"
	cargo test
done
