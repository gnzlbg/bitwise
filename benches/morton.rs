#[macro_use]
extern crate bencher;

extern crate bitwise;

use bencher::Bencher;

use std::fmt::Debug;
use bitwise::word::Word;
use bitwise::word::morton;

use bitwise::word::morton::testing_utils::*;

struct BMI2Encode2D;
impl RunnerFn for BMI2Encode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bmi2::encode_2d::<T, T>(bencher::black_box(v),
                                                           bencher::black_box(v)));
    }
}

fn encode_2d_u08_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BMI2Encode2D {}));
}
fn encode_2d_u16_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BMI2Encode2D {}));
}
fn encode_2d_u32_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BMI2Encode2D {}));
}
fn encode_2d_u64_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BMI2Encode2D {}));
}


struct LUTEncode2D;
impl RunnerFn for LUTEncode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::lut::encode_2d::<T, T>(bencher::black_box(v),
                                                          bencher::black_box(v)));
    }
}

fn encode_2d_u08_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&LUTEncode2D {}));
}
fn encode_2d_u16_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&LUTEncode2D {}));
}
fn encode_2d_u32_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&LUTEncode2D {}));
}
fn encode_2d_u64_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&LUTEncode2D {}));
}


struct BITMASKEncode2D;
impl RunnerFn for BITMASKEncode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bitmask::encode_2d::<T, T>(bencher::black_box(v),
                                                            bencher::black_box(v)));
    }
}

fn encode_2d_u08_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BITMASKEncode2D {}));
}
fn encode_2d_u16_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BITMASKEncode2D {}));
}
fn encode_2d_u32_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BITMASKEncode2D {}));
}
fn encode_2d_u64_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BITMASKEncode2D {}));
}


struct Encode2D;
impl RunnerFn for Encode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::encode_2d::<T>(bencher::black_box(v), bencher::black_box(v)));
    }
}

fn encode_2d_u08(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&Encode2D {}));
}
fn encode_2d_u16(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&Encode2D {}));
}
fn encode_2d_u32(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&Encode2D {}));
}
fn encode_2d_u64(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&Encode2D {}));
}


struct BMI2Decode2D;
impl RunnerFn for BMI2Decode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bmi2::decode_2d::<T, T>(bencher::black_box(v)));
    }
}

fn decode_2d_u08_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BMI2Decode2D {}));
}
fn decode_2d_u16_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BMI2Decode2D {}));
}
fn decode_2d_u32_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BMI2Decode2D {}));
}
fn decode_2d_u64_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BMI2Decode2D {}));
}


struct LUTDecode2D;
impl RunnerFn for LUTDecode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::lut::decode_2d::<T, T>(bencher::black_box(v)));
    }
}

fn decode_2d_u08_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&LUTDecode2D {}));
}
fn decode_2d_u16_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&LUTDecode2D {}));
}
fn decode_2d_u32_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&LUTDecode2D {}));
}
fn decode_2d_u64_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&LUTDecode2D {}));
}


struct BITMASKDecode2D;
impl RunnerFn for BITMASKDecode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bitmask::decode_2d::<T, T>(bencher::black_box(v)));
    }
}

fn decode_2d_u08_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BITMASKDecode2D {}));
}
fn decode_2d_u16_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BITMASKDecode2D {}));
}
fn decode_2d_u32_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BITMASKDecode2D {}));
}
fn decode_2d_u64_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BITMASKDecode2D {}));
}

struct Decode2D;
impl RunnerFn for Decode2D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::decode_2d::<T>(bencher::black_box(v)));
    }
}

fn decode_2d_u08(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&Decode2D {}));
}
fn decode_2d_u16(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&Decode2D {}));
}
fn decode_2d_u32(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&Decode2D {}));
}
fn decode_2d_u64(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&Decode2D {}));
}


