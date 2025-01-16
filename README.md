this need nightly to work


https://bim.easyaccessmaterials.com/protected/content/dcs_cc3/geo/geo_ref_sheet.pdf


cargo test
cargo bench
--timings

https://nnethercote.github.io/perf-book/bounds-checks.html
https://github.com/Shnatsel/bounds-check-cookbook/
https://www.randomlists.com/random-numbers?dup=false&qty=50000&max=9999&min=1

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
https://github.dev/rust-lang/libm
## rust is werid
how to get rust to opztime for your cpu 

find your cpu target,for example github codespace youse ryzen epic cpu whick zen3 cores

just add this to to cargo  change the target-cpu and targetfeature for cpu 

```toml
[profile.release]
opt-level = 3
debug = false
codegen-units = 1
lto = true
panic = 'unwind'
incremental = false
overflow-checks = true
strip = "debuginfo"
target-cpu = "znver3"
target-feature = "+avx2"
```


input code
```rust 
#[no_mangle]
pub fn get_distance_2(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    (x.powf(2.0) + y.powf(2.0)).sqrt()
}
```


```asm
get_distance_2:
        unpcklpd        xmm1, xmm3
        unpcklpd        xmm0, xmm2
        subpd   xmm1, xmm0
        mulpd   xmm1, xmm1
        movapd  xmm0, xmm1
        unpckhpd        xmm0, xmm1
        addsd   xmm0, xmm1
        sqrtsd  xmm0, xmm0
        ret
```

but if you add `-C target-cpu=native`

```
get_distance_2:
        vsubsd  xmm0, xmm1, xmm0
        vsubsd  xmm1, xmm3, xmm2
        vmulsd  xmm0, xmm0, xmm0
        vmulsd  xmm1, xmm1, xmm1
        vaddsd  xmm0, xmm0, xmm1
        vsqrtsd xmm0, xmm0, xmm0
        ret
```
