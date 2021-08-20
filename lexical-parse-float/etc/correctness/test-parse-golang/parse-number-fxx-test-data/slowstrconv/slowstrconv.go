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

// Package slowstrconv provides a slow, simple implementation of the standard
// library's strconv.ParseFloat function. Simplicity of implementation makes it
// easier to review for correctness.
//
// Its purpose is to generate test data for exercising strconv.ParseFloat
// (whose goals include runtime performance, not just correctness) and
// similarly optimized functions in other packages.
package slowstrconv

import (
	"errors"
	"strconv"
)

// threshold is such that 1e+threshold and 1e-threshold are effectively
// infinity and zero. The largest and smallest positive finite fxx values are
// approximately:
//
//      Largest    Smallest   Smallest
//      Normal     Normal     Subnormal
// f16  6.55e+4    6.10e-5    5.96e-8
// f32  3.40e+38   1.18e-38   1.40e-45
// f64  1.80e+308  2.23e-308  4.94e-324
const threshold = 350

var errInvalidNumber = errors.New("slowstrconv: invalid number")

// ParseFloatResult contains half-, single- and double-precision floating point
// approximations to a number.
type ParseFloatResult struct {
	F16 uint16 // 1 sign,  5 exponent (  -15 bias), 10 mantissa bits.
	F32 uint32 // 1 sign,  8 exponent ( -127 bias), 23 mantissa bits.
	F64 uint64 // 1 sign, 11 exponent (-1023 bias), 52 mantissa bits.
}

// ParseFloatFromBytes takes input like "1.23e45" and returns the half-,
// single- and double-precision floating point approximations to that number.
func ParseFloatFromBytes(s []byte) (ParseFloatResult, error) {
	negative := false
	if (len(s) == 0) || (len(s) >= 32768) {
		return ParseFloatResult{}, errInvalidNumber
	} else if s[0] == '+' {
		s = s[1:]
	} else if s[0] == '-' {
		s = s[1:]
		negative = true
	}

	r, err := parseFloatFromBytes(s)
	if err != nil {
		return r, err
	} else if negative {
		r.F16 |= 0x8000
		r.F32 |= 0x8000_0000
		r.F64 |= 0x8000_0000_0000_0000
	}
	return r, nil
}

func parseFloatFromBytes(s []byte) (ret ParseFloatResult, retErr error) {
	h := highPrecisionDecimal{}
	if !h.parse(s) {
		return ParseFloatResult{}, errInvalidNumber
	}

	// Repeatedly scale by 2 until we're in the half-open range [1 .. 2].
	exp2 := int32(0)
	for {
		// Handle zero and obvious extremes.
		if (h.numDigits == 0) || (h.decimalPoint < -threshold) {
			return ParseFloatResult{
				F16: 0x0000,
				F32: 0x0000_0000,
				F64: 0x0000_0000_0000_0000,
			}, nil
		} else if h.decimalPoint > +threshold {
			return ParseFloatResult{
				F16: 0x7C00,
				F32: 0x7F80_0000,
				F64: 0x7FF0_0000_0000_0000,
			}, nil
		}

		if h.decimalPoint < 1 {
			// Multiply by 2 if h is positive and less than 1.
			h.mul2()
			exp2--
		} else if (h.decimalPoint > 1) || ((h.decimalPoint == 1) && (h.digits[0] >= 2)) {
			// Divide by 2 if h greater than or equal to 2.
			h.div2()
			exp2++
		} else {
			break
		}
	}

	// Start with h in the range [1<<0 .. 1<<1].
	//
	// Scale h to be in the range [1<<10 .. 1<<11].
	h.mul2NTimes(10)
	ret.F16 = uint16(h.pack(exp2, 5, 10))
	// Scale h to be in the range [1<<23 .. 1<<24].
	h.mul2NTimes(23 - 10)
	ret.F32 = uint32(h.pack(exp2, 8, 23))
	// Scale h to be in the range [1<<52 .. 1<<53].
	h.mul2NTimes(52 - 23)
	ret.F64 = uint64(h.pack(exp2, 11, 52))
	return ret, nil
}