struct BMI2Encode3D;
impl RunnerFn for BMI2Encode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bmi2::encode_3d::<T, T>(bencher::black_box(v),
                                                           bencher::black_box(v),
                                                           bencher::black_box(v)));
    }
}

fn encode_3d_u08_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BMI2Encode3D {}));
}
fn encode_3d_u16_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BMI2Encode3D {}));
}
fn encode_3d_u32_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BMI2Encode3D {}));
}
fn encode_3d_u64_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BMI2Encode3D {}));
}


struct LUTEncode3D;
impl RunnerFn for LUTEncode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::lut::encode_3d::<T, T>(bencher::black_box(v),
                                                          bencher::black_box(v),
                                                          bencher::black_box(v)));
    }
}

fn encode_3d_u08_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&LUTEncode3D {}));
}
fn encode_3d_u16_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&LUTEncode3D {}));
}
fn encode_3d_u32_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&LUTEncode3D {}));
}
fn encode_3d_u64_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&LUTEncode3D {}));
}


struct BITMASKEncode3D;
impl RunnerFn for BITMASKEncode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bitmask::encode_3d::<T, T>(bencher::black_box(v),
                                                            bencher::black_box(v),
                                                            bencher::black_box(v)));
    }
}

fn encode_3d_u08_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BITMASKEncode3D {}));
}
fn encode_3d_u16_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BITMASKEncode3D {}));
}
fn encode_3d_u32_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BITMASKEncode3D {}));
}
fn encode_3d_u64_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BITMASKEncode3D {}));
}


struct Encode3D;
impl RunnerFn for Encode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::encode_3d::<T>(bencher::black_box(v),
                                                  bencher::black_box(v),
                                                  bencher::black_box(v)));
    }
}

fn encode_3d_u08(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&Encode3D {}));
}
fn encode_3d_u16(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&Encode3D {}));
}
fn encode_3d_u32(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&Encode3D {}));
}
fn encode_3d_u64(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&Encode3D {}));
}


struct BMI3Decode3D;
impl RunnerFn for BMI3Decode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bmi2::decode_3d::<T, T>(bencher::black_box(v)));
    }
}

fn decode_3d_u08_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BMI3Decode3D {}));
}
fn decode_3d_u16_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BMI3Decode3D {}));
}
fn decode_3d_u32_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BMI3Decode3D {}));
}
fn decode_3d_u64_bmi2(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BMI3Decode3D {}));
}


struct LUTDecode3D;
impl RunnerFn for LUTDecode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::lut::decode_3d::<T, T>(bencher::black_box(v)));
    }
}

fn decode_3d_u08_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&LUTDecode3D {}));
}
fn decode_3d_u16_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&LUTDecode3D {}));
}
fn decode_3d_u32_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&LUTDecode3D {}));
}
fn decode_3d_u64_lut(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&LUTDecode3D {}));
}


struct BITMASKDecode3D;
impl RunnerFn for BITMASKDecode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::bitmask::decode_3d::<T, T>(bencher::black_box(v)));
    }
}

fn decode_3d_u08_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&BITMASKDecode3D {}));
}
fn decode_3d_u16_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&BITMASKDecode3D {}));
}
fn decode_3d_u32_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&BITMASKDecode3D {}));
}
fn decode_3d_u64_bitmask(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&BITMASKDecode3D {}));
}

struct Decode3D;
impl RunnerFn for Decode3D {
    fn run<T: Word + Debug>(&self, v: T) {
        bencher::black_box(morton::decode_3d::<T>(bencher::black_box(v)));
    }
}

fn decode_3d_u08(b: &mut Bencher) {
    b.iter(|| Runner::run_u8(&Decode3D {}));
}
fn decode_3d_u16(b: &mut Bencher) {
    b.iter(|| Runner::run_u16(&Decode3D {}));
}
fn decode_3d_u32(b: &mut Bencher) {
    b.iter(|| Runner::run_u32(&Decode3D {}));
}
fn decode_3d_u64(b: &mut Bencher) {
    b.iter(|| Runner::run_u64(&Decode3D {}));
}


