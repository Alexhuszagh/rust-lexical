# `parse_number_fxx` Test Data

This repository contains test data for `parse_number_fxx` implementations (for
`fxx` being `f16`, `f32` or `f64`), also known as `StringToDouble`, `strtod`,
`atof`, etc. These convert from an ASCII string to a 16-, 32- or 64-bit value
(IEEE 754 half-, single- or double-precision floating point).

Most of the `data/*.txt` files were derived by running
`script/extract-numbery-strings.go` on various repositories or zip files,
listed further below. Their contents look like:

    3C00 3F800000 3FF0000000000000 1
    3D00 3FA00000 3FF4000000000000 1.25
    3D9A 3FB33333 3FF6666666666666 1.4
    57B7 42F6E979 405EDD2F1A9FBE77 123.456
    622A 44454000 4088A80000000000 789
    7C00 7F800000 7FF0000000000000 123.456e789

For example, parsing `"1.4"` as a `float32` gives the bits `0x3FB33333`.

In this case, the final line's `float16`, `float32` and `float64` values are
all infinity. The largest finite `float{16,32,64}` values are approximately
`6.55e+4`, `3.40e+38` and `1.80e+308`.

For each line of these `data/*.txt` files, the `f16`, `f32` and `f64`
hexadecimal digits and the ASCII string subslices are:

- When column indexes start at 0: `[0..4]`, `[5..13]`, `[14..30]` and `[31..]`.
- When column indexes start at 1: `[1..5]`, `[6..14]`, `[15..31]` and `[32..]`.

The first half (the high 16 bits) of the `f32` hexadecimal digits are also
known as the `bfloat16` format.


## Data

In the `data` directory:

- `freetype-2-7.txt` was extracted from [Freetype](https://www.freetype.org/)
  2.7
- `google-double-conversion.txt` was extracted from
  [google/double-conversion](https://github.com/google/double-conversion)
- `google-wuffs.txt` was extracted from
  [google/wuffs](https://github.com/google/wuffs)
- `ibm-fpgen.txt` was extracted from IBM's
  [IEEE 754R test suite](https://www.research.ibm.com/haifa/projects/verification/fpgen/test_suite_download.shtml)
- `lemire-fast-double-parser.txt` was extracted from
  [lemire/fast\_double\_parser](https://github.com/lemire/fast_double_parser)
- `lemire-fast-float.txt` was extracted from
  [lemire/fast\_float](https://github.com/lemire/fast_float)
- `more-test-cases.txt` was extracted from this repository's manually curated
  collection of [more test cases](./more-test-cases)
- `tencent-rapidjson.txt` was extracted from
  [Tencent/rapidjson](https://github.com/Tencent/rapidjson)
- `ulfjack-ryu.txt` was extracted from
  [ulfjack/ryu](https://github.com/ulfjack/ryu)


### remyoudompheng/fptest

The `data/remyoudompheng-fptest-?.txt` files were created by running `go test
-test.run=TestTortureAtof64` in the
[remyoudompheng/fptest](https://github.com/remyoudompheng/fptest) repository
(with the following patch), running the resultant `TestTortureAtof64.txt` file
through `script/extract-numbery-strings.go` and then using `sed` to split what
would be a 189 MiB file into multiple (million line) files:

```diff
diff --git a/torture_test.go b/torture_test.go
index 87ba7e7..59887ff 100644
--- a/torture_test.go
+++ b/torture_test.go
@@ -1,8 +1,11 @@
 package fptest

 import (
+       "bufio"
        "bytes"
+       "fmt"
        "math"
+       "os"
        "strconv"
        "testing"

@@ -124,6 +127,11 @@ func TestTortureShortest32(t *testing.T) {
 }

 func TestTortureAtof64(t *testing.T) {
+       tmpFile, _ := os.Create("/tmp/TestTortureAtof64.txt")
+       defer tmpFile.Close()
+       tmpWriter := bufio.NewWriter(tmpFile)
+       defer tmpWriter.Flush()
+
        count := 0
        buf := make([]byte, 64)
        roundUp := false
@@ -140,6 +148,7 @@ func TestTortureAtof64(t *testing.T) {
                        t.Errorf("could not parse %q: %s", s, err)
                        return
                }
+               fmt.Fprintf(tmpWriter, "%s\n", s)
                expect := x
                if roundUp {
                        expect = y
```


## Users

Programs that use this test data set:

- `script/manual-test-parse-number-f64.cc` in
  [google/wuffs](https://github.com/google/wuffs)
