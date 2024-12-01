#!/bin/env bash
day=$1
if [ -f "src/day$day.rs" ];
then
echo "File for day $day already exists"
else
echo "Adding day $day"
# Replace day number in template and create new day file
sed "s|\DAY_NUMBER|${day}|g" "templates/day_template.rs" > "src/day$day.rs"

# Create Input files for easy filling
touch "inputs/day${day}_test.txt"
touch "inputs/day${day}.txt"

# Add benchmarks
sed "s|\DAY_NUMBER|${day}|g" "templates/bench_template.rs" > "benches/day${day}.rs"
sed  -i "/[[:blank:]]*\/\/ Add days here/i mod day${day};" benches/aoc.rs


bench_day_code="criterion_group!(day${day}, day${day}::bench_day${day}_p1, day${day}::bench_day${day}_p2);"
sed  -i "/[[:blank:]]*\/\/ Add day group here/i ${bench_day_code}" benches/aoc.rs

# Add group to main
sed -i "\$s/)/, day${day})/" benches/aoc.rs

# Add match to main.rs
sed  -i "/[[:blank:]]*\/\/ Day invocations/i ${day} => day${day}::run()?," src/bin/main.rs
echo "pub mod day${day};" >> src/lib.rs

# Format all touched files
rustfmt "src/day$day.rs"
rustfmt src/bin/main.rs
rustfmt src/lib.rs
rustfmt benches/aoc.rs
rustfmt "benches/day$day.rs"

fi