benchmark_group!(encode_2d_u08_g,
                 encode_2d_u08_bmi2,
                 encode_2d_u08_lut,
                 encode_2d_u08_bitmask,
                 encode_2d_u08);
benchmark_group!(decode_2d_u08_g,
                 decode_2d_u08_bmi2,
                 decode_2d_u08_lut,
                 decode_2d_u08_bitmask,
                 decode_2d_u08);
benchmark_group!(encode_3d_u08_g,
                 encode_3d_u08_bmi2,
                 encode_3d_u08_lut,
                 encode_3d_u08_bitmask,
                 encode_3d_u08);
benchmark_group!(decode_3d_u08_g,
                 decode_3d_u08_bmi2,
                 decode_3d_u08_lut,
                 decode_3d_u08_bitmask,
                 decode_3d_u08);

benchmark_group!(encode_2d_u16_g,
                 encode_2d_u16_bmi2,
                 encode_2d_u16_lut,
                 encode_2d_u16_bitmask,
                 encode_2d_u16);
benchmark_group!(decode_2d_u16_g,
                 decode_2d_u16_bmi2,
                 decode_2d_u16_lut,
                 decode_2d_u16_bitmask,
                 decode_2d_u16);
benchmark_group!(encode_3d_u16_g,
                 encode_3d_u16_bmi2,
                 encode_3d_u16_lut,
                 encode_3d_u16_bitmask,
                 encode_3d_u16);
benchmark_group!(decode_3d_u16_g,
                 decode_3d_u16_bmi2,
                 decode_3d_u16_lut,
                 decode_3d_u16_bitmask,
                 decode_3d_u16);

benchmark_group!(encode_2d_u32_g,
                 encode_2d_u32_bmi2,
                 encode_2d_u32_lut,
                 encode_2d_u32_bitmask,
                 encode_2d_u32);
benchmark_group!(decode_2d_u32_g,
                 decode_2d_u32_bmi2,
                 decode_2d_u32_lut,
                 decode_2d_u32_bitmask,
                 decode_2d_u32);
benchmark_group!(encode_3d_u32_g,
                 encode_3d_u32_bmi2,
                 encode_3d_u32_lut,
                 encode_3d_u32_bitmask,
                 encode_3d_u32);
benchmark_group!(decode_3d_u32_g,
                 decode_3d_u32_bmi2,
                 decode_3d_u32_lut,
                 decode_3d_u32_bitmask,
                 decode_3d_u32);

benchmark_group!(encode_2d_u64_g,
                 encode_2d_u64_bmi2,
                 encode_2d_u64_lut,
                 encode_2d_u64_bitmask,
                 encode_2d_u64);
benchmark_group!(decode_2d_u64_g,
                 decode_2d_u64_bmi2,
                 decode_2d_u64_lut,
                 decode_2d_u64_bitmask,
                 decode_2d_u64);
benchmark_group!(encode_3d_u64_g,
                 encode_3d_u64_bmi2,
                 encode_3d_u64_lut,
                 encode_3d_u64_bitmask,
                 encode_3d_u64);
benchmark_group!(decode_3d_u64_g,
                 decode_3d_u64_bmi2,
                 decode_3d_u64_lut,
                 decode_3d_u64_bitmask,
                 decode_3d_u64);


benchmark_main!(encode_2d_u08_g,
                decode_2d_u08_g,
                encode_3d_u08_g,
                decode_3d_u08_g,
                encode_2d_u16_g,
                decode_2d_u16_g,
                encode_3d_u16_g,
                decode_3d_u16_g,
                encode_2d_u32_g,
                decode_2d_u32_g,
                encode_3d_u32_g,
                decode_3d_u32_g,
                encode_2d_u64_g,
                decode_2d_u64_g,
                encode_3d_u64_g,
                decode_3d_u64_g);
