# just manual: https://github.com/casey/just#readme

run:
	rustc +stage1 -Zself-profile -Zself-profile-events=default,args src/main.rs
	crox $(ls -t *profdata | head -1)
	cat chrome_profiler.json | jq -r '.[] | select(.name == "evaluate_predicate_recursively") | .args.arg0' | sed -E 's/depth=[0-9]+/depth=?/' | sort | uniq -c | sort -n