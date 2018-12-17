package parse

import "encoding/json"
import "io/ioutil"
import "os"
import "path"
import "strconv"
import "testing"

var denormal_data []string;
var large_data []string;
var digits2_data []string;
var digits8_data []string;
var digits16_data []string;
var digits32_data []string;
var digits64_data []string;
var result float64;

func BenchmarkDenormal10(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[0], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal20(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[1], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal30(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[2], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal40(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[3], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal50(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[4], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal100(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[5], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal200(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[6], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal400(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[7], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal800(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[8], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal1600(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[9], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal3200(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[10], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDenormal6400(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(denormal_data[11], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge10(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[0], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge20(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[1], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge30(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[2], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge40(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[3], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge50(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[4], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge100(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[5], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge200(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[6], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge400(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[7], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge800(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[8], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge1600(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[9], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge3200(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[10], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkLarge6400(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        if f, err := strconv.ParseFloat(large_data[11], 64); err == nil {
            r = f
        }
    }
    result = r
}

func BenchmarkDigits2(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        for _, element := range digits2_data {
            if f, err := strconv.ParseFloat(element, 64); err == nil {
                r = f
            }
        }
    }
    result = r
}

func BenchmarkDigits8(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        for _, element := range digits8_data {
            if f, err := strconv.ParseFloat(element, 64); err == nil {
                r = f
            }
        }
    }
    result = r
}

func BenchmarkDigits16(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        for _, element := range digits16_data {
            if f, err := strconv.ParseFloat(element, 64); err == nil {
                r = f
            }
        }
    }
    result = r
}

func BenchmarkDigits32(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        for _, element := range digits32_data {
            if f, err := strconv.ParseFloat(element, 64); err == nil {
                r = f
            }
        }
    }
    result = r
}

func BenchmarkDigits64(b *testing.B) {
    var r float64
    for n := 0; n < b.N; n++ {
        for _, element := range digits64_data {
            if f, err := strconv.ParseFloat(element, 64); err == nil {
                r = f
            }
        }
    }
    result = r
}

func TestMain(m *testing.M) {
    cwd, _ := os.Getwd();
    data_dir := path.Join(cwd, "..", "data");

    // denormal data.
    denormal_path := path.Join(data_dir, "denormal_halfway.json");
    denormal_json, _ := ioutil.ReadFile(denormal_path)
    json.Unmarshal([]byte(denormal_json), &denormal_data);

    // large data.
    large_path := path.Join(data_dir, "large_halfway.json");
    large_json, _ := ioutil.ReadFile(large_path)
    json.Unmarshal([]byte(large_json), &large_data);

    // digits2 data.
    digits2_path := path.Join(data_dir, "digits2.json");
    digits2_json, _ := ioutil.ReadFile(digits2_path)
    json.Unmarshal([]byte(digits2_json), &digits2_data);

    // digits8 data.
    digits8_path := path.Join(data_dir, "digits8.json");
    digits8_json, _ := ioutil.ReadFile(digits8_path)
    json.Unmarshal([]byte(digits8_json), &digits8_data);

    // digits16 data.
    digits16_path := path.Join(data_dir, "digits16.json");
    digits16_json, _ := ioutil.ReadFile(digits16_path)
    json.Unmarshal([]byte(digits16_json), &digits16_data);

    // digits32 data.
    digits32_path := path.Join(data_dir, "digits32.json");
    digits32_json, _ := ioutil.ReadFile(digits32_path)
    json.Unmarshal([]byte(digits32_json), &digits32_data);

    // digits64 data.
    digits64_path := path.Join(data_dir, "digits64.json");
    digits64_json, _ := ioutil.ReadFile(digits64_path)
    json.Unmarshal([]byte(digits64_json), &digits64_data);

    os.Exit(m.Run())
}