// hpdPrecision is somewhat arbitrary such that the highPrecisionDecimal.digits
// array is sufficiently large (at least several hundred digits of precision)
// and unsafe.Sizeof(highPrecisionDecimal{}) is an aesthetically pleasing 1024.
const hpdPrecision = 1024 - 7

// highPrecisionDecimal is a fixed precision floating point decimal number. It
// has hundreds of digits of precision.
//
// For example, if digits[:numDigits] is {7, 8, 9}:
//   - A decimalPoint of -2 means ".00789"
//   - A decimalPoint of -1 means ".0789"
//   - A decimalPoint of +0 means ".789"
//   - A decimalPoint of +1 means "7.89"
//   - A decimalPoint of +2 means "78.9"
//   - A decimalPoint of +3 means "789."
//   - A decimalPoint of +4 means "7890."
//   - A decimalPoint of +5 means "78900."
//
// truncated set true means that there are non-zero digits after the first
// hpdPrecision digits. This can affect rounding, where "a half exactly" could
// round down but "a half and a little bit more" could round up.
type highPrecisionDecimal struct {
	decimalPoint int32
	numDigits    uint16
	truncated    bool
	digits       [hpdPrecision]uint8
}

func (h *highPrecisionDecimal) String() string {
	s := []byte(nil)
	s = append(s, "dp="...)
	s = strconv.AppendInt(s, int64(h.decimalPoint), 10)
	s = append(s, ",digits="...)
	for _, digit := range h.digits[:h.numDigits] {
		s = append(s, '0'+digit)
	}
	if h.truncated {
		s = append(s, '+')
	}
	return string(s)
}

func (h *highPrecisionDecimal) parse(s []byte) (ok bool) {
	i := 0
	sawDigits := false
	sawDot := false
	for ; i < len(s); i++ {
		if c := s[i]; c == '.' {
			if sawDot {
				return false
			}
			sawDot = true
			h.decimalPoint = int32(h.numDigits)
		} else if ('0' <= c) && (c <= '9') {
			sawDigits = true
			if (c == '0') && (h.numDigits == 0) { // Ignore leading zeroes.
				h.decimalPoint--
			} else if h.numDigits < hpdPrecision {
				h.digits[h.numDigits] = c - '0'
				h.numDigits++
			} else if c != '0' {
				h.truncated = true
			}
		} else {
			break
		}
	}
	if !sawDigits {
		return false
	}
	if !sawDot {
		h.decimalPoint = int32(h.numDigits)
	}

	// An optional exponent moves the decimal point. If we read a very large,
	// very long number, just be sure to move the decimal point by a big
	// number. It doesn't matter if it's not the exact number.
	if (i < len(s)) && ((s[i] == 'E') || (s[i] == 'e')) {
		i++
		if i >= len(s) {
			return false
		}
		eSign := int32(+1)
		if s[i] == '+' {
			i++
		} else if s[i] == '-' {
			eSign = -1
			i++
		}
		if (i >= len(s)) || (s[i] < '0') || ('9' < s[i]) {
			return false
		}

		e := int32(0)
		for ; i < len(s); i++ {
			if (s[i] < '0') || ('9' < s[i]) {
				return false
			}
			if e < (30 * threshold) {
				e = e*10 + int32(s[i]-'0')
			}
		}
		h.decimalPoint += e * eSign
	}

	if i != len(s) {
		return false
	}
	h.trimTrailingZeroes()
	return true
}

