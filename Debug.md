# Normal

running 4 tests
test atof_real_f32_lexical ... bench:   9,522,969 ns/iter (+/- 568,037)
test atof_real_f32_parse   ... bench:   7,705,735 ns/iter (+/- 427,745)
test atof_real_f64_lexical ... bench:     579,470 ns/iter (+/- 53,207)
test atof_real_f64_parse   ... bench:     458,225 ns/iter (+/- 34,298)

# Removing Parse Mantissa

running 4 tests
test atof_real_f32_lexical ... bench:   3,910,359 ns/iter (+/- 179,289)
test atof_real_f32_parse   ... bench:   7,696,323 ns/iter (+/- 470,487)
test atof_real_f64_lexical ... bench:     232,261 ns/iter (+/- 15,690)
test atof_real_f64_parse   ... bench:     458,034 ns/iter (+/- 27,882)

# Removing Parse Mantissa + Previous (Does nothing).

running 4 tests
test atof_real_f32_lexical ... bench:   3,894,768 ns/iter (+/- 274,067)
test atof_real_f32_parse   ... bench:   7,595,580 ns/iter (+/- 503,974)
test atof_real_f64_lexical ... bench:     230,111 ns/iter (+/- 21,987)
test atof_real_f64_parse   ... bench:     457,584 ns/iter (+/- 87,386)

# Remove Filter Special + Previous (Does little).

running 4 tests
test atof_real_f32_lexical ... bench:   3,693,884 ns/iter (+/- 154,190)
test atof_real_f32_parse   ... bench:   7,542,517 ns/iter (+/- 324,042)
test atof_real_f64_lexical ... bench:     238,669 ns/iter (+/- 47,890)
test atof_real_f64_parse   ... bench:     450,920 ns/iter (+/- 43,167)

# Remove Filter Sign + Previous (Does little).

running 4 tests
test atof_real_f32_lexical ... bench:   3,531,515 ns/iter (+/- 146,990)
test atof_real_f32_parse   ... bench:   7,506,981 ns/iter (+/- 419,084)
test atof_real_f64_lexical ... bench:     211,185 ns/iter (+/- 9,688)
test atof_real_f64_parse   ... bench:     439,824 ns/iter (+/- 44,580)

# Remove Fast Path + Previous (Does little).

running 4 tests
test atof_real_f32_lexical ... bench:   3,567,233 ns/iter (+/- 194,263)
test atof_real_f32_parse   ... bench:   7,482,216 ns/iter (+/- 320,260)
test atof_real_f64_lexical ... bench:     211,949 ns/iter (+/- 11,644)
test atof_real_f64_parse   ... bench:     438,263 ns/iter (+/- 46,988)

# Remove all float processing (baseline)

running 4 tests
test atof_real_f32_lexical ... bench:   3,609,686 ns/iter (+/- 212,827)
test atof_real_f32_parse   ... bench:   7,596,774 ns/iter (+/- 797,073)
test atof_real_f64_lexical ... bench:     220,805 ns/iter (+/- 37,128)
test atof_real_f64_parse   ... bench:     439,415 ns/iter (+/- 57,691)

# Conclusion

Almost all of the overhead is in parse_mantissa.
