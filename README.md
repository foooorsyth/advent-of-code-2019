# Advent of Code 2019 in Rust
https://adventofcode.com/2019

![Advent of Code](https://i.imgur.com/gRDcR6a.jpg)

![](https://github.com/aimkey/advent-of-code-2019/workflows/FmtBuildTest/badge.svg)
![](https://gitlab.com/aimkey/advent-of-code-2019/badges/master/pipeline.svg)

**Running:**

```cargo run --release -- <day>```

So, ```cargo run --release -- 7``` will execute Parts 1 and 2 from Day 7

**Testing:**

```cargo test --release```

**Debugging in VS Code:**

* Install C/C++, CodeLLDB, and Rust (rls) Extensions. 
* Set ```// Day to debug``` arg in .vscode/launch.json
* Press F5

**Constraints:**

Standard lib only. No external crates.

Day | Part 1 | Part 2 | Comments
--- | ------- | ------- | --- 
1 | :heavy_check_mark: | :heavy_check_mark: | 
2 | :heavy_check_mark: | :heavy_check_mark: | Brute forced Part 2 because of small range -- no inverse function
3 | :heavy_check_mark: | :heavy_check_mark: | Cached str rep of points for first wire, did lookups of points in second wire for intersections
4 | :heavy_check_mark: | :heavy_check_mark: | Decently optimized but could go further by restricting loop range by considering gap value and higher power digit values
5 | :heavy_check_mark: | :heavy_check_mark: | 
6 | :heavy_check_mark: | :heavy_check_mark: | I fought the Borrow Checker, and the Borrow Checker (almost) won
7 | :heavy_check_mark: | :heavy_check_mark: | 
8 | :heavy_check_mark: | :heavy_check_mark: | Was expecting "Be sure to drink your Ovaltine"
9 | :heavy_check_mark: | :heavy_check_mark: | 
10 | :heavy_check_mark: | :heavy_check_mark: | 
11 | :heavy_check_mark: | :heavy_check_mark: | 
12 | :heavy_check_mark: | :heavy_check_mark: | Thanks, Euclid
13 | :heavy_check_mark: | :heavy_check_mark: | Used an aimbot :)
14 | :heavy_check_mark: | :heavy_check_mark: | 
15 | :heavy_check_mark: | :heavy_check_mark: | 
16 | :heavy_check_mark: | :heavy_check_mark: | 
17 | :heavy_check_mark: | :heavy_check_mark: | Solve by hand: 2 minutes. Solve programmatically: 2 hours
18 | :heavy_check_mark: | :heavy_check_mark: | My brain hurts
19 | :heavy_check_mark: | :heavy_check_mark: | 
20 | | | 
21 | | | 
22 | | | 
23 | | | 
24 | | | 
25 | | | 
