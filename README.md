# Small Performance Study in Rust

There are usually multiple ways to accomplish the same task and there are various criteria for selecting the best way to accomplish that task. In Rust programming language some repeatable tasks can be done either in traditional way of using `for` type of loop or in Rust idiomatic way through use of iterator chains. The current study investigates if the idiomatic way carries with it improved performance.

For the comparison the following four computational tasks were selected:
1. Counting the number of items in a list satisfying a criterion
2. Formatting a list of items into a String
3. Constructing a new list out of modified values of another list
4. Inserting entries into various map implementations: `HashMap`, `IndexMap`, and `BTreeMap`

## Approach 

Cargo `bench` framework and `criterion` crate were selected to measure computation performance. The conclusions were made based on the runs using MacBook Pro with Sequoia MacOS after making multiple runs like this:
```
cargo bench --bench <bench_name>
```
where `<bench_name>` is one of the benches defined in `Cargo.toml` corresponding to a computation under the investigation.

## Results

### Count Items (`count_cmp`)

Comparison of counting list elements using `for` loop vs iterator chain approach. The first approach keeps a counter which is incremented if the criteria is met for the item inside the `for` loop over the list. The second uses `.filter()` iterator method to discard items not meeting the criteria and then `.count()` method to count what left. The check if a number is odd is used as a criteria.

#### Conclusion

No significant difference is observed.

### List format (`listfmt_cmp`)

Comparison of constructing a `String` using `for` loop vs constructing a vector and use its `.join()` method vs using `.format_with()` iterator method provided by `itertools` crate. The first approach initializes a `String` and then uses its `.push()` method to append values and when necessary the `','` character as a separator. The second approach uses iterator's `.map()` method to convert items to a `String`, collects them into a `Vec<String>` using `.collect()` method and then uses `.join(',')` method to construct the result. The third approach uses iterator's `.format_with()` method to both convert items to a `String` and combine them with the separator.

#### Conclusion

The `for` loop approach (~68us) consistently outperforms `.collect::<Vec<String>>().join(',')` approach (~110us), being almost twice as fast. While `.format_with()` approach (~20us) consistently outperforms `for` loop approach, being more than three times as fast.

### List value map (`vecmap_cmp`)

Comparison of constructing a new list using `for` loop vs iterator chain approach. The first approach initializes vector and then uses its `.push()` method to append values to the list. The second approach iterates over the input list, modifies them using `.map()` method and then collects the result. For the first approach there's also a variation where the destination Vector is either preallocated using `Vec::with_capacity()` method with the number of the result items (1000) or allocated using default `Vec::new()`. To avoid the modification calculation to interfere with measurement simple square of the input value is used.

#### Conclusion

When the list is preallocated the construction (~700ns) outperforms the case where the default allocator is used (~1300ns), being almost twice as fast. While collecting mapped values (~300ns) outperforms pushing into preallocated list, being more than twice as fast.

### Map insert (`mapinsert_cmp`)

Comparison of inserting entries to into a map in a `for` loop vs using map's `::from_iter()` method. The comparison is done with 3 different type of maps: `HashMap`, `IndexMap`, and `BTreeMap` using various number of elements: 1, 5, 25, 125, 1000. For `HashMap` and `IndexMap` also compare preallocation of the map with the number of result entries (`BTreeMap` does not have preallocation)

#### Conclusion

When preallocation is used with `HashMap` or `IndexMap` the performance gain is better for bigger number of elements. For 1 element the difference is unnoticeable, for 5 ~10%, for 1000 ~30%. The performance of `::from_iter()` method is comparable to the insert with the map being preallocated which for `BTreeMap` result in significant performance gain for 1000 elements - insert ~250us vs from_iter ~140us. The number of elements also effects comparative performance among the map implementations. With a small number of elements (1-5, numbers are for 5) `BTreeMap` (~600ns) slightly outperforms `HashMap` even when preallocation is used (~670ns) and `HashMap` slightly outperforms `IndexMap` (~780ns). With a bigger number of elements (125 and up, numbers are for 1000, with preallocation) the order is reversed: `IndexMap` (~125us) slightly outperforms `HashMap` (~130us) which outperforms `BTreeMap` (~250us)