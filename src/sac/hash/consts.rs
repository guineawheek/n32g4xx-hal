pub(crate) const SHA1_K : [u32;4] = [
    0x9979825a,
    0xa1ebd96e,
    0xdcbc1b8f,
    0xd6c162ca
];

pub(crate) const SHA1_IV : [u32;5] = [
    0x01234567,
    0x89abcdef,
    0xfedcba98,
    0x76543210,
    0xf0e1d2c3
];

pub(crate) const SHA256_K : [u32;0x40] = [
    0x982f8a42,
    0x91443771,
    0xcffbc0b5,
    0xa5dbb5e9,
    0x5bc25639,
    0xf111f159,
    0xa4823f92,
    0xd55e1cab,
    0x98aa07d8,
    0x015b8312,
    0xbe853124,
    0xc37d0c55,
    0x745dbe72,
    0xfeb1de80,
    0xa706dc9b,
    0x74f19bc1,
    0xc1699be4,
    0x8647beef,
    0xc69dc10f,
    0xcca10c24,
    0x6f2ce92d,
    0xaa84744a,
    0xdca9b05c,
    0xda88f976,
    0x52513e98,
    0x6dc631a8,
    0xc82703b0,
    0xc77f59bf,
    0xf30be0c6,
    0x4791a7d5,
    0x5163ca06,
    0x67292914,
    0x850ab727,
    0x38211b2e,
    0xfc6d2c4d,
    0x130d3853,
    0x54730a65,
    0xbb0a6a76,
    0x2ec9c281,
    0x852c7292,
    0xa1e8bfa2,
    0x4b661aa8,
    0x708b4bc2,
    0xa3516cc7,
    0x19e892d1,
    0x240699d6,
    0x85350ef4,
    0x70a06a10,
    0x16c1a419,
    0x086c371e,
    0x4c774827,
    0xb5bcb034,
    0xb30c1c39,
    0x4aaad84e,
    0x4fca9c5b,
    0xf36f2e68,
    0xee828f74,
    0x6f63a578,
    0x1478c884,
    0x0802c78c,
    0xfaffbe90,
    0xeb6c50a4,
    0xf7a3f9be,
    0xf27871c6
];

pub(crate) const SHA224_IV : [u32;8] = [
    0xd89e05c1,
    0x07d57c36,
    0x17dd7030,
    0x39590ef7,
    0x310bc0ff,
    0x11155868,
    0xa78ff964,
    0xa44ffabe
];

pub(crate) const SHA256_IV : [u32;8] = [
    0x67e6096a,
    0x85ae67bb,
    0x72f36e3c,
    0x3af54fa5,
    0x7f520e51,
    0x8c68059b,
    0xabd9831f,
    0x19cde05b
];

pub(crate) const SM3_K : [u32;2] = [
    0x1945cc79,
    0x8a9d877a
];

pub(crate) const SM3_IV : [u32;8] = [
    0x6f168073,
	0xb9b21449,
	0xd7422417,
	0x00068ada,
	0xbc306fa9,
	0xaa383116,
	0x4dee8de3,
	0x4e0efbb0
];

pub(crate) const MD5_IV : [u32;4] = [
    0x67452301,
    0xefcdab89,
    0x98badcfe,
    0x10325476
];

pub(crate) const MD5_K : [u32;0x40] = [
    0xd76aa478,
	0xe8c7b756,
	0x242070db,
	0xc1bdceee,
	0xf57c0faf,
	0x4787c62a,
	0xa8304613,
	0xfd469501,
	0x698098d8,
	0x8b44f7af,
	0xffff5bb1,
	0x895cd7be,
	0x6b901122,
	0xfd987193,
	0xa679438e,
	0x49b40821,
	0xf61e2562,
	0xc040b340,
	0x265e5a51,
	0xe9b6c7aa,
	0xd62f105d,
	0x02441453,
	0xd8a1e681,
	0xe7d3fbc8,
	0x21e1cde6,
	0xc33707d6,
	0xf4d50d87,
	0x455a14ed,
	0xa9e3e905,
	0xfcefa3f8,
	0x676f02d9,
	0x8d2a4c8a,
	0xfffa3942,
	0x8771f681,
	0x6d9d6122,
	0xfde5380c,
	0xa4beea44,
	0x4bdecfa9,
	0xf6bb4b60,
	0xbebfbc70,
	0x289b7ec6,
	0xeaa127fa,
	0xd4ef3085,
	0x04881d05,
	0xd9d4d039,
	0xe6db99e5,
	0x1fa27cf8,
	0xc4ac5665,
	0xf4292244,
	0x432aff97,
	0xab9423a7,
	0xfc93a039,
	0x655b59c3,
	0x8f0ccc92,
	0xffeff47d,
	0x85845dd1,
	0x6fa87e4f,
	0xfe2ce6e0,
	0xa3014314,
	0x4e0811a1,
	0xf7537e82,
	0xbd3af235,
	0x2ad7d2bb,
	0xeb86d391
];

pub(crate) const MD5_S : [u32; 0x10] = [
    0x00000007,
	0x0000000c,
	0x00000011,
	0x00000016,
	0x00000005,
	0x00000009,
	0x0000000e,
	0x00000014,
	0x00000004,
	0x0000000b,
	0x00000010,
	0x00000017,
	0x00000006,
	0x0000000a,
	0x0000000f,
	0x00000015
];