func (h *highPrecisionDecimal) div2() {
	// Read and write indexes, working left to right.
	rx := 0
	wx := 0

	acc := uint32(0)
	if h.numDigits == 0 {
		// h's number used to be zero and remains zero.
		return
	} else if (h.numDigits == 1) || (h.digits[0] >= 2) {
		acc = uint32(h.digits[0])
		rx = 1
	} else {
		acc = (10 * uint32(h.digits[0])) + uint32(h.digits[1])
		rx = 2
		h.decimalPoint--
		if h.decimalPoint < -threshold {
			h.numDigits = 0
			h.decimalPoint = 0
			h.truncated = false
			return
		}
	}

	for (rx < int(h.numDigits)) || (acc > 0) {
		newDigit := uint8(acc >> 1)

		acc = 10 * (acc & 1)
		if rx < int(h.numDigits) {
			acc += uint32(h.digits[rx])
			rx++
		}

		if wx < hpdPrecision {
			h.digits[wx] = newDigit
			wx++
		} else if newDigit > 0 {
			h.truncated = true
		}
	}

	h.numDigits = uint16(wx)
	h.trimTrailingZeroes()
}

func (h *highPrecisionDecimal) mul2() {
	numNewDigits := uint16(0)
	if h.numDigits == 0 {
		// h's number used to be zero and remains zero.
		return
	} else if h.digits[0] >= 5 {
		numNewDigits = 1
	}

	// Read and write indexes, working right to left.
	rx := int(h.numDigits - 1)
	wx := int(h.numDigits - 1 + numNewDigits)

	acc := uint32(0)
	for (rx >= 0) || (acc > 0) {
		if rx >= 0 {
			acc += uint32(h.digits[rx]) << 1
		}
		quo := acc / 10
		rem := acc % 10
		if wx < hpdPrecision {
			h.digits[wx] = uint8(rem)
		} else if rem > 0 {
			h.truncated = true
		}
		acc = quo
		rx--
		wx--
	}

	h.numDigits += numNewDigits
	if h.numDigits > hpdPrecision {
		h.numDigits = hpdPrecision
	}
	h.decimalPoint += int32(numNewDigits)
	h.trimTrailingZeroes()
}

func (h *highPrecisionDecimal) mul2NTimes(n uint32) {
	for ; n > 0; n-- {
		h.mul2()
	}
}

func (h *highPrecisionDecimal) trimTrailingZeroes() {
	for (h.numDigits > 0) && (h.digits[h.numDigits-1] == 0) {
		h.numDigits--
	}
}

func (h *highPrecisionDecimal) roundedInteger() (n uint64) {
	if h.decimalPoint < 0 {
		return 0 // "Point zero something" rounds to zero.
	}
	i := int32(0)
	for ; (i < h.decimalPoint) && (i < int32(h.numDigits)); i++ {
		n = (10 * n) + uint64(h.digits[i])
	}
	for ; i < h.decimalPoint; i++ {
		n = 10 * n
	}

	if i >= int32(h.numDigits) {
		return n // No fractional part, so no rounding.
	} else if c := h.digits[i]; c < 5 {
		return n + 0 // Round down.
	} else if (c > 5) || h.truncated {
		return n + 1 // Round up.
	}
	for i++; i < int32(h.numDigits); i++ {
		if h.digits[i] > 0 {
			return n + 1 // Round up.
		}
	}
	// Exactly half-way between two integers: NNN.500000etc. Round to even.
	if (n & 1) == 0 {
		return n + 0 // Round down.
	}
	return n + 1 // Round up.
}

func (h *highPrecisionDecimal) pack(exp2 int32, expBits uint32, manBits uint32) uint64 {
	exp2 += (int32(1) << (expBits - 1)) - 1
	exp2Adjustment := int32(0)
	man := h.roundedInteger()
	if (man >> (manBits + 1)) > 0 {
		man >>= 1
		exp2Adjustment = 1
	}
	if e, eMax := exp2+exp2Adjustment, int32((1<<expBits)-1); e >= eMax {
		return uint64(eMax) << manBits
	} else if e > 0 {
		man &= (uint64(1) << manBits) - 1
		return (uint64(e) << manBits) | man
	}

	subnormal := *h
	for exp2 <= 0 {
		if (subnormal.numDigits == 0) || (subnormal.decimalPoint < 0) {
			return 0
		}
		subnormal.div2()
		exp2++
	}
	return subnormal.roundedInteger()
}
