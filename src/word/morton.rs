//! Encoding/decoding of Morton Z-curve indices.

#![allow(dead_code)]

use word;
use word::Word;

pub mod lut {
    //! Encoding/decoding of Morton Z-curve indices using a look-up table.

    use word::{Word, ToWord};

    const ENCODE_2D: [u16; 256] =
        [0, 1, 4, 5, 16, 17, 20, 21, 64, 65, 68, 69, 80, 81, 84, 85, 256, 257, 260, 261, 272, 273,
         276, 277, 320, 321, 324, 325, 336, 337, 340, 341, 1024, 1025, 1028, 1029, 1040, 1041,
         1044, 1045, 1088, 1089, 1092, 1093, 1104, 1105, 1108, 1109, 1280, 1281, 1284, 1285, 1296,
         1297, 1300, 1301, 1344, 1345, 1348, 1349, 1360, 1361, 1364, 1365, 4096, 4097, 4100, 4101,
         4112, 4113, 4116, 4117, 4160, 4161, 4164, 4165, 4176, 4177, 4180, 4181, 4352, 4353, 4356,
         4357, 4368, 4369, 4372, 4373, 4416, 4417, 4420, 4421, 4432, 4433, 4436, 4437, 5120, 5121,
         5124, 5125, 5136, 5137, 5140, 5141, 5184, 5185, 5188, 5189, 5200, 5201, 5204, 5205, 5376,
         5377, 5380, 5381, 5392, 5393, 5396, 5397, 5440, 5441, 5444, 5445, 5456, 5457, 5460, 5461,
         16384, 16385, 16388, 16389, 16400, 16401, 16404, 16405, 16448, 16449, 16452, 16453,
         16464, 16465, 16468, 16469, 16640, 16641, 16644, 16645, 16656, 16657, 16660, 16661,
         16704, 16705, 16708, 16709, 16720, 16721, 16724, 16725, 17408, 17409, 17412, 17413,
         17424, 17425, 17428, 17429, 17472, 17473, 17476, 17477, 17488, 17489, 17492, 17493,
         17664, 17665, 17668, 17669, 17680, 17681, 17684, 17685, 17728, 17729, 17732, 17733,
         17744, 17745, 17748, 17749, 20480, 20481, 20484, 20485, 20496, 20497, 20500, 20501,
         20544, 20545, 20548, 20549, 20560, 20561, 20564, 20565, 20736, 20737, 20740, 20741,
         20752, 20753, 20756, 20757, 20800, 20801, 20804, 20805, 20816, 20817, 20820, 20821,
         21504, 21505, 21508, 21509, 21520, 21521, 21524, 21525, 21568, 21569, 21572, 21573,
         21584, 21585, 21588, 21589, 21760, 21761, 21764, 21765, 21776, 21777, 21780, 21781,
         21824, 21825, 21828, 21829, 21840, 21841, 21844, 21845];

    const DECODE_2D: [u8; 256] =
        [0, 1, 0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3, 4, 5, 4, 5, 6, 7, 6, 7, 4, 5, 4, 5, 6, 7,
         6, 7, 0, 1, 0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3, 4, 5, 4, 5, 6, 7, 6, 7, 4, 5, 4, 5,
         6, 7, 6, 7, 8, 9, 8, 9, 10, 11, 10, 11, 8, 9, 8, 9, 10, 11, 10, 11, 12, 13, 12, 13, 14,
         15, 14, 15, 12, 13, 12, 13, 14, 15, 14, 15, 8, 9, 8, 9, 10, 11, 10, 11, 8, 9, 8, 9, 10,
         11, 10, 11, 12, 13, 12, 13, 14, 15, 14, 15, 12, 13, 12, 13, 14, 15, 14, 15, 0, 1, 0, 1,
         2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3, 4, 5, 4, 5, 6, 7, 6, 7, 4, 5, 4, 5, 6, 7, 6, 7, 0, 1,
         0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3, 4, 5, 4, 5, 6, 7, 6, 7, 4, 5, 4, 5, 6, 7, 6, 7,
         8, 9, 8, 9, 10, 11, 10, 11, 8, 9, 8, 9, 10, 11, 10, 11, 12, 13, 12, 13, 14, 15, 14, 15,
         12, 13, 12, 13, 14, 15, 14, 15, 8, 9, 8, 9, 10, 11, 10, 11, 8, 9, 8, 9, 10, 11, 10, 11,
         12, 13, 12, 13, 14, 15, 14, 15, 12, 13, 12, 13, 14, 15, 14, 15];

