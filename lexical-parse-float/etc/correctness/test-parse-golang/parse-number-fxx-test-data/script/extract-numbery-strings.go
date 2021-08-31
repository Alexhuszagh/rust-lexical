// Copyright 2020 The ParseNumberFxxTestData Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ----------------

// +build ignore

package main

// extract-numbery-strings.go is like /usr/bin/strings but only extracts
// strings that look like floating point numbers, such as "123.456e789". The
// leading '+' or '-' is omitted.
//
// It prints one number per line, with the string preceded by the 16-bit,
// 32-bit and 64-bit (half-, single- and double-precision) IEEE floating point
// representation of the number. For example:
//
// 3C00 3F800000 3FF0000000000000 1
// 3D00 3FA00000 3FF4000000000000 1.25
// 3D9A 3FB33333 3FF6666666666666 1.4
// 57B7 42F6E979 405EDD2F1A9FBE77 123.456
// 622A 44454000 4088A80000000000 789
// 7C00 7F800000 7FF0000000000000 123.456e789
//
// For example, parsing "1.4" as a float32 gives the bits 0x3FB33333.
//
// In this case, the final line's float16, float32 and float64 values are all
// infinity. The largest finite float{16,32,64} values are approximately
// 6.55e+4, 3.40e+38 and 1.80e+308.

import (
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"sort"
	"unicode/utf8"

	"github.com/nigeltao/parse-number-fxx-test-data/slowstrconv"
)

const noisy = false

var numbers = map[string]struct{}{}

func isDigit(x byte) bool    { return (('0' <= x) && (x <= '9')) }
func isDigitDot(x byte) bool { return (('0' <= x) && (x <= '9')) || (x == '.') }

func main() {
	args := []string{"."}
	if len(os.Args) > 1 {
		args = os.Args[1:]
	}
	for _, arg := range args {
		if z, err := os.Stat(arg); err != nil {
			continue
		} else if z.IsDir() {
			doDir(arg)
		} else {
			doFile(arg)
		}
	}
	printSortedNumbers()
}

func doDir(filename string) {
	if b := filepath.Base(filename); (len(b) > 1) && (b[0] == '.') {
		return
	}
	if noisy {
		fmt.Fprintln(os.Stderr, "dir: ", filename)
	}

	f, err := os.Open(filename)
	if err != nil {
		return
	}
	defer f.Close()
	infos, err := f.Readdir(-1)
	if err != nil {
		return
	}
	for _, info := range infos {
		childname := filepath.Join(filename, info.Name())
		if info.IsDir() {
			doDir(childname)
		} else {
			doFile(childname)
		}
	}
}

func doFile(filename string) {
	if b := filepath.Base(filename); (len(b) > 1) && (b[0] == '.') {
		return
	}
	src, err := ioutil.ReadFile(filename)
	if (err != nil) || looksLikeBinary(src) {
		return
	} else if noisy {
		fmt.Fprintln(os.Stderr, "file:", filename)
	}

	for i := 0; i < len(src); {
		c := src[i]
		if !isDigitDot(c) {
			i++
			continue
		}
		start := i

		// Grab "123.456".
		for dotted := false; i < len(src); {
			c := src[i]
			if isDigit(c) {
				i++
				continue
			} else if (c == '.') && !dotted {
				dotted = true
				i++
				continue
			}
			break
		}

		// Grab "e+789".
		if (i < len(src)) && ((src[i] == 'E') || (src[i] == 'e')) {
			j := i + 1
			if (j < len(src)) && ((src[j] == '+') || (src[j] == '-')) {
				j++
			}
			for (j < len(src)) && isDigit(src[j]) {
				j++
				i = j
			}
		}

		s := string(src[start:i])
		if len(s) > 1024 {
			s = s[:1024]
		}

		// Trim leading '0's and trailing non-digits.
		for (len(s) > 1) && (s[0] == '0') && isDigit(s[1]) {
			s = s[1:]
		}
		for (len(s) > 0) && !isDigit(s[len(s)-1]) {
			s = s[:len(s)-1]
		}

		if s != "" {
			numbers[s] = struct{}{}
		}
	}
}

func printSortedNumbers() {
	sortedNumbers := make([]string, 0, len(numbers))
	for n := range numbers {
		if f, err := slowstrconv.ParseFloatFromBytes([]byte(n)); err == nil {
			sortedNumbers = append(sortedNumbers,
				fmt.Sprintf("%04X %08X %016X %s", f.F16, f.F32, f.F64, n))
		}
	}
	sort.Strings(sortedNumbers)
	for _, n := range sortedNumbers {
		fmt.Println(n)
	}
}

func looksLikeBinary(s []byte) bool {
	if len(s) > 1024 {
		s = s[:1024]
	}
	for i := 0; (i < 4) && (len(s) > 0); i++ {
		if r, size := utf8.DecodeLastRune(s); (r != utf8.RuneError) || (size != 1) {
			break
		}
		s = s[:len(s)-1]
	}
	return !utf8.Valid(s)
}
