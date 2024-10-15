
to opztie said rust program for cpu 
run "rustc --print target-cpus"
read the curret cpu 
add -Z tune-cpu=znver3 -C target-cpu=znver3

-Z tune-cpu=native -C opt-level=3 -C overflow-checks=true -C strip=symbols -C target-cpu=native -C code-model=small -C codegen-units=0 -C control-flow-guard=false -C debug-assertions=false -C debuginfo=0 -C embed-bitcode=yes -C force-unwind-tables=yes -C link-self-contained=yes -C lto=fat -C relocation-model=pie -C relro-level=full

# build arg
```sh
-Z tune-cpu=machine -C opt-level=3 -C overflow-checks=true -C strip=debuginfo -C target-cpu=native
```

# Impltions

### 1

```rust 
#[no_mangle]
pub fn get_distance_1(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt()
}
```
```asm
get_distance_3:
        vsubsd  xmm0, xmm1, xmm0
        vmulsd  xmm0, xmm0, xmm0
        vsubsd  xmm1, xmm3, xmm2
        vmulsd  xmm1, xmm1, xmm1
        vaddsd  xmm0, xmm0, xmm1
        vsqrtsd xmm0, xmm0, xmm0
        ret
```

2

```rust
#[no_mangle]
pub fn get_distance_2(x1:f64,x2:f64,y1:f64,y2:f64) ->f64 {
    let x:f64 = (x2-x1).powf(2.0);
    let y:f64 = (y2-y1).powf(2.0);
    return (x+y).sqrt();
}
```
```asm
get_distance_3:
        vsubsd  xmm0, xmm1, xmm0
        vmulsd  xmm0, xmm0, xmm0
        vsubsd  xmm1, xmm3, xmm2
        vmulsd  xmm1, xmm1, xmm1
        vaddsd  xmm0, xmm0, xmm1
        vsqrtsd xmm0, xmm0, xmm0
        ret
```

3
```rust 
#[no_mangle]
pub fn get_distance_3(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    (x2 - x1).hypot(y2 - y1)
}
```
```asm
get_distance_3:
        vsubsd  xmm0, xmm1, xmm0
        vsubsd  xmm1, xmm3, xmm2
        jmp     qword ptr [rip + hypot@GOTPCREL]
```

4
```rust 
#[no_mangle]
pub fn get_distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    (x * x + y * y).sqrt()
}
```
```asm 
get_distance:
        vsubsd  xmm0, xmm1, xmm0
        vsubsd  xmm1, xmm3, xmm2
        vmulsd  xmm0, xmm0, xmm0
        vmulsd  xmm1, xmm1, xmm1
        vaddsd  xmm0, xmm0, xmm1
        vsqrtsd xmm0, xmm0, xmm0
        ret
```





https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DEArgoKkl9ZATwDKjdAGFUtEywYgATKUcAZPAZMADl3ACNMYhBpAAdUBUI7Bhc3Dz04hNsBAKDQlgiorktMayyGIQImYgIU908fK0wbJIqqghyQ8Mjoy0rq2rSi8zaOvIKJAEpLVBNiZHYOAFIvAGZFgFYAIQZUAH0WQ2B6DYARRY0AQRiTMIBqKgZb4EwCXfxhhnndlYhVLhB7gA2aS3VReAFUYGkW4AT3%2BQJBMPBCImtwAtIsVtgEbdFgB2TbnC63Em3KBg9GgrgTAB0cQA7lQIF4aRpUctNmSkZS4bSGUyWWzaQoAI7VCATIn4s6XAD0suWay2O32h2O6xlF3l1zuDyeLzeeA%2BXy8v3hkJBYIhUNh5ptSOtklRGKxOPxhLlstJt3oBFBuJWJ1BXkpf0xHq1XtJvthAaD3LRtvDRPl3t%2BdNQjOZrPZXk5MIzWcFE2FYoIEpTCrxmqJHCmtE4614nm4vFQnAASmY/QoZnNMLjVjxSARNHXSBAkGgsLhCCRyJQaPRmGxOMP%2BIIRGJ2FIZIJFCp1BwtKRdD4DEYQKZzMVSkkHAxnK46t5fI/Rl0onjSBlEgIBp4RS/mUH75N0DQlE0ZStP0z5pBBd4CDB7SBJ0YFfr0bQAXowzVKB4xeFMvazPMnBSqsGzbHsBxGOqmo6vcjzPK87yVJ8mBmo60JWgi0JwlxsLIhazqYtiFq4gSRLejGqhxsGoZcMmlzSS8saYvGIaJnCSnEqSvy3AAVP6HKxkZMKluKkqXNK44NhwTakC2J7thwXbmLcxH9oOKxeLwo7HnWUwANbRAAnDSAAcXiSKFoVeF46wRXiGihRFRR2ZIzZjqQLm8Aoej%2BVoUxwLAU6oCwMR0JEC4QGgFVVVEDB4MACAELQMJ8HQBCRPlEBhNlYSBFUMJrrwg3MMQMIAPJhNoUGjaQdVsIIU0MO12VYGEJjAE4Yi0PlrakFgNFHAsJ74MQUF4AAbpgB0npgqhNCY3ULYE3V2SetB4GExDDS4WDZQQxB4CwC23cQYTxJgJyYCd32XgFUxUAYwAKAAangmD0lNMSMAtG7CKI4i7oTB5qNlujpYcV7dvoP35ZAUyoDEZQHWiABatwECYQRosg1zqQcyAIKh6JOLcLMEGi9C3bQ6krOLksQyjmb8wgTRBQo6nAyYA5ohL5ggzE6mbdtgT8ErfTMfzguBswti3W2EMg1gjNQCu7DmOgEOkNdYh6ygHIcl4Qq3ld9gQI42FFP4qFjN0QHxH%2ByRwYBP7JyB8efjhkHNEhfQ1GnueIeUhf4YnmGwak6e4ShuQ59S0wkTu9aNllAU5ZwtzXgQyC3E1LVtTCZKzkQxDedSfljsVZX1fQZAULV5WVQvICD617WdbQ3XEL1/Wd%2BNw0LUfk0zXNNgLUtjAEKt62d2bO17Qdw7HWqZ28BdV23fdvCPc9r1DrvRKNlb6v1/oYA/iOEGYNDoQyhkoWG8NAigCRnwVGGMsY4zxi2dcsgtwk2kGTJQFNO5nn0DTXu9Mwju2ZqzJI7MuY8z5gLEwQsmAizFgbSWrMZaYDlgrJWqAVa0DViLTW2tAy631obYGeATaBkfhbVAVsqg21YepB2N1MDO0iK7O68AICexAJgfAZQ/YBwWKHLwPASzh3zp4KOj4Y4%2BDjg3dCIAIoZ0yEkGO3iU4V0KPY6ChcXHBJaOXbOHivF1z8XXQJkxm79m8G3eyHdnLd17v3Dew9R5mJIN5Qi080GThAPgKgVAaqEwITuIhshyZHi%2BggfK6VmnlKoAQGEeMQAaH0M0x6wMmBS3vBwXgxBmmvnGQodpnTum9KmQMv6wz7CjNSQ5JybZOAnDwBU24mNsaRB7t2bJzVN4j2ugoI55gTlD3asUoqwUQDrA0DSLgeJnkaDxHiSQXAvChQ0OeTgmVHLZVypYAqM9Um%2BRBZ3MFhVAp%2Bx6veSQQA