    const ENCODE_3D: [u32; 256] =
        [0x00000000, 0x00000001, 0x00000008, 0x00000009, 0x00000040, 0x00000041, 0x00000048,
         0x00000049, 0x00000200, 0x00000201, 0x00000208, 0x00000209, 0x00000240, 0x00000241,
         0x00000248, 0x00000249, 0x00001000, 0x00001001, 0x00001008, 0x00001009, 0x00001040,
         0x00001041, 0x00001048, 0x00001049, 0x00001200, 0x00001201, 0x00001208, 0x00001209,
         0x00001240, 0x00001241, 0x00001248, 0x00001249, 0x00008000, 0x00008001, 0x00008008,
         0x00008009, 0x00008040, 0x00008041, 0x00008048, 0x00008049, 0x00008200, 0x00008201,
         0x00008208, 0x00008209, 0x00008240, 0x00008241, 0x00008248, 0x00008249, 0x00009000,
         0x00009001, 0x00009008, 0x00009009, 0x00009040, 0x00009041, 0x00009048, 0x00009049,
         0x00009200, 0x00009201, 0x00009208, 0x00009209, 0x00009240, 0x00009241, 0x00009248,
         0x00009249, 0x00040000, 0x00040001, 0x00040008, 0x00040009, 0x00040040, 0x00040041,
         0x00040048, 0x00040049, 0x00040200, 0x00040201, 0x00040208, 0x00040209, 0x00040240,
         0x00040241, 0x00040248, 0x00040249, 0x00041000, 0x00041001, 0x00041008, 0x00041009,
         0x00041040, 0x00041041, 0x00041048, 0x00041049, 0x00041200, 0x00041201, 0x00041208,
         0x00041209, 0x00041240, 0x00041241, 0x00041248, 0x00041249, 0x00048000, 0x00048001,
         0x00048008, 0x00048009, 0x00048040, 0x00048041, 0x00048048, 0x00048049, 0x00048200,
         0x00048201, 0x00048208, 0x00048209, 0x00048240, 0x00048241, 0x00048248, 0x00048249,
         0x00049000, 0x00049001, 0x00049008, 0x00049009, 0x00049040, 0x00049041, 0x00049048,
         0x00049049, 0x00049200, 0x00049201, 0x00049208, 0x00049209, 0x00049240, 0x00049241,
         0x00049248, 0x00049249, 0x00200000, 0x00200001, 0x00200008, 0x00200009, 0x00200040,
         0x00200041, 0x00200048, 0x00200049, 0x00200200, 0x00200201, 0x00200208, 0x00200209,
         0x00200240, 0x00200241, 0x00200248, 0x00200249, 0x00201000, 0x00201001, 0x00201008,
         0x00201009, 0x00201040, 0x00201041, 0x00201048, 0x00201049, 0x00201200, 0x00201201,
         0x00201208, 0x00201209, 0x00201240, 0x00201241, 0x00201248, 0x00201249, 0x00208000,
         0x00208001, 0x00208008, 0x00208009, 0x00208040, 0x00208041, 0x00208048, 0x00208049,
         0x00208200, 0x00208201, 0x00208208, 0x00208209, 0x00208240, 0x00208241, 0x00208248,
         0x00208249, 0x00209000, 0x00209001, 0x00209008, 0x00209009, 0x00209040, 0x00209041,
         0x00209048, 0x00209049, 0x00209200, 0x00209201, 0x00209208, 0x00209209, 0x00209240,
         0x00209241, 0x00209248, 0x00209249, 0x00240000, 0x00240001, 0x00240008, 0x00240009,
         0x00240040, 0x00240041, 0x00240048, 0x00240049, 0x00240200, 0x00240201, 0x00240208,
         0x00240209, 0x00240240, 0x00240241, 0x00240248, 0x00240249, 0x00241000, 0x00241001,
         0x00241008, 0x00241009, 0x00241040, 0x00241041, 0x00241048, 0x00241049, 0x00241200,
         0x00241201, 0x00241208, 0x00241209, 0x00241240, 0x00241241, 0x00241248, 0x00241249,
         0x00248000, 0x00248001, 0x00248008, 0x00248009, 0x00248040, 0x00248041, 0x00248048,
         0x00248049, 0x00248200, 0x00248201, 0x00248208, 0x00248209, 0x00248240, 0x00248241,
         0x00248248, 0x00248249, 0x00249000, 0x00249001, 0x00249008, 0x00249009, 0x00249040,
         0x00249041, 0x00249048, 0x00249049, 0x00249200, 0x00249201, 0x00249208, 0x00249209,
         0x00249240, 0x00249241, 0x00249248, 0x00249249];

    const DECODE_3D: [u8; 512] =
        [0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3,
         2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3,
         2, 3, 2, 3, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7,
         6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5,
         6, 7, 6, 7, 6, 7, 6, 7, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1,
         0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1,
         0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 4, 5,
         4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7,
         4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3,
         2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3,
         2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7,
         6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5,
         6, 7, 6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 0, 1, 0, 1, 0, 1,
         0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1,
         0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 4, 5,
         4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7,
         4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7, 6, 7, 4, 5, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7,
         6, 7];

    #[inline]
    pub fn encode_2d<MortonIndex: Word, Coordinate: Word>(x: Coordinate,
                                                          y: Coordinate)
                                                          -> MortonIndex {
        let mut result = MortonIndex::zero();
        const MASK: u8 = 0x000000FF;
        let mask = Coordinate::from_u8(MASK);
        for i in (1..Coordinate::byte_size().to_u8() + 1).rev() {
            let shift = Coordinate::from_u8((i - 1) * 8);
            let y_index = ((y >> shift) & mask).to_u8() as usize;
            let x_index = ((x >> shift) & mask).to_u8() as usize;
            result = result.wrapping_shl(MortonIndex::from_u8(16)) |
                     (ENCODE_2D[y_index] << 1).to() |
                     (ENCODE_2D[x_index]).to();
        }
        result
    }

    #[inline]
    fn decode_2d_h<Coordinate: Word, MortonIndex: Word>(v: MortonIndex,
                                                        lut: &[u8],
                                                        startshift: u8)
                                                        -> Coordinate {
        let mut result = MortonIndex::zero();
        const MASK: u8 = 0x000000FF;
        let mask = MortonIndex::from_u8(MASK);
        let loops = MortonIndex::byte_size().to_u8();
        for i in 0..loops {
            let shift = (i * 8) + startshift;
            let index: usize = ((v >> shift.to()) & mask.to()).to();
            let val = MortonIndex::from_u8(lut[index]);
            result = result | (val << (4 * i as u32).to()).to();
        }
        result.to()
    }

    #[inline]
    pub fn decode_2d<Coordinate: Word, MortonIndex: Word>(v: MortonIndex)
                                                          -> (Coordinate, Coordinate) {
        (decode_2d_h(v, &DECODE_2D, 0), decode_2d_h(v, &DECODE_2D, 1))
    }

    #[inline]
    pub fn encode_3d<MortonIndex: Word, Coordinate: Word>(x: Coordinate,
                                                          y: Coordinate,
                                                          z: Coordinate)
                                                          -> MortonIndex {
        let mut result = MortonIndex::zero();
        const MASK: u8 = 0x000000FF;
        let mask = Coordinate::from_u8(MASK);
        for i in (1..Coordinate::byte_size().to_u8() + 1).rev() {
            let shift = Coordinate::from_u8((i - 1) * 8);
            let y_index = ((y >> shift) & mask).to_u8() as usize;
            let x_index = ((x >> shift) & mask).to_u8() as usize;
            let z_index = ((z >> shift) & mask).to_u8() as usize;
            result = result.wrapping_shl(MortonIndex::from_u8(24)) |
                     (ENCODE_3D[z_index] << 2).to() |
                     (ENCODE_3D[y_index] << 1).to() |
                     (ENCODE_3D[x_index]).to();
        }
        result
    }

    #[inline]
    fn decode_3d_h<Coordinate: Word, MortonIndex: Word>(v: MortonIndex,
                                                        lut: &[u8],
                                                        startshift: u8)
                                                        -> Coordinate {
        let mut result = MortonIndex::zero();
        const MASK: u16 = 0x000001ff; // TODO: try u8
        let mask = MortonIndex::from_u16(MASK);
        let loops = MortonIndex::byte_size().to_u8();
        let loops = if loops >= 8 { 7 } else { loops };
        for i in 0..loops {
            let shift = (i * 9) + startshift;
            let index: usize = ((v >> shift.to()) & mask).to();
            let val = MortonIndex::from_u8(lut[index]);
            result = result | (val << (3 * i as u32).to()).to();
        }
        result.to()
    }


    #[inline]
    pub fn decode_3d<Coordinate: Word, MortonIndex: Word>
        (v: MortonIndex)
         -> (Coordinate, Coordinate, Coordinate) {
        (decode_3d_h(v, &DECODE_3D, 0),
         decode_3d_h(v, &DECODE_3D, 1),
         decode_3d_h(v, &DECODE_3D, 2))
    }
}

pub mod bmi2 {
    //! Encoding/decoding of Morton Z-curve indices using BMI2 pdep/pext instructions.
    use word::{Word, ToWord, ParallelBitsDeposit, ParallelBitsExtract};

    #[inline]
    pub fn encode_2d<MortonIndex: Word, Coordinate: Word>(x: Coordinate,
                                                          y: Coordinate)
                                                          -> MortonIndex {
        (y.parallel_bits_deposit(0xAAAAAAAAAAAAAAAAu64) |
         x.parallel_bits_deposit(0x5555555555555555u64))
            .to()
    }

    #[inline]
    pub fn encode_3d<MortonIndex: Word, Coordinate: Word>(x: Coordinate,
                                                          y: Coordinate,
                                                          z: Coordinate)
                                                          -> MortonIndex {
        (z.parallel_bits_deposit(0x4924924924924924u64) |
         y.parallel_bits_deposit(0x2492492492492492u64) |
         x.parallel_bits_deposit(0x9249249249249249u64))
            .to()
    }

    #[inline]
    pub fn decode_2d<Coordinate: Word, MortonIndex: Word>(v: MortonIndex)
                                                          -> (Coordinate, Coordinate) {
        (v.parallel_bits_extract(0x5555555555555555u64).to(),
         v.parallel_bits_extract(0xAAAAAAAAAAAAAAAAu64).to())
    }


    #[inline]
    pub fn decode_3d<Coordinate: Word, MortonIndex: Word>
        (v: MortonIndex)
         -> (Coordinate, Coordinate, Coordinate) {
        (v.parallel_bits_extract(0x9249249249249249u64).to(),
         v.parallel_bits_extract(0x2492492492492492u64).to(),
         v.parallel_bits_extract(0x4924924924924924u64).to())
    }
}


pub mod magic {
    use word::Word;

    const MASK_2D_U32: [u32; 6] = [0xFFFFFFFF, 0x0000FFFF, 0x00FF00FF, 0x0F0F0F0F, 0x33333333,
                                   0x55555555];
    const MASK_2D_U64: [u64; 6] = [0x00000000FFFFFFFF,
                                   0x0000FFFF0000FFFF,
                                   0x00FF00FF00FF00FF,
                                   0x0F0F0F0F0F0F0F0F,
                                   0x3333333333333333,
                                   0x5555555555555555];

    const MASK_3D_U32: [u32; 5] = [0x000003ff, 0x30000ff, 0x0300f00f, 0x30c30c3, 0x9249249];
    const MASK_3D_U64: [u64; 6] = [0x1fffffu64,
                                   0x1f00000000ffffu64,
                                   0x1f0000ff0000ffu64,
                                   0x100f00f00f00f00fu64,
                                   0x10c30c30c30c30c3u64,
                                   0x1249249249249249u64];


    #[inline]
    fn split_by_second_bits<MortonIndex: Word, Coordinate: Word>(x: Coordinate) -> MortonIndex {
        match Coordinate::bit_size().to_u8() {
            32 => {
                let mut x = x.to_u32();
                x = (x | x << 16) & MASK_2D_U32[1];
                x = (x | x << 8) & MASK_2D_U32[2];
                x = (x | x << 4) & MASK_2D_U32[3];
                x = (x | x << 2) & MASK_2D_U32[4];
                x = (x | x << 1) & MASK_2D_U32[5];
                MortonIndex::from_u32(x)
            }
            64 => {
                let mut x = x.to_u64();
                x = (x | x << 32) & MASK_2D_U64[0];
                x = (x | x << 16) & MASK_2D_U64[1];
                x = (x | x << 8) & MASK_2D_U64[2];
                x = (x | x << 4) & MASK_2D_U64[3];
                x = (x | x << 2) & MASK_2D_U64[4];
                x = (x | x << 1) & MASK_2D_U64[5];
                MortonIndex::from_u64(x)
            }
            _ => MortonIndex::from_u32(split_by_second_bits(x.to_u32())),
        }
    }

    #[inline]
    fn get_second_bits<Coordinate: Word, MortonIndex: Word>(x: MortonIndex) -> Coordinate {
        match MortonIndex::bit_size().to_u8() {
            32 => {
                let mut x = x.to_u32();
                x &= MASK_2D_U32[5];
                x = (x ^ (x >> 1)) & MASK_2D_U32[4];
                x = (x ^ (x >> 2)) & MASK_2D_U32[3];
                x = (x ^ (x >> 4)) & MASK_2D_U32[2];
                x = (x ^ (x >> 8)) & MASK_2D_U32[1];
                x = (x ^ (x >> 16)) & MASK_2D_U32[0];
                Coordinate::from_u32(x)
            }
            64 => {
                let mut x = x.to_u64();
                x &= MASK_2D_U64[5];
                x = (x ^ (x >> 1)) & MASK_2D_U64[4];
                x = (x ^ (x >> 2)) & MASK_2D_U64[3];
                x = (x ^ (x >> 4)) & MASK_2D_U64[2];
                x = (x ^ (x >> 8)) & MASK_2D_U64[1];
                x = (x ^ (x >> 16)) & MASK_2D_U64[0];
                Coordinate::from_u64(x)
            }
            _ => Coordinate::from_u32(get_second_bits(x.to_u32())),
        }
    }

    #[inline]
    pub fn encode_2d<MortonIndex: Word, Coordinate: Word>(x: Coordinate,
                                                          y: Coordinate)
                                                          -> MortonIndex {
        split_by_second_bits::<MortonIndex, Coordinate>(x) |
        (split_by_second_bits::<MortonIndex, Coordinate>(y) << MortonIndex::one())
    }

    #[inline]
    pub fn decode_2d<Coordinate: Word, MortonIndex: Word>(v: MortonIndex)
                                                          -> (Coordinate, Coordinate) {
        (get_second_bits(v), get_second_bits(v >> MortonIndex::one()))
    }

    #[inline]
    fn split_by_third_bits<MortonIndex: Word, Coordinate: Word>(x: Coordinate) -> MortonIndex {
        match Coordinate::bit_size().to_u8() {
            32 => {
                let mut x = x.to_u32();
                x &= MASK_3D_U32[0];
                x = (x | x << 16) & MASK_3D_U32[1];
                x = (x | x << 8) & MASK_3D_U32[2];
                x = (x | x << 4) & MASK_3D_U32[3];
                x = (x | x << 2) & MASK_3D_U32[4];
                MortonIndex::from_u32(x)
            }
            64 => {
                let mut x = x.to_u64() & MASK_3D_U64[0];
                x = (x | x << 32) & MASK_3D_U64[1];
                x = (x | x << 16) & MASK_3D_U64[2];
                x = (x | x << 8) & MASK_3D_U64[3];
                x = (x | x << 4) & MASK_3D_U64[4];
                x = (x | x << 2) & MASK_3D_U64[5];
                MortonIndex::from_u64(x)
            }
            _ => MortonIndex::from_u32(split_by_third_bits(x.to_u32())),
        }
    }

    #[inline]
    fn get_third_bits<Coordinate: Word, MortonIndex: Word>(x: MortonIndex) -> Coordinate {
        match MortonIndex::bit_size().to_u8() {
            32 => {
                let mut x = x.to_u32();
                x &= MASK_3D_U32[4];
                x = (x ^ (x >> 2)) & MASK_3D_U32[3];
                x = (x ^ (x >> 4)) & MASK_3D_U32[2];
                x = (x ^ (x >> 8)) & MASK_3D_U32[1];
                x = (x ^ (x >> 16)) & MASK_3D_U32[0];
                Coordinate::from_u32(x)
            }
            64 => {
                let mut x = x.to_u64() & MASK_3D_U64[5];
                x = (x ^ (x >> 2)) & MASK_3D_U64[4];
                x = (x ^ (x >> 4)) & MASK_3D_U64[3];
                x = (x ^ (x >> 8)) & MASK_3D_U64[2];
                x = (x ^ ((x) >> 16)) & MASK_3D_U64[1];
                x = (x ^ ((x) >> 32)) & MASK_3D_U64[0];
                Coordinate::from_u64(x)
            }

            _ => Coordinate::from_u32(get_third_bits(x.to_u32())),
        }
    }

    #[inline]
    pub fn encode_3d<MortonIndex: Word, Coordinate: Word>(x: Coordinate,
                                                          y: Coordinate,
                                                          z: Coordinate)
                                                          -> MortonIndex {
        split_by_third_bits::<MortonIndex, Coordinate>(x) |
        (split_by_third_bits::<MortonIndex, Coordinate>(y) << MortonIndex::one()) |
        (split_by_third_bits::<MortonIndex, Coordinate>(z) << MortonIndex::from_u8(2))
    }

    #[inline]
    pub fn decode_3d<Coordinate: Word, MortonIndex: Word>
        (v: MortonIndex)
         -> (Coordinate, Coordinate, Coordinate) {
        (get_third_bits(v),
         get_third_bits(v >> MortonIndex::one()),
         get_third_bits(v >> MortonIndex::from_u8(2)))
    }
}

/// Encode coordinates `x` and `y` into an interleaved Morton index for a
/// z-Curve.
///
/// Using `_.i` to denote the `i`-th bit in a word, given the `x` and `y`
/// coordinates with the following bit patterns:
///
/// `x: |x.M|...|x.1|x.0|`
/// `y: |y.M|...|y.1|y.0|`
///
/// where `M == T::bit_size()`,
///
/// this function encodes the bits of `x` and `y` into a value `v` using the
/// Morton z-Curve encoding:
///
/// `v = |y.N|x.N|...|y.1|x.1|y.0|x.0|`
///
/// where `N == T::bit_size() / 2`.
///
/// # Example
/// ```
/// use bitwise::word::morton;
///
/// let x = 0b0000_1010u8;
/// let y = 0b0000_0011u8;
/// let r = 0b01_00_11_10u8;
/// assert_eq!(morton::encode_2d(x, y), r);
/// assert_eq!(morton::encode_2d(x as u32, y as u32), r as u32);
/// assert_eq!(morton::encode_2d(x as u64, y as u64), r as u64);
/// ```
#[inline]
pub fn encode_2d<T: Word>(x: T, y: T) -> T {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        if cfg!(target_feature = "bmi2") {
            bmi2::encode_2d(x, y)
        } else {
            // magic::encode_2d(x, y)
            lut::encode_2d(x, y)
        }
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        // magic::encode_2d(x, y)
        lut::encode_2d(x, y)
    }
}

#[inline]
fn encode_hb_3d<T: Word>(v: T, x: T, y: T, _: T) -> T {
    let mut v = v;
    match T::bit_size().to_u8() {
        32 => {
            v = word::copy_bit(x, 10u8, v, 30u8);
            v = word::copy_bit(y, 10u8, v, 31u8);
        }
        64 => {
            v = word::copy_bit(x, 21u8, v, 63u8);
        }
        _ => {}
    };
    v
}

/// Encode coordinates `x`, `y`, and `z` into an interleaved Morton index for a
/// z-Curve.
///
/// Using `_.i` to denote the `i`-th bit in a word, given the `x`, `y`, and `z`
/// coordinates with the following bit patterns:
///
/// `x: |x.M|...|x.1|x.0|`
/// `y: |y.M|...|y.1|y.0|`
/// `z: |z.M|...|z.1|z.0|`
///
/// where `M == T::bit_size()`,
///
/// this function encodes the bits of `x`, `y`, and `z` into a value `v` using
/// the Morton z-Curve encoding:
///
/// `v = |z.N|y.N|x.N|...|z.1|y.1|x.1|z.0|y.0|x.0|`
///
/// where `N == T::bit_size() / 3`.
///
/// Note: the encoded Morton index is always invertible to the coordinates using
/// the `encode_3d` function.
///
/// # Example
/// ```
/// use bitwise::word::morton;
///
/// let x = 0b0000_1010u8;
/// let y = 0b0000_0011u8;
/// let z = 0b0000_1110u8;
/// let r = 0b00_111_010u8;
/// let r32 = 0b101_100_111_010u32;
/// assert_eq!(morton::encode_3d(x, y, z), r);
/// assert_eq!(morton::encode_3d(x as u32, y as u32, z as u32), r32);
/// ```
#[inline]
pub fn encode_3d<T: Word>(x: T, y: T, z: T) -> T {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        if cfg!(target_feature = "bmi2") {
            bmi2::encode_3d(x, y, z)
        } else {
            let v = magic::encode_3d(x, y, z);
            encode_hb_3d(v, x, y, z)
        }
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        let v = magic::encode_3d(x, y, z);
        encode_hb_3d(v, x, y, z)
    }
}

/// Decodes an interleaved Morton index for a Z-Curve into two-dimensional
/// coordinates.
///
/// Using `_.i` to denote the `i`-th bit in a word, this function decodes
/// the following bit pattern of `v`:
///
/// `v = |y.N|x.N|...|y.1|x.1|y.0|x.0|`
///
/// where `N == T::bit_size() / 2`,
///
/// into the coordinates `x` and `y` with the following bit pattern:
///
/// `x: |...0|x.N|...|x.1|x.0|`
/// `y: |...0|y.N|...|y.1|y.0|`
///
/// # Example
/// ```
/// use bitwise::word::morton;
///
/// let x = 0b0000_1010u8;
/// let y = 0b0000_0011u8;
/// let r = 0b01_00_11_10u8;
/// assert_eq!(morton::decode_2d(r), (x, y));
/// ```
#[inline]
pub fn decode_2d<T: Word>(v: T) -> (T, T) {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        if cfg!(target_feature = "bmi2") {
            bmi2::decode_2d(v)
        } else {
            magic::decode_2d(v)
            //lut::decode_2d(v)
        }
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        magic::decode_2d(v)
        //lut::decode_2d(v)
    }
}

#[inline]
fn decode_hb_3d<T: Word>(v: T, x: T, y: T, z: T) -> (T, T, T) {
    let mut x = x;
    let mut y = y;
    match T::bit_size().to_u8() {
        32 => {
            x = word::copy_bit(v, 30u8, x, 10u8);
            y = word::copy_bit(v, 31u8, y, 10u8);
        }
        64 => {
            x = word::copy_bit(v, 63u8, x, 21u8);
        }
        _ => {}
    };
    (x, y, z)
}

/// Decodes an interleaved Morton index for a Z-Curve into three-dimensional
/// coordinates.
///
/// Using `_.i` to denote the `i`-th bit in a word, this function decodes
/// the following bit pattern of `v`:
///
/// `v = |z.N|y.N|x.N|...|z.1|y.1|x.1|z.0|y.0|x.0|`
///
/// where `N == T::bit_size() / 3`,
///
/// into the coordinates `x`, `y`, and `z` with the following bit pattern:
///
/// `x: ...|x.N|...|x.1|x.0|`
/// `y: ...|y.N|...|y.1|y.0|`
/// `y: ...|y.N|...|y.1|y.0|`
///
/// Note: the decoded coordinates are always invertible to the morton code using
/// the `decode_3d` function.
///
/// # Example
/// ```
/// use bitwise::word::morton;
///
/// let x = 0b0000_0010u8;
/// let y = 0b0000_0011u8;
/// let z = 0b0000_0010u8;
/// let r = 0b00_111_010u8;
/// assert_eq!(morton::decode_3d(r), (x, y, z));
/// ```
#[inline]
pub fn decode_3d<T: Word>(v: T) -> (T, T, T) {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        if cfg!(target_feature = "bmi2") {
            bmi2::decode_3d(v)
        } else {
            let (x, y, z) = magic::decode_3d(v);
            decode_hb_3d(v, x, y, z)
        }
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        let (x, y, z) = magic::decode_3d(v);
        decode_hb_3d(v, x, y, z)
    }
}

#[inline]
pub fn encode<T: Word>(v: &[T]) -> T {
    match v.len() {
        2 => encode_2d(v[0], v[1]),
        3 => encode_3d(v[0], v[1], v[2]),
        _ => unreachable!(),
    }
}

#[inline]
pub fn decode<T: Word>(v: T, r: &mut [T]) {
    // TODO: use unsafe { get_unchecked }
    match r.len() {
        2 => {
            let (x, y) = decode_2d(v);
            r[0] = x;
            r[1] = y;
        }
        3 => {
            let (x, y, z) = decode_3d(v);
            r[0] = x;
            r[1] = y;
            r[2] = z;
        }
        _ => unreachable!(),
    }
}

// These are reused from the benches. TODO: make them optional.
pub mod testing_utils {
    use super::*;
    use std::fmt::Debug;

    pub trait RunnerFn {
        fn run<T: Word + Debug>(&self, T);
    }

    pub struct Runner;
    impl Runner {
        pub fn run_u8<T: RunnerFn>(i: &T) {
            (0u8..u8::max_value()).map(|v| i.run(v)).count();
        }
        pub fn run_u16<T: RunnerFn>(i: &T) {
            (0u16..u16::max_value()).map(|v| i.run(v)).count();
        }
        pub fn run_u32<T: RunnerFn>(i: &T) {
            let t0_min = u16::max_value() as u32;
            let t0_max = t0_min + 1000000;
            (t0_min..t0_max).map(|v| i.run(v)).count();
            let t1_max = u32::max_value();
            let t1_min = t1_max - 1000000;
            (t1_min..t1_max).map(|v| i.run(v)).count();
        }

        pub fn run_u64<T: RunnerFn>(i: &T) {
            let t0_min = u32::max_value() as u64;
            let t0_max = t0_min + 1000000;
            (t0_min..t0_max).map(|v| i.run(v)).count();
            let t1_max = u64::max_value();
            let t1_min = t1_max - 1000000;
            (t1_min..t1_max).map(|v| i.run(v)).count();

        }

        pub fn run<T: RunnerFn>(i: T) {
            Runner::run_u8(&i);
            Runner::run_u16(&i);
            Runner::run_u32(&i);
            Runner::run_u64(&i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::testing_utils::*;
    use std::fmt::Debug;

    struct BMI2Invariant;

    impl RunnerFn for BMI2Invariant {
        fn run<T: Word + Debug>(&self, v: T) {
            {
                // 2D:
                let (x, y) = bmi2::decode_2d::<T, _>(v);
                let vs = bmi2::encode_2d::<T, _>(x, y);
                assert_eq!(vs, v);
            }
            {
                // 3D:
                let (x, y, z) = bmi2::decode_3d::<T, _>(v);
                let vs = bmi2::encode_3d::<T, _>(x, y, z);
                assert_eq!(vs, v);
            }
        }
    }

    #[test]
    fn bmi2_invariant() {
        Runner::run(BMI2Invariant {});
    }

    struct MagicInvariant;

    impl RunnerFn for MagicInvariant {
        fn run<T: Word + Debug>(&self, v: T) {
            {
                // 2D:
                let (x, y) = magic::decode_2d::<T, _>(v);
                let vs = magic::encode_2d::<T, _>(x, y);
                let (x2, y2) = magic::decode_2d::<T, _>(vs);
                let vs3 = magic::encode_2d::<T, _>(x2, y2);
                assert_eq!(vs3, vs);
            }
            {
                let (x, y, z) = magic::decode_3d::<T, _>(v);
                let vs = magic::encode_3d::<T, _>(x, y, z);
                let (x2, y2, z2) = magic::decode_3d::<T, _>(vs);
                let vs3 = magic::encode_3d::<T, _>(x2, y2, z2);
                assert_eq!(vs3, vs);
            }

        }
    }


    #[test]
    fn magic_invariant() {
        Runner::run(MagicInvariant {});
    }

    struct LUTInvariant;

    impl RunnerFn for LUTInvariant {
        fn run<T: Word + Debug>(&self, v: T) {
            {
                // 2D:
                let (x, y) = lut::decode_2d::<T, _>(v);
                let vs = lut::encode_2d::<T, _>(x, y);
                let (x2, y2) = lut::decode_2d::<T, _>(vs);
                let vs3 = lut::encode_2d::<T, _>(x2, y2);
                assert_eq!(vs3, vs);
            }
            {
                let (x, y, z) = lut::decode_3d::<T, _>(v);
                let vs = lut::encode_3d::<T, _>(x, y, z);
                let (x2, y2, z2) = lut::decode_3d::<T, _>(vs);
                let vs3 = lut::encode_3d::<T, _>(x2, y2, z2);
                assert_eq!(vs3, vs);
            }
        }
    }

    #[test]
    fn lut_invariant() {
        Runner::run(LUTInvariant {});
    }

    struct CmpImpls;

    impl RunnerFn for CmpImpls {
        fn run<T: Word + Debug>(&self, v: T) {
            use word::clear_bits_geq::ClearBitsGeq;
            {
                // 2D:
                let (x_bmi2, y_bmi2) = bmi2::decode_2d::<T, _>(v);

                let (x_m, y_m) = decode_2d::<T>(v);
                assert_eq!(x_bmi2, x_m);
                assert_eq!(y_bmi2, y_m);

                let v_bmi2: T = bmi2::encode_2d(x_bmi2, y_bmi2);
                assert_eq!(v_bmi2, v);

                let v_m: T = encode_2d(x_bmi2, y_bmi2);
                assert_eq!(v_m, v);

                let (x_mb, y_mb) = magic::decode_2d::<T, _>(v);

                assert_eq!(x_bmi2, x_mb);
                assert_eq!(y_bmi2, y_mb);

                let v_mb = magic::encode_2d::<T, _>(x_bmi2, y_bmi2);
                assert_eq!(v_mb, v);

                let (x_lut, y_lut) = lut::decode_2d(v);
                assert_eq!(x_bmi2, x_lut);
                assert_eq!(y_bmi2, y_lut);

                let v_lut: T = lut::encode_2d(x_bmi2, y_bmi2);
                assert_eq!(v_lut, v);
            }
            {
                // 3D

                {
                    //bmi2 and the public impls are always invertible
                    let (x_bmi2, y_bmi2, z_bmi2) = bmi2::decode_3d::<T, _>(v);
                    let (x_m, y_m, z_m) = decode_3d(v);

                    assert_eq!(x_bmi2, x_m);
                    assert_eq!(y_bmi2, y_m);
                    assert_eq!(z_bmi2, z_m);

                    let v_bmi2: T = bmi2::encode_3d(x_bmi2, y_bmi2, z_bmi2);
                    assert_eq!(v_bmi2, v);

                    let v_m: T = encode_3d(x_bmi2, y_bmi2, z_bmi2);
                    assert_eq!(v_m, v);
                }

                // note: BMI2 and LUT/MAGIC produce different results for 3D
                // because the last 2bits of a 32bit word and the last bit of a
                // 64bit word are not preserved in the decoding (such that
                // re-encoding produces a different key than the original value)
                let v = match T::bit_size().to_u8() {
                    32 => v.clear_bits_geq(30u8),
                    64 => v.clear_bits_geq(63u8),
                    _ => v,
                };

                let (x_bmi2, y_bmi2, z_bmi2) = bmi2::decode_3d::<T, _>(v);
                let (x_mb, y_mb, z_mb) = magic::decode_3d::<T, _>(v);

                assert_eq!(x_bmi2, x_mb);
                assert_eq!(y_bmi2, y_mb);
                assert_eq!(z_bmi2, z_mb);

                let v_mb = magic::encode_3d::<T, _>(x_bmi2, y_bmi2, z_bmi2);
                assert_eq!(v_mb, v);


                let (x_lut, y_lut, z_lut) = lut::decode_3d(v);
                assert_eq!(x_bmi2, x_lut);
                assert_eq!(y_bmi2, y_lut);
                assert_eq!(z_bmi2, z_lut);

                let v_lut: T = lut::encode_3d(x_bmi2, y_bmi2, z_bmi2);
                assert_eq!(v_lut, v);
            }

        }
    }

    #[test]
    fn cmp_impls() {
        Runner::run(CmpImpls {});
    }
}
