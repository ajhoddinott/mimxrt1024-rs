#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 Register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer Register"]
    pub timer: TIMER,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    pub rxmgmask: RXMGMASK,
    #[doc = "0x14 - Rx Buffer 14 Mask Register"]
    pub rx14mask: RX14MASK,
    #[doc = "0x18 - Rx Buffer 15 Mask Register"]
    pub rx15mask: RX15MASK,
    #[doc = "0x1c - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x20 - Error and Status 1 Register"]
    pub esr1: ESR1,
    #[doc = "0x24 - Interrupt Masks 2 Register"]
    pub imask2: IMASK2,
    #[doc = "0x28 - Interrupt Masks 1 Register"]
    pub imask1: IMASK1,
    #[doc = "0x2c - Interrupt Flags 2 Register"]
    pub iflag2: IFLAG2,
    #[doc = "0x30 - Interrupt Flags 1 Register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 Register"]
    pub esr2: ESR2,
    _reserved14: [u8; 0x08],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask Register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    _reserved17: [u8; 0x08],
    #[doc = "0x58 - Debug 1 register"]
    pub dbg1: DBG1,
    #[doc = "0x5c - Debug 2 register"]
    pub dbg2: DBG2,
    _reserved19: [u8; 0x20],
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    pub cs0: CS0,
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    pub id0: ID0,
    #[doc = "0x88 - Message Buffer 0 WORD0 Register"]
    pub word00: WORD00,
    #[doc = "0x8c - Message Buffer 0 WORD1 Register"]
    pub word10: WORD10,
    #[doc = "0x90 - Message Buffer 1 CS Register"]
    pub cs1: CS1,
    #[doc = "0x94 - Message Buffer 1 ID Register"]
    pub id1: ID1,
    #[doc = "0x98 - Message Buffer 1 WORD0 Register"]
    pub word01: WORD01,
    #[doc = "0x9c - Message Buffer 1 WORD1 Register"]
    pub word11: WORD11,
    #[doc = "0xa0 - Message Buffer 2 CS Register"]
    pub cs2: CS2,
    #[doc = "0xa4 - Message Buffer 2 ID Register"]
    pub id2: ID2,
    #[doc = "0xa8 - Message Buffer 2 WORD0 Register"]
    pub word02: WORD02,
    #[doc = "0xac - Message Buffer 2 WORD1 Register"]
    pub word12: WORD12,
    #[doc = "0xb0 - Message Buffer 3 CS Register"]
    pub cs3: CS3,
    #[doc = "0xb4 - Message Buffer 3 ID Register"]
    pub id3: ID3,
    #[doc = "0xb8 - Message Buffer 3 WORD0 Register"]
    pub word03: WORD03,
    #[doc = "0xbc - Message Buffer 3 WORD1 Register"]
    pub word13: WORD13,
    #[doc = "0xc0 - Message Buffer 4 CS Register"]
    pub cs4: CS4,
    #[doc = "0xc4 - Message Buffer 4 ID Register"]
    pub id4: ID4,
    #[doc = "0xc8 - Message Buffer 4 WORD0 Register"]
    pub word04: WORD04,
    #[doc = "0xcc - Message Buffer 4 WORD1 Register"]
    pub word14: WORD14,
    #[doc = "0xd0 - Message Buffer 5 CS Register"]
    pub cs5: CS5,
    #[doc = "0xd4 - Message Buffer 5 ID Register"]
    pub id5: ID5,
    #[doc = "0xd8 - Message Buffer 5 WORD0 Register"]
    pub word05: WORD05,
    #[doc = "0xdc - Message Buffer 5 WORD1 Register"]
    pub word15: WORD15,
    #[doc = "0xe0 - Message Buffer 6 CS Register"]
    pub cs6: CS6,
    #[doc = "0xe4 - Message Buffer 6 ID Register"]
    pub id6: ID6,
    #[doc = "0xe8 - Message Buffer 6 WORD0 Register"]
    pub word06: WORD06,
    #[doc = "0xec - Message Buffer 6 WORD1 Register"]
    pub word16: WORD16,
    #[doc = "0xf0 - Message Buffer 7 CS Register"]
    pub cs7: CS7,
    #[doc = "0xf4 - Message Buffer 7 ID Register"]
    pub id7: ID7,
    #[doc = "0xf8 - Message Buffer 7 WORD0 Register"]
    pub word07: WORD07,
    #[doc = "0xfc - Message Buffer 7 WORD1 Register"]
    pub word17: WORD17,
    #[doc = "0x100 - Message Buffer 8 CS Register"]
    pub cs8: CS8,
    #[doc = "0x104 - Message Buffer 8 ID Register"]
    pub id8: ID8,
    #[doc = "0x108 - Message Buffer 8 WORD0 Register"]
    pub word08: WORD08,
    #[doc = "0x10c - Message Buffer 8 WORD1 Register"]
    pub word18: WORD18,
    #[doc = "0x110 - Message Buffer 9 CS Register"]
    pub cs9: CS9,
    #[doc = "0x114 - Message Buffer 9 ID Register"]
    pub id9: ID9,
    #[doc = "0x118 - Message Buffer 9 WORD0 Register"]
    pub word09: WORD09,
    #[doc = "0x11c - Message Buffer 9 WORD1 Register"]
    pub word19: WORD19,
    #[doc = "0x120 - Message Buffer 10 CS Register"]
    pub cs10: CS10,
    #[doc = "0x124 - Message Buffer 10 ID Register"]
    pub id10: ID10,
    #[doc = "0x128 - Message Buffer 10 WORD0 Register"]
    pub word010: WORD010,
    #[doc = "0x12c - Message Buffer 10 WORD1 Register"]
    pub word110: WORD110,
    #[doc = "0x130 - Message Buffer 11 CS Register"]
    pub cs11: CS11,
    #[doc = "0x134 - Message Buffer 11 ID Register"]
    pub id11: ID11,
    #[doc = "0x138 - Message Buffer 11 WORD0 Register"]
    pub word011: WORD011,
    #[doc = "0x13c - Message Buffer 11 WORD1 Register"]
    pub word111: WORD111,
    #[doc = "0x140 - Message Buffer 12 CS Register"]
    pub cs12: CS12,
    #[doc = "0x144 - Message Buffer 12 ID Register"]
    pub id12: ID12,
    #[doc = "0x148 - Message Buffer 12 WORD0 Register"]
    pub word012: WORD012,
    #[doc = "0x14c - Message Buffer 12 WORD1 Register"]
    pub word112: WORD112,
    #[doc = "0x150 - Message Buffer 13 CS Register"]
    pub cs13: CS13,
    #[doc = "0x154 - Message Buffer 13 ID Register"]
    pub id13: ID13,
    #[doc = "0x158 - Message Buffer 13 WORD0 Register"]
    pub word013: WORD013,
    #[doc = "0x15c - Message Buffer 13 WORD1 Register"]
    pub word113: WORD113,
    #[doc = "0x160 - Message Buffer 14 CS Register"]
    pub cs14: CS14,
    #[doc = "0x164 - Message Buffer 14 ID Register"]
    pub id14: ID14,
    #[doc = "0x168 - Message Buffer 14 WORD0 Register"]
    pub word014: WORD014,
    #[doc = "0x16c - Message Buffer 14 WORD1 Register"]
    pub word114: WORD114,
    #[doc = "0x170 - Message Buffer 15 CS Register"]
    pub cs15: CS15,
    #[doc = "0x174 - Message Buffer 15 ID Register"]
    pub id15: ID15,
    #[doc = "0x178 - Message Buffer 15 WORD0 Register"]
    pub word015: WORD015,
    #[doc = "0x17c - Message Buffer 15 WORD1 Register"]
    pub word115: WORD115,
    #[doc = "0x180 - Message Buffer 16 CS Register"]
    pub cs16: CS16,
    #[doc = "0x184 - Message Buffer 16 ID Register"]
    pub id16: ID16,
    #[doc = "0x188 - Message Buffer 16 WORD0 Register"]
    pub word016: WORD016,
    #[doc = "0x18c - Message Buffer 16 WORD1 Register"]
    pub word116: WORD116,
    #[doc = "0x190 - Message Buffer 17 CS Register"]
    pub cs17: CS17,
    #[doc = "0x194 - Message Buffer 17 ID Register"]
    pub id17: ID17,
    #[doc = "0x198 - Message Buffer 17 WORD0 Register"]
    pub word017: WORD017,
    #[doc = "0x19c - Message Buffer 17 WORD1 Register"]
    pub word117: WORD117,
    #[doc = "0x1a0 - Message Buffer 18 CS Register"]
    pub cs18: CS18,
    #[doc = "0x1a4 - Message Buffer 18 ID Register"]
    pub id18: ID18,
    #[doc = "0x1a8 - Message Buffer 18 WORD0 Register"]
    pub word018: WORD018,
    #[doc = "0x1ac - Message Buffer 18 WORD1 Register"]
    pub word118: WORD118,
    #[doc = "0x1b0 - Message Buffer 19 CS Register"]
    pub cs19: CS19,
    #[doc = "0x1b4 - Message Buffer 19 ID Register"]
    pub id19: ID19,
    #[doc = "0x1b8 - Message Buffer 19 WORD0 Register"]
    pub word019: WORD019,
    #[doc = "0x1bc - Message Buffer 19 WORD1 Register"]
    pub word119: WORD119,
    #[doc = "0x1c0 - Message Buffer 20 CS Register"]
    pub cs20: CS20,
    #[doc = "0x1c4 - Message Buffer 20 ID Register"]
    pub id20: ID20,
    #[doc = "0x1c8 - Message Buffer 20 WORD0 Register"]
    pub word020: WORD020,
    #[doc = "0x1cc - Message Buffer 20 WORD1 Register"]
    pub word120: WORD120,
    #[doc = "0x1d0 - Message Buffer 21 CS Register"]
    pub cs21: CS21,
    #[doc = "0x1d4 - Message Buffer 21 ID Register"]
    pub id21: ID21,
    #[doc = "0x1d8 - Message Buffer 21 WORD0 Register"]
    pub word021: WORD021,
    #[doc = "0x1dc - Message Buffer 21 WORD1 Register"]
    pub word121: WORD121,
    #[doc = "0x1e0 - Message Buffer 22 CS Register"]
    pub cs22: CS22,
    #[doc = "0x1e4 - Message Buffer 22 ID Register"]
    pub id22: ID22,
    #[doc = "0x1e8 - Message Buffer 22 WORD0 Register"]
    pub word022: WORD022,
    #[doc = "0x1ec - Message Buffer 22 WORD1 Register"]
    pub word122: WORD122,
    #[doc = "0x1f0 - Message Buffer 23 CS Register"]
    pub cs23: CS23,
    #[doc = "0x1f4 - Message Buffer 23 ID Register"]
    pub id23: ID23,
    #[doc = "0x1f8 - Message Buffer 23 WORD0 Register"]
    pub word023: WORD023,
    #[doc = "0x1fc - Message Buffer 23 WORD1 Register"]
    pub word123: WORD123,
    #[doc = "0x200 - Message Buffer 24 CS Register"]
    pub cs24: CS24,
    #[doc = "0x204 - Message Buffer 24 ID Register"]
    pub id24: ID24,
    #[doc = "0x208 - Message Buffer 24 WORD0 Register"]
    pub word024: WORD024,
    #[doc = "0x20c - Message Buffer 24 WORD1 Register"]
    pub word124: WORD124,
    #[doc = "0x210 - Message Buffer 25 CS Register"]
    pub cs25: CS25,
    #[doc = "0x214 - Message Buffer 25 ID Register"]
    pub id25: ID25,
    #[doc = "0x218 - Message Buffer 25 WORD0 Register"]
    pub word025: WORD025,
    #[doc = "0x21c - Message Buffer 25 WORD1 Register"]
    pub word125: WORD125,
    #[doc = "0x220 - Message Buffer 26 CS Register"]
    pub cs26: CS26,
    #[doc = "0x224 - Message Buffer 26 ID Register"]
    pub id26: ID26,
    #[doc = "0x228 - Message Buffer 26 WORD0 Register"]
    pub word026: WORD026,
    #[doc = "0x22c - Message Buffer 26 WORD1 Register"]
    pub word126: WORD126,
    #[doc = "0x230 - Message Buffer 27 CS Register"]
    pub cs27: CS27,
    #[doc = "0x234 - Message Buffer 27 ID Register"]
    pub id27: ID27,
    #[doc = "0x238 - Message Buffer 27 WORD0 Register"]
    pub word027: WORD027,
    #[doc = "0x23c - Message Buffer 27 WORD1 Register"]
    pub word127: WORD127,
    #[doc = "0x240 - Message Buffer 28 CS Register"]
    pub cs28: CS28,
    #[doc = "0x244 - Message Buffer 28 ID Register"]
    pub id28: ID28,
    #[doc = "0x248 - Message Buffer 28 WORD0 Register"]
    pub word028: WORD028,
    #[doc = "0x24c - Message Buffer 28 WORD1 Register"]
    pub word128: WORD128,
    #[doc = "0x250 - Message Buffer 29 CS Register"]
    pub cs29: CS29,
    #[doc = "0x254 - Message Buffer 29 ID Register"]
    pub id29: ID29,
    #[doc = "0x258 - Message Buffer 29 WORD0 Register"]
    pub word029: WORD029,
    #[doc = "0x25c - Message Buffer 29 WORD1 Register"]
    pub word129: WORD129,
    #[doc = "0x260 - Message Buffer 30 CS Register"]
    pub cs30: CS30,
    #[doc = "0x264 - Message Buffer 30 ID Register"]
    pub id30: ID30,
    #[doc = "0x268 - Message Buffer 30 WORD0 Register"]
    pub word030: WORD030,
    #[doc = "0x26c - Message Buffer 30 WORD1 Register"]
    pub word130: WORD130,
    #[doc = "0x270 - Message Buffer 31 CS Register"]
    pub cs31: CS31,
    #[doc = "0x274 - Message Buffer 31 ID Register"]
    pub id31: ID31,
    #[doc = "0x278 - Message Buffer 31 WORD0 Register"]
    pub word031: WORD031,
    #[doc = "0x27c - Message Buffer 31 WORD1 Register"]
    pub word131: WORD131,
    #[doc = "0x280 - Message Buffer 32 CS Register"]
    pub cs32: CS32,
    #[doc = "0x284 - Message Buffer 32 ID Register"]
    pub id32: ID32,
    #[doc = "0x288 - Message Buffer 32 WORD0 Register"]
    pub word032: WORD032,
    #[doc = "0x28c - Message Buffer 32 WORD1 Register"]
    pub word132: WORD132,
    #[doc = "0x290 - Message Buffer 33 CS Register"]
    pub cs33: CS33,
    #[doc = "0x294 - Message Buffer 33 ID Register"]
    pub id33: ID33,
    #[doc = "0x298 - Message Buffer 33 WORD0 Register"]
    pub word033: WORD033,
    #[doc = "0x29c - Message Buffer 33 WORD1 Register"]
    pub word133: WORD133,
    #[doc = "0x2a0 - Message Buffer 34 CS Register"]
    pub cs34: CS34,
    #[doc = "0x2a4 - Message Buffer 34 ID Register"]
    pub id34: ID34,
    #[doc = "0x2a8 - Message Buffer 34 WORD0 Register"]
    pub word034: WORD034,
    #[doc = "0x2ac - Message Buffer 34 WORD1 Register"]
    pub word134: WORD134,
    #[doc = "0x2b0 - Message Buffer 35 CS Register"]
    pub cs35: CS35,
    #[doc = "0x2b4 - Message Buffer 35 ID Register"]
    pub id35: ID35,
    #[doc = "0x2b8 - Message Buffer 35 WORD0 Register"]
    pub word035: WORD035,
    #[doc = "0x2bc - Message Buffer 35 WORD1 Register"]
    pub word135: WORD135,
    #[doc = "0x2c0 - Message Buffer 36 CS Register"]
    pub cs36: CS36,
    #[doc = "0x2c4 - Message Buffer 36 ID Register"]
    pub id36: ID36,
    #[doc = "0x2c8 - Message Buffer 36 WORD0 Register"]
    pub word036: WORD036,
    #[doc = "0x2cc - Message Buffer 36 WORD1 Register"]
    pub word136: WORD136,
    #[doc = "0x2d0 - Message Buffer 37 CS Register"]
    pub cs37: CS37,
    #[doc = "0x2d4 - Message Buffer 37 ID Register"]
    pub id37: ID37,
    #[doc = "0x2d8 - Message Buffer 37 WORD0 Register"]
    pub word037: WORD037,
    #[doc = "0x2dc - Message Buffer 37 WORD1 Register"]
    pub word137: WORD137,
    #[doc = "0x2e0 - Message Buffer 38 CS Register"]
    pub cs38: CS38,
    #[doc = "0x2e4 - Message Buffer 38 ID Register"]
    pub id38: ID38,
    #[doc = "0x2e8 - Message Buffer 38 WORD0 Register"]
    pub word038: WORD038,
    #[doc = "0x2ec - Message Buffer 38 WORD1 Register"]
    pub word138: WORD138,
    #[doc = "0x2f0 - Message Buffer 39 CS Register"]
    pub cs39: CS39,
    #[doc = "0x2f4 - Message Buffer 39 ID Register"]
    pub id39: ID39,
    #[doc = "0x2f8 - Message Buffer 39 WORD0 Register"]
    pub word039: WORD039,
    #[doc = "0x2fc - Message Buffer 39 WORD1 Register"]
    pub word139: WORD139,
    #[doc = "0x300 - Message Buffer 40 CS Register"]
    pub cs40: CS40,
    #[doc = "0x304 - Message Buffer 40 ID Register"]
    pub id40: ID40,
    #[doc = "0x308 - Message Buffer 40 WORD0 Register"]
    pub word040: WORD040,
    #[doc = "0x30c - Message Buffer 40 WORD1 Register"]
    pub word140: WORD140,
    #[doc = "0x310 - Message Buffer 41 CS Register"]
    pub cs41: CS41,
    #[doc = "0x314 - Message Buffer 41 ID Register"]
    pub id41: ID41,
    #[doc = "0x318 - Message Buffer 41 WORD0 Register"]
    pub word041: WORD041,
    #[doc = "0x31c - Message Buffer 41 WORD1 Register"]
    pub word141: WORD141,
    #[doc = "0x320 - Message Buffer 42 CS Register"]
    pub cs42: CS42,
    #[doc = "0x324 - Message Buffer 42 ID Register"]
    pub id42: ID42,
    #[doc = "0x328 - Message Buffer 42 WORD0 Register"]
    pub word042: WORD042,
    #[doc = "0x32c - Message Buffer 42 WORD1 Register"]
    pub word142: WORD142,
    #[doc = "0x330 - Message Buffer 43 CS Register"]
    pub cs43: CS43,
    #[doc = "0x334 - Message Buffer 43 ID Register"]
    pub id43: ID43,
    #[doc = "0x338 - Message Buffer 43 WORD0 Register"]
    pub word043: WORD043,
    #[doc = "0x33c - Message Buffer 43 WORD1 Register"]
    pub word143: WORD143,
    #[doc = "0x340 - Message Buffer 44 CS Register"]
    pub cs44: CS44,
    #[doc = "0x344 - Message Buffer 44 ID Register"]
    pub id44: ID44,
    #[doc = "0x348 - Message Buffer 44 WORD0 Register"]
    pub word044: WORD044,
    #[doc = "0x34c - Message Buffer 44 WORD1 Register"]
    pub word144: WORD144,
    #[doc = "0x350 - Message Buffer 45 CS Register"]
    pub cs45: CS45,
    #[doc = "0x354 - Message Buffer 45 ID Register"]
    pub id45: ID45,
    #[doc = "0x358 - Message Buffer 45 WORD0 Register"]
    pub word045: WORD045,
    #[doc = "0x35c - Message Buffer 45 WORD1 Register"]
    pub word145: WORD145,
    #[doc = "0x360 - Message Buffer 46 CS Register"]
    pub cs46: CS46,
    #[doc = "0x364 - Message Buffer 46 ID Register"]
    pub id46: ID46,
    #[doc = "0x368 - Message Buffer 46 WORD0 Register"]
    pub word046: WORD046,
    #[doc = "0x36c - Message Buffer 46 WORD1 Register"]
    pub word146: WORD146,
    #[doc = "0x370 - Message Buffer 47 CS Register"]
    pub cs47: CS47,
    #[doc = "0x374 - Message Buffer 47 ID Register"]
    pub id47: ID47,
    #[doc = "0x378 - Message Buffer 47 WORD0 Register"]
    pub word047: WORD047,
    #[doc = "0x37c - Message Buffer 47 WORD1 Register"]
    pub word147: WORD147,
    #[doc = "0x380 - Message Buffer 48 CS Register"]
    pub cs48: CS48,
    #[doc = "0x384 - Message Buffer 48 ID Register"]
    pub id48: ID48,
    #[doc = "0x388 - Message Buffer 48 WORD0 Register"]
    pub word048: WORD048,
    #[doc = "0x38c - Message Buffer 48 WORD1 Register"]
    pub word148: WORD148,
    #[doc = "0x390 - Message Buffer 49 CS Register"]
    pub cs49: CS49,
    #[doc = "0x394 - Message Buffer 49 ID Register"]
    pub id49: ID49,
    #[doc = "0x398 - Message Buffer 49 WORD0 Register"]
    pub word049: WORD049,
    #[doc = "0x39c - Message Buffer 49 WORD1 Register"]
    pub word149: WORD149,
    #[doc = "0x3a0 - Message Buffer 50 CS Register"]
    pub cs50: CS50,
    #[doc = "0x3a4 - Message Buffer 50 ID Register"]
    pub id50: ID50,
    #[doc = "0x3a8 - Message Buffer 50 WORD0 Register"]
    pub word050: WORD050,
    #[doc = "0x3ac - Message Buffer 50 WORD1 Register"]
    pub word150: WORD150,
    #[doc = "0x3b0 - Message Buffer 51 CS Register"]
    pub cs51: CS51,
    #[doc = "0x3b4 - Message Buffer 51 ID Register"]
    pub id51: ID51,
    #[doc = "0x3b8 - Message Buffer 51 WORD0 Register"]
    pub word051: WORD051,
    #[doc = "0x3bc - Message Buffer 51 WORD1 Register"]
    pub word151: WORD151,
    #[doc = "0x3c0 - Message Buffer 52 CS Register"]
    pub cs52: CS52,
    #[doc = "0x3c4 - Message Buffer 52 ID Register"]
    pub id52: ID52,
    #[doc = "0x3c8 - Message Buffer 52 WORD0 Register"]
    pub word052: WORD052,
    #[doc = "0x3cc - Message Buffer 52 WORD1 Register"]
    pub word152: WORD152,
    #[doc = "0x3d0 - Message Buffer 53 CS Register"]
    pub cs53: CS53,
    #[doc = "0x3d4 - Message Buffer 53 ID Register"]
    pub id53: ID53,
    #[doc = "0x3d8 - Message Buffer 53 WORD0 Register"]
    pub word053: WORD053,
    #[doc = "0x3dc - Message Buffer 53 WORD1 Register"]
    pub word153: WORD153,
    #[doc = "0x3e0 - Message Buffer 54 CS Register"]
    pub cs54: CS54,
    #[doc = "0x3e4 - Message Buffer 54 ID Register"]
    pub id54: ID54,
    #[doc = "0x3e8 - Message Buffer 54 WORD0 Register"]
    pub word054: WORD054,
    #[doc = "0x3ec - Message Buffer 54 WORD1 Register"]
    pub word154: WORD154,
    #[doc = "0x3f0 - Message Buffer 55 CS Register"]
    pub cs55: CS55,
    #[doc = "0x3f4 - Message Buffer 55 ID Register"]
    pub id55: ID55,
    #[doc = "0x3f8 - Message Buffer 55 WORD0 Register"]
    pub word055: WORD055,
    #[doc = "0x3fc - Message Buffer 55 WORD1 Register"]
    pub word155: WORD155,
    #[doc = "0x400 - Message Buffer 56 CS Register"]
    pub cs56: CS56,
    #[doc = "0x404 - Message Buffer 56 ID Register"]
    pub id56: ID56,
    #[doc = "0x408 - Message Buffer 56 WORD0 Register"]
    pub word056: WORD056,
    #[doc = "0x40c - Message Buffer 56 WORD1 Register"]
    pub word156: WORD156,
    #[doc = "0x410 - Message Buffer 57 CS Register"]
    pub cs57: CS57,
    #[doc = "0x414 - Message Buffer 57 ID Register"]
    pub id57: ID57,
    #[doc = "0x418 - Message Buffer 57 WORD0 Register"]
    pub word057: WORD057,
    #[doc = "0x41c - Message Buffer 57 WORD1 Register"]
    pub word157: WORD157,
    #[doc = "0x420 - Message Buffer 58 CS Register"]
    pub cs58: CS58,
    #[doc = "0x424 - Message Buffer 58 ID Register"]
    pub id58: ID58,
    #[doc = "0x428 - Message Buffer 58 WORD0 Register"]
    pub word058: WORD058,
    #[doc = "0x42c - Message Buffer 58 WORD1 Register"]
    pub word158: WORD158,
    #[doc = "0x430 - Message Buffer 59 CS Register"]
    pub cs59: CS59,
    #[doc = "0x434 - Message Buffer 59 ID Register"]
    pub id59: ID59,
    #[doc = "0x438 - Message Buffer 59 WORD0 Register"]
    pub word059: WORD059,
    #[doc = "0x43c - Message Buffer 59 WORD1 Register"]
    pub word159: WORD159,
    #[doc = "0x440 - Message Buffer 60 CS Register"]
    pub cs60: CS60,
    #[doc = "0x444 - Message Buffer 60 ID Register"]
    pub id60: ID60,
    #[doc = "0x448 - Message Buffer 60 WORD0 Register"]
    pub word060: WORD060,
    #[doc = "0x44c - Message Buffer 60 WORD1 Register"]
    pub word160: WORD160,
    #[doc = "0x450 - Message Buffer 61 CS Register"]
    pub cs61: CS61,
    #[doc = "0x454 - Message Buffer 61 ID Register"]
    pub id61: ID61,
    #[doc = "0x458 - Message Buffer 61 WORD0 Register"]
    pub word061: WORD061,
    #[doc = "0x45c - Message Buffer 61 WORD1 Register"]
    pub word161: WORD161,
    #[doc = "0x460 - Message Buffer 62 CS Register"]
    pub cs62: CS62,
    #[doc = "0x464 - Message Buffer 62 ID Register"]
    pub id62: ID62,
    #[doc = "0x468 - Message Buffer 62 WORD0 Register"]
    pub word062: WORD062,
    #[doc = "0x46c - Message Buffer 62 WORD1 Register"]
    pub word162: WORD162,
    #[doc = "0x470 - Message Buffer 63 CS Register"]
    pub cs63: CS63,
    #[doc = "0x474 - Message Buffer 63 ID Register"]
    pub id63: ID63,
    #[doc = "0x478 - Message Buffer 63 WORD0 Register"]
    pub word063: WORD063,
    #[doc = "0x47c - Message Buffer 63 WORD1 Register"]
    pub word163: WORD163,
    _reserved275: [u8; 0x0400],
    #[doc = "0x880..0x980 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 64],
    _reserved276: [u8; 0x60],
    #[doc = "0x9e0 - Glitch Filter Width Registers"]
    pub gfwr: GFWR,
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control 1 Register"]
pub mod ctrl1;
#[doc = "TIMER (rw) register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Free Running Timer Register"]
pub mod timer;
#[doc = "RXMGMASK (rw) register accessor: an alias for `Reg<RXMGMASK_SPEC>`"]
pub type RXMGMASK = crate::Reg<rxmgmask::RXMGMASK_SPEC>;
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "RX14MASK (rw) register accessor: an alias for `Reg<RX14MASK_SPEC>`"]
pub type RX14MASK = crate::Reg<rx14mask::RX14MASK_SPEC>;
#[doc = "Rx Buffer 14 Mask Register"]
pub mod rx14mask;
#[doc = "RX15MASK (rw) register accessor: an alias for `Reg<RX15MASK_SPEC>`"]
pub type RX15MASK = crate::Reg<rx15mask::RX15MASK_SPEC>;
#[doc = "Rx Buffer 15 Mask Register"]
pub mod rx15mask;
#[doc = "ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "ESR1 (rw) register accessor: an alias for `Reg<ESR1_SPEC>`"]
pub type ESR1 = crate::Reg<esr1::ESR1_SPEC>;
#[doc = "Error and Status 1 Register"]
pub mod esr1;
#[doc = "IMASK2 (rw) register accessor: an alias for `Reg<IMASK2_SPEC>`"]
pub type IMASK2 = crate::Reg<imask2::IMASK2_SPEC>;
#[doc = "Interrupt Masks 2 Register"]
pub mod imask2;
#[doc = "IMASK1 (rw) register accessor: an alias for `Reg<IMASK1_SPEC>`"]
pub type IMASK1 = crate::Reg<imask1::IMASK1_SPEC>;
#[doc = "Interrupt Masks 1 Register"]
pub mod imask1;
#[doc = "IFLAG2 (rw) register accessor: an alias for `Reg<IFLAG2_SPEC>`"]
pub type IFLAG2 = crate::Reg<iflag2::IFLAG2_SPEC>;
#[doc = "Interrupt Flags 2 Register"]
pub mod iflag2;
#[doc = "IFLAG1 (rw) register accessor: an alias for `Reg<IFLAG1_SPEC>`"]
pub type IFLAG1 = crate::Reg<iflag1::IFLAG1_SPEC>;
#[doc = "Interrupt Flags 1 Register"]
pub mod iflag1;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 Register"]
pub mod ctrl2;
#[doc = "ESR2 (r) register accessor: an alias for `Reg<ESR2_SPEC>`"]
pub type ESR2 = crate::Reg<esr2::ESR2_SPEC>;
#[doc = "Error and Status 2 Register"]
pub mod esr2;
#[doc = "CRCR (r) register accessor: an alias for `Reg<CRCR_SPEC>`"]
pub type CRCR = crate::Reg<crcr::CRCR_SPEC>;
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "RXFGMASK (rw) register accessor: an alias for `Reg<RXFGMASK_SPEC>`"]
pub type RXFGMASK = crate::Reg<rxfgmask::RXFGMASK_SPEC>;
#[doc = "Rx FIFO Global Mask Register"]
pub mod rxfgmask;
#[doc = "RXFIR (r) register accessor: an alias for `Reg<RXFIR_SPEC>`"]
pub type RXFIR = crate::Reg<rxfir::RXFIR_SPEC>;
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "DBG1 (r) register accessor: an alias for `Reg<DBG1_SPEC>`"]
pub type DBG1 = crate::Reg<dbg1::DBG1_SPEC>;
#[doc = "Debug 1 register"]
pub mod dbg1;
#[doc = "DBG2 (r) register accessor: an alias for `Reg<DBG2_SPEC>`"]
pub type DBG2 = crate::Reg<dbg2::DBG2_SPEC>;
#[doc = "Debug 2 register"]
pub mod dbg2;
#[doc = "CS0 (rw) register accessor: an alias for `Reg<CS0_SPEC>`"]
pub type CS0 = crate::Reg<cs0::CS0_SPEC>;
#[doc = "Message Buffer 0 CS Register"]
pub mod cs0;
#[doc = "ID0 (rw) register accessor: an alias for `Reg<ID0_SPEC>`"]
pub type ID0 = crate::Reg<id0::ID0_SPEC>;
#[doc = "Message Buffer 0 ID Register"]
pub mod id0;
#[doc = "WORD00 (rw) register accessor: an alias for `Reg<WORD00_SPEC>`"]
pub type WORD00 = crate::Reg<word00::WORD00_SPEC>;
#[doc = "Message Buffer 0 WORD0 Register"]
pub mod word00;
#[doc = "WORD10 (rw) register accessor: an alias for `Reg<WORD10_SPEC>`"]
pub type WORD10 = crate::Reg<word10::WORD10_SPEC>;
#[doc = "Message Buffer 0 WORD1 Register"]
pub mod word10;
#[doc = "CS1 (rw) register accessor: an alias for `Reg<CS1_SPEC>`"]
pub type CS1 = crate::Reg<cs1::CS1_SPEC>;
#[doc = "Message Buffer 1 CS Register"]
pub mod cs1;
#[doc = "ID1 (rw) register accessor: an alias for `Reg<ID1_SPEC>`"]
pub type ID1 = crate::Reg<id1::ID1_SPEC>;
#[doc = "Message Buffer 1 ID Register"]
pub mod id1;
#[doc = "WORD01 (rw) register accessor: an alias for `Reg<WORD01_SPEC>`"]
pub type WORD01 = crate::Reg<word01::WORD01_SPEC>;
#[doc = "Message Buffer 1 WORD0 Register"]
pub mod word01;
#[doc = "WORD11 (rw) register accessor: an alias for `Reg<WORD11_SPEC>`"]
pub type WORD11 = crate::Reg<word11::WORD11_SPEC>;
#[doc = "Message Buffer 1 WORD1 Register"]
pub mod word11;
#[doc = "CS2 (rw) register accessor: an alias for `Reg<CS2_SPEC>`"]
pub type CS2 = crate::Reg<cs2::CS2_SPEC>;
#[doc = "Message Buffer 2 CS Register"]
pub mod cs2;
#[doc = "ID2 (rw) register accessor: an alias for `Reg<ID2_SPEC>`"]
pub type ID2 = crate::Reg<id2::ID2_SPEC>;
#[doc = "Message Buffer 2 ID Register"]
pub mod id2;
#[doc = "WORD02 (rw) register accessor: an alias for `Reg<WORD02_SPEC>`"]
pub type WORD02 = crate::Reg<word02::WORD02_SPEC>;
#[doc = "Message Buffer 2 WORD0 Register"]
pub mod word02;
#[doc = "WORD12 (rw) register accessor: an alias for `Reg<WORD12_SPEC>`"]
pub type WORD12 = crate::Reg<word12::WORD12_SPEC>;
#[doc = "Message Buffer 2 WORD1 Register"]
pub mod word12;
#[doc = "CS3 (rw) register accessor: an alias for `Reg<CS3_SPEC>`"]
pub type CS3 = crate::Reg<cs3::CS3_SPEC>;
#[doc = "Message Buffer 3 CS Register"]
pub mod cs3;
#[doc = "ID3 (rw) register accessor: an alias for `Reg<ID3_SPEC>`"]
pub type ID3 = crate::Reg<id3::ID3_SPEC>;
#[doc = "Message Buffer 3 ID Register"]
pub mod id3;
#[doc = "WORD03 (rw) register accessor: an alias for `Reg<WORD03_SPEC>`"]
pub type WORD03 = crate::Reg<word03::WORD03_SPEC>;
#[doc = "Message Buffer 3 WORD0 Register"]
pub mod word03;
#[doc = "WORD13 (rw) register accessor: an alias for `Reg<WORD13_SPEC>`"]
pub type WORD13 = crate::Reg<word13::WORD13_SPEC>;
#[doc = "Message Buffer 3 WORD1 Register"]
pub mod word13;
#[doc = "CS4 (rw) register accessor: an alias for `Reg<CS4_SPEC>`"]
pub type CS4 = crate::Reg<cs4::CS4_SPEC>;
#[doc = "Message Buffer 4 CS Register"]
pub mod cs4;
#[doc = "ID4 (rw) register accessor: an alias for `Reg<ID4_SPEC>`"]
pub type ID4 = crate::Reg<id4::ID4_SPEC>;
#[doc = "Message Buffer 4 ID Register"]
pub mod id4;
#[doc = "WORD04 (rw) register accessor: an alias for `Reg<WORD04_SPEC>`"]
pub type WORD04 = crate::Reg<word04::WORD04_SPEC>;
#[doc = "Message Buffer 4 WORD0 Register"]
pub mod word04;
#[doc = "WORD14 (rw) register accessor: an alias for `Reg<WORD14_SPEC>`"]
pub type WORD14 = crate::Reg<word14::WORD14_SPEC>;
#[doc = "Message Buffer 4 WORD1 Register"]
pub mod word14;
#[doc = "CS5 (rw) register accessor: an alias for `Reg<CS5_SPEC>`"]
pub type CS5 = crate::Reg<cs5::CS5_SPEC>;
#[doc = "Message Buffer 5 CS Register"]
pub mod cs5;
#[doc = "ID5 (rw) register accessor: an alias for `Reg<ID5_SPEC>`"]
pub type ID5 = crate::Reg<id5::ID5_SPEC>;
#[doc = "Message Buffer 5 ID Register"]
pub mod id5;
#[doc = "WORD05 (rw) register accessor: an alias for `Reg<WORD05_SPEC>`"]
pub type WORD05 = crate::Reg<word05::WORD05_SPEC>;
#[doc = "Message Buffer 5 WORD0 Register"]
pub mod word05;
#[doc = "WORD15 (rw) register accessor: an alias for `Reg<WORD15_SPEC>`"]
pub type WORD15 = crate::Reg<word15::WORD15_SPEC>;
#[doc = "Message Buffer 5 WORD1 Register"]
pub mod word15;
#[doc = "CS6 (rw) register accessor: an alias for `Reg<CS6_SPEC>`"]
pub type CS6 = crate::Reg<cs6::CS6_SPEC>;
#[doc = "Message Buffer 6 CS Register"]
pub mod cs6;
#[doc = "ID6 (rw) register accessor: an alias for `Reg<ID6_SPEC>`"]
pub type ID6 = crate::Reg<id6::ID6_SPEC>;
#[doc = "Message Buffer 6 ID Register"]
pub mod id6;
#[doc = "WORD06 (rw) register accessor: an alias for `Reg<WORD06_SPEC>`"]
pub type WORD06 = crate::Reg<word06::WORD06_SPEC>;
#[doc = "Message Buffer 6 WORD0 Register"]
pub mod word06;
#[doc = "WORD16 (rw) register accessor: an alias for `Reg<WORD16_SPEC>`"]
pub type WORD16 = crate::Reg<word16::WORD16_SPEC>;
#[doc = "Message Buffer 6 WORD1 Register"]
pub mod word16;
#[doc = "CS7 (rw) register accessor: an alias for `Reg<CS7_SPEC>`"]
pub type CS7 = crate::Reg<cs7::CS7_SPEC>;
#[doc = "Message Buffer 7 CS Register"]
pub mod cs7;
#[doc = "ID7 (rw) register accessor: an alias for `Reg<ID7_SPEC>`"]
pub type ID7 = crate::Reg<id7::ID7_SPEC>;
#[doc = "Message Buffer 7 ID Register"]
pub mod id7;
#[doc = "WORD07 (rw) register accessor: an alias for `Reg<WORD07_SPEC>`"]
pub type WORD07 = crate::Reg<word07::WORD07_SPEC>;
#[doc = "Message Buffer 7 WORD0 Register"]
pub mod word07;
#[doc = "WORD17 (rw) register accessor: an alias for `Reg<WORD17_SPEC>`"]
pub type WORD17 = crate::Reg<word17::WORD17_SPEC>;
#[doc = "Message Buffer 7 WORD1 Register"]
pub mod word17;
#[doc = "CS8 (rw) register accessor: an alias for `Reg<CS8_SPEC>`"]
pub type CS8 = crate::Reg<cs8::CS8_SPEC>;
#[doc = "Message Buffer 8 CS Register"]
pub mod cs8;
#[doc = "ID8 (rw) register accessor: an alias for `Reg<ID8_SPEC>`"]
pub type ID8 = crate::Reg<id8::ID8_SPEC>;
#[doc = "Message Buffer 8 ID Register"]
pub mod id8;
#[doc = "WORD08 (rw) register accessor: an alias for `Reg<WORD08_SPEC>`"]
pub type WORD08 = crate::Reg<word08::WORD08_SPEC>;
#[doc = "Message Buffer 8 WORD0 Register"]
pub mod word08;
#[doc = "WORD18 (rw) register accessor: an alias for `Reg<WORD18_SPEC>`"]
pub type WORD18 = crate::Reg<word18::WORD18_SPEC>;
#[doc = "Message Buffer 8 WORD1 Register"]
pub mod word18;
#[doc = "CS9 (rw) register accessor: an alias for `Reg<CS9_SPEC>`"]
pub type CS9 = crate::Reg<cs9::CS9_SPEC>;
#[doc = "Message Buffer 9 CS Register"]
pub mod cs9;
#[doc = "ID9 (rw) register accessor: an alias for `Reg<ID9_SPEC>`"]
pub type ID9 = crate::Reg<id9::ID9_SPEC>;
#[doc = "Message Buffer 9 ID Register"]
pub mod id9;
#[doc = "WORD09 (rw) register accessor: an alias for `Reg<WORD09_SPEC>`"]
pub type WORD09 = crate::Reg<word09::WORD09_SPEC>;
#[doc = "Message Buffer 9 WORD0 Register"]
pub mod word09;
#[doc = "WORD19 (rw) register accessor: an alias for `Reg<WORD19_SPEC>`"]
pub type WORD19 = crate::Reg<word19::WORD19_SPEC>;
#[doc = "Message Buffer 9 WORD1 Register"]
pub mod word19;
#[doc = "CS10 (rw) register accessor: an alias for `Reg<CS10_SPEC>`"]
pub type CS10 = crate::Reg<cs10::CS10_SPEC>;
#[doc = "Message Buffer 10 CS Register"]
pub mod cs10;
#[doc = "ID10 (rw) register accessor: an alias for `Reg<ID10_SPEC>`"]
pub type ID10 = crate::Reg<id10::ID10_SPEC>;
#[doc = "Message Buffer 10 ID Register"]
pub mod id10;
#[doc = "WORD010 (rw) register accessor: an alias for `Reg<WORD010_SPEC>`"]
pub type WORD010 = crate::Reg<word010::WORD010_SPEC>;
#[doc = "Message Buffer 10 WORD0 Register"]
pub mod word010;
#[doc = "WORD110 (rw) register accessor: an alias for `Reg<WORD110_SPEC>`"]
pub type WORD110 = crate::Reg<word110::WORD110_SPEC>;
#[doc = "Message Buffer 10 WORD1 Register"]
pub mod word110;
#[doc = "CS11 (rw) register accessor: an alias for `Reg<CS11_SPEC>`"]
pub type CS11 = crate::Reg<cs11::CS11_SPEC>;
#[doc = "Message Buffer 11 CS Register"]
pub mod cs11;
#[doc = "ID11 (rw) register accessor: an alias for `Reg<ID11_SPEC>`"]
pub type ID11 = crate::Reg<id11::ID11_SPEC>;
#[doc = "Message Buffer 11 ID Register"]
pub mod id11;
#[doc = "WORD011 (rw) register accessor: an alias for `Reg<WORD011_SPEC>`"]
pub type WORD011 = crate::Reg<word011::WORD011_SPEC>;
#[doc = "Message Buffer 11 WORD0 Register"]
pub mod word011;
#[doc = "WORD111 (rw) register accessor: an alias for `Reg<WORD111_SPEC>`"]
pub type WORD111 = crate::Reg<word111::WORD111_SPEC>;
#[doc = "Message Buffer 11 WORD1 Register"]
pub mod word111;
#[doc = "CS12 (rw) register accessor: an alias for `Reg<CS12_SPEC>`"]
pub type CS12 = crate::Reg<cs12::CS12_SPEC>;
#[doc = "Message Buffer 12 CS Register"]
pub mod cs12;
#[doc = "ID12 (rw) register accessor: an alias for `Reg<ID12_SPEC>`"]
pub type ID12 = crate::Reg<id12::ID12_SPEC>;
#[doc = "Message Buffer 12 ID Register"]
pub mod id12;
#[doc = "WORD012 (rw) register accessor: an alias for `Reg<WORD012_SPEC>`"]
pub type WORD012 = crate::Reg<word012::WORD012_SPEC>;
#[doc = "Message Buffer 12 WORD0 Register"]
pub mod word012;
#[doc = "WORD112 (rw) register accessor: an alias for `Reg<WORD112_SPEC>`"]
pub type WORD112 = crate::Reg<word112::WORD112_SPEC>;
#[doc = "Message Buffer 12 WORD1 Register"]
pub mod word112;
#[doc = "CS13 (rw) register accessor: an alias for `Reg<CS13_SPEC>`"]
pub type CS13 = crate::Reg<cs13::CS13_SPEC>;
#[doc = "Message Buffer 13 CS Register"]
pub mod cs13;
#[doc = "ID13 (rw) register accessor: an alias for `Reg<ID13_SPEC>`"]
pub type ID13 = crate::Reg<id13::ID13_SPEC>;
#[doc = "Message Buffer 13 ID Register"]
pub mod id13;
#[doc = "WORD013 (rw) register accessor: an alias for `Reg<WORD013_SPEC>`"]
pub type WORD013 = crate::Reg<word013::WORD013_SPEC>;
#[doc = "Message Buffer 13 WORD0 Register"]
pub mod word013;
#[doc = "WORD113 (rw) register accessor: an alias for `Reg<WORD113_SPEC>`"]
pub type WORD113 = crate::Reg<word113::WORD113_SPEC>;
#[doc = "Message Buffer 13 WORD1 Register"]
pub mod word113;
#[doc = "CS14 (rw) register accessor: an alias for `Reg<CS14_SPEC>`"]
pub type CS14 = crate::Reg<cs14::CS14_SPEC>;
#[doc = "Message Buffer 14 CS Register"]
pub mod cs14;
#[doc = "ID14 (rw) register accessor: an alias for `Reg<ID14_SPEC>`"]
pub type ID14 = crate::Reg<id14::ID14_SPEC>;
#[doc = "Message Buffer 14 ID Register"]
pub mod id14;
#[doc = "WORD014 (rw) register accessor: an alias for `Reg<WORD014_SPEC>`"]
pub type WORD014 = crate::Reg<word014::WORD014_SPEC>;
#[doc = "Message Buffer 14 WORD0 Register"]
pub mod word014;
#[doc = "WORD114 (rw) register accessor: an alias for `Reg<WORD114_SPEC>`"]
pub type WORD114 = crate::Reg<word114::WORD114_SPEC>;
#[doc = "Message Buffer 14 WORD1 Register"]
pub mod word114;
#[doc = "CS15 (rw) register accessor: an alias for `Reg<CS15_SPEC>`"]
pub type CS15 = crate::Reg<cs15::CS15_SPEC>;
#[doc = "Message Buffer 15 CS Register"]
pub mod cs15;
#[doc = "ID15 (rw) register accessor: an alias for `Reg<ID15_SPEC>`"]
pub type ID15 = crate::Reg<id15::ID15_SPEC>;
#[doc = "Message Buffer 15 ID Register"]
pub mod id15;
#[doc = "WORD015 (rw) register accessor: an alias for `Reg<WORD015_SPEC>`"]
pub type WORD015 = crate::Reg<word015::WORD015_SPEC>;
#[doc = "Message Buffer 15 WORD0 Register"]
pub mod word015;
#[doc = "WORD115 (rw) register accessor: an alias for `Reg<WORD115_SPEC>`"]
pub type WORD115 = crate::Reg<word115::WORD115_SPEC>;
#[doc = "Message Buffer 15 WORD1 Register"]
pub mod word115;
#[doc = "CS16 (rw) register accessor: an alias for `Reg<CS16_SPEC>`"]
pub type CS16 = crate::Reg<cs16::CS16_SPEC>;
#[doc = "Message Buffer 16 CS Register"]
pub mod cs16;
#[doc = "ID16 (rw) register accessor: an alias for `Reg<ID16_SPEC>`"]
pub type ID16 = crate::Reg<id16::ID16_SPEC>;
#[doc = "Message Buffer 16 ID Register"]
pub mod id16;
#[doc = "WORD016 (rw) register accessor: an alias for `Reg<WORD016_SPEC>`"]
pub type WORD016 = crate::Reg<word016::WORD016_SPEC>;
#[doc = "Message Buffer 16 WORD0 Register"]
pub mod word016;
#[doc = "WORD116 (rw) register accessor: an alias for `Reg<WORD116_SPEC>`"]
pub type WORD116 = crate::Reg<word116::WORD116_SPEC>;
#[doc = "Message Buffer 16 WORD1 Register"]
pub mod word116;
#[doc = "CS17 (rw) register accessor: an alias for `Reg<CS17_SPEC>`"]
pub type CS17 = crate::Reg<cs17::CS17_SPEC>;
#[doc = "Message Buffer 17 CS Register"]
pub mod cs17;
#[doc = "ID17 (rw) register accessor: an alias for `Reg<ID17_SPEC>`"]
pub type ID17 = crate::Reg<id17::ID17_SPEC>;
#[doc = "Message Buffer 17 ID Register"]
pub mod id17;
#[doc = "WORD017 (rw) register accessor: an alias for `Reg<WORD017_SPEC>`"]
pub type WORD017 = crate::Reg<word017::WORD017_SPEC>;
#[doc = "Message Buffer 17 WORD0 Register"]
pub mod word017;
#[doc = "WORD117 (rw) register accessor: an alias for `Reg<WORD117_SPEC>`"]
pub type WORD117 = crate::Reg<word117::WORD117_SPEC>;
#[doc = "Message Buffer 17 WORD1 Register"]
pub mod word117;
#[doc = "CS18 (rw) register accessor: an alias for `Reg<CS18_SPEC>`"]
pub type CS18 = crate::Reg<cs18::CS18_SPEC>;
#[doc = "Message Buffer 18 CS Register"]
pub mod cs18;
#[doc = "ID18 (rw) register accessor: an alias for `Reg<ID18_SPEC>`"]
pub type ID18 = crate::Reg<id18::ID18_SPEC>;
#[doc = "Message Buffer 18 ID Register"]
pub mod id18;
#[doc = "WORD018 (rw) register accessor: an alias for `Reg<WORD018_SPEC>`"]
pub type WORD018 = crate::Reg<word018::WORD018_SPEC>;
#[doc = "Message Buffer 18 WORD0 Register"]
pub mod word018;
#[doc = "WORD118 (rw) register accessor: an alias for `Reg<WORD118_SPEC>`"]
pub type WORD118 = crate::Reg<word118::WORD118_SPEC>;
#[doc = "Message Buffer 18 WORD1 Register"]
pub mod word118;
#[doc = "CS19 (rw) register accessor: an alias for `Reg<CS19_SPEC>`"]
pub type CS19 = crate::Reg<cs19::CS19_SPEC>;
#[doc = "Message Buffer 19 CS Register"]
pub mod cs19;
#[doc = "ID19 (rw) register accessor: an alias for `Reg<ID19_SPEC>`"]
pub type ID19 = crate::Reg<id19::ID19_SPEC>;
#[doc = "Message Buffer 19 ID Register"]
pub mod id19;
#[doc = "WORD019 (rw) register accessor: an alias for `Reg<WORD019_SPEC>`"]
pub type WORD019 = crate::Reg<word019::WORD019_SPEC>;
#[doc = "Message Buffer 19 WORD0 Register"]
pub mod word019;
#[doc = "WORD119 (rw) register accessor: an alias for `Reg<WORD119_SPEC>`"]
pub type WORD119 = crate::Reg<word119::WORD119_SPEC>;
#[doc = "Message Buffer 19 WORD1 Register"]
pub mod word119;
#[doc = "CS20 (rw) register accessor: an alias for `Reg<CS20_SPEC>`"]
pub type CS20 = crate::Reg<cs20::CS20_SPEC>;
#[doc = "Message Buffer 20 CS Register"]
pub mod cs20;
#[doc = "ID20 (rw) register accessor: an alias for `Reg<ID20_SPEC>`"]
pub type ID20 = crate::Reg<id20::ID20_SPEC>;
#[doc = "Message Buffer 20 ID Register"]
pub mod id20;
#[doc = "WORD020 (rw) register accessor: an alias for `Reg<WORD020_SPEC>`"]
pub type WORD020 = crate::Reg<word020::WORD020_SPEC>;
#[doc = "Message Buffer 20 WORD0 Register"]
pub mod word020;
#[doc = "WORD120 (rw) register accessor: an alias for `Reg<WORD120_SPEC>`"]
pub type WORD120 = crate::Reg<word120::WORD120_SPEC>;
#[doc = "Message Buffer 20 WORD1 Register"]
pub mod word120;
#[doc = "CS21 (rw) register accessor: an alias for `Reg<CS21_SPEC>`"]
pub type CS21 = crate::Reg<cs21::CS21_SPEC>;
#[doc = "Message Buffer 21 CS Register"]
pub mod cs21;
#[doc = "ID21 (rw) register accessor: an alias for `Reg<ID21_SPEC>`"]
pub type ID21 = crate::Reg<id21::ID21_SPEC>;
#[doc = "Message Buffer 21 ID Register"]
pub mod id21;
#[doc = "WORD021 (rw) register accessor: an alias for `Reg<WORD021_SPEC>`"]
pub type WORD021 = crate::Reg<word021::WORD021_SPEC>;
#[doc = "Message Buffer 21 WORD0 Register"]
pub mod word021;
#[doc = "WORD121 (rw) register accessor: an alias for `Reg<WORD121_SPEC>`"]
pub type WORD121 = crate::Reg<word121::WORD121_SPEC>;
#[doc = "Message Buffer 21 WORD1 Register"]
pub mod word121;
#[doc = "CS22 (rw) register accessor: an alias for `Reg<CS22_SPEC>`"]
pub type CS22 = crate::Reg<cs22::CS22_SPEC>;
#[doc = "Message Buffer 22 CS Register"]
pub mod cs22;
#[doc = "ID22 (rw) register accessor: an alias for `Reg<ID22_SPEC>`"]
pub type ID22 = crate::Reg<id22::ID22_SPEC>;
#[doc = "Message Buffer 22 ID Register"]
pub mod id22;
#[doc = "WORD022 (rw) register accessor: an alias for `Reg<WORD022_SPEC>`"]
pub type WORD022 = crate::Reg<word022::WORD022_SPEC>;
#[doc = "Message Buffer 22 WORD0 Register"]
pub mod word022;
#[doc = "WORD122 (rw) register accessor: an alias for `Reg<WORD122_SPEC>`"]
pub type WORD122 = crate::Reg<word122::WORD122_SPEC>;
#[doc = "Message Buffer 22 WORD1 Register"]
pub mod word122;
#[doc = "CS23 (rw) register accessor: an alias for `Reg<CS23_SPEC>`"]
pub type CS23 = crate::Reg<cs23::CS23_SPEC>;
#[doc = "Message Buffer 23 CS Register"]
pub mod cs23;
#[doc = "ID23 (rw) register accessor: an alias for `Reg<ID23_SPEC>`"]
pub type ID23 = crate::Reg<id23::ID23_SPEC>;
#[doc = "Message Buffer 23 ID Register"]
pub mod id23;
#[doc = "WORD023 (rw) register accessor: an alias for `Reg<WORD023_SPEC>`"]
pub type WORD023 = crate::Reg<word023::WORD023_SPEC>;
#[doc = "Message Buffer 23 WORD0 Register"]
pub mod word023;
#[doc = "WORD123 (rw) register accessor: an alias for `Reg<WORD123_SPEC>`"]
pub type WORD123 = crate::Reg<word123::WORD123_SPEC>;
#[doc = "Message Buffer 23 WORD1 Register"]
pub mod word123;
#[doc = "CS24 (rw) register accessor: an alias for `Reg<CS24_SPEC>`"]
pub type CS24 = crate::Reg<cs24::CS24_SPEC>;
#[doc = "Message Buffer 24 CS Register"]
pub mod cs24;
#[doc = "ID24 (rw) register accessor: an alias for `Reg<ID24_SPEC>`"]
pub type ID24 = crate::Reg<id24::ID24_SPEC>;
#[doc = "Message Buffer 24 ID Register"]
pub mod id24;
#[doc = "WORD024 (rw) register accessor: an alias for `Reg<WORD024_SPEC>`"]
pub type WORD024 = crate::Reg<word024::WORD024_SPEC>;
#[doc = "Message Buffer 24 WORD0 Register"]
pub mod word024;
#[doc = "WORD124 (rw) register accessor: an alias for `Reg<WORD124_SPEC>`"]
pub type WORD124 = crate::Reg<word124::WORD124_SPEC>;
#[doc = "Message Buffer 24 WORD1 Register"]
pub mod word124;
#[doc = "CS25 (rw) register accessor: an alias for `Reg<CS25_SPEC>`"]
pub type CS25 = crate::Reg<cs25::CS25_SPEC>;
#[doc = "Message Buffer 25 CS Register"]
pub mod cs25;
#[doc = "ID25 (rw) register accessor: an alias for `Reg<ID25_SPEC>`"]
pub type ID25 = crate::Reg<id25::ID25_SPEC>;
#[doc = "Message Buffer 25 ID Register"]
pub mod id25;
#[doc = "WORD025 (rw) register accessor: an alias for `Reg<WORD025_SPEC>`"]
pub type WORD025 = crate::Reg<word025::WORD025_SPEC>;
#[doc = "Message Buffer 25 WORD0 Register"]
pub mod word025;
#[doc = "WORD125 (rw) register accessor: an alias for `Reg<WORD125_SPEC>`"]
pub type WORD125 = crate::Reg<word125::WORD125_SPEC>;
#[doc = "Message Buffer 25 WORD1 Register"]
pub mod word125;
#[doc = "CS26 (rw) register accessor: an alias for `Reg<CS26_SPEC>`"]
pub type CS26 = crate::Reg<cs26::CS26_SPEC>;
#[doc = "Message Buffer 26 CS Register"]
pub mod cs26;
#[doc = "ID26 (rw) register accessor: an alias for `Reg<ID26_SPEC>`"]
pub type ID26 = crate::Reg<id26::ID26_SPEC>;
#[doc = "Message Buffer 26 ID Register"]
pub mod id26;
#[doc = "WORD026 (rw) register accessor: an alias for `Reg<WORD026_SPEC>`"]
pub type WORD026 = crate::Reg<word026::WORD026_SPEC>;
#[doc = "Message Buffer 26 WORD0 Register"]
pub mod word026;
#[doc = "WORD126 (rw) register accessor: an alias for `Reg<WORD126_SPEC>`"]
pub type WORD126 = crate::Reg<word126::WORD126_SPEC>;
#[doc = "Message Buffer 26 WORD1 Register"]
pub mod word126;
#[doc = "CS27 (rw) register accessor: an alias for `Reg<CS27_SPEC>`"]
pub type CS27 = crate::Reg<cs27::CS27_SPEC>;
#[doc = "Message Buffer 27 CS Register"]
pub mod cs27;
#[doc = "ID27 (rw) register accessor: an alias for `Reg<ID27_SPEC>`"]
pub type ID27 = crate::Reg<id27::ID27_SPEC>;
#[doc = "Message Buffer 27 ID Register"]
pub mod id27;
#[doc = "WORD027 (rw) register accessor: an alias for `Reg<WORD027_SPEC>`"]
pub type WORD027 = crate::Reg<word027::WORD027_SPEC>;
#[doc = "Message Buffer 27 WORD0 Register"]
pub mod word027;
#[doc = "WORD127 (rw) register accessor: an alias for `Reg<WORD127_SPEC>`"]
pub type WORD127 = crate::Reg<word127::WORD127_SPEC>;
#[doc = "Message Buffer 27 WORD1 Register"]
pub mod word127;
#[doc = "CS28 (rw) register accessor: an alias for `Reg<CS28_SPEC>`"]
pub type CS28 = crate::Reg<cs28::CS28_SPEC>;
#[doc = "Message Buffer 28 CS Register"]
pub mod cs28;
#[doc = "ID28 (rw) register accessor: an alias for `Reg<ID28_SPEC>`"]
pub type ID28 = crate::Reg<id28::ID28_SPEC>;
#[doc = "Message Buffer 28 ID Register"]
pub mod id28;
#[doc = "WORD028 (rw) register accessor: an alias for `Reg<WORD028_SPEC>`"]
pub type WORD028 = crate::Reg<word028::WORD028_SPEC>;
#[doc = "Message Buffer 28 WORD0 Register"]
pub mod word028;
#[doc = "WORD128 (rw) register accessor: an alias for `Reg<WORD128_SPEC>`"]
pub type WORD128 = crate::Reg<word128::WORD128_SPEC>;
#[doc = "Message Buffer 28 WORD1 Register"]
pub mod word128;
#[doc = "CS29 (rw) register accessor: an alias for `Reg<CS29_SPEC>`"]
pub type CS29 = crate::Reg<cs29::CS29_SPEC>;
#[doc = "Message Buffer 29 CS Register"]
pub mod cs29;
#[doc = "ID29 (rw) register accessor: an alias for `Reg<ID29_SPEC>`"]
pub type ID29 = crate::Reg<id29::ID29_SPEC>;
#[doc = "Message Buffer 29 ID Register"]
pub mod id29;
#[doc = "WORD029 (rw) register accessor: an alias for `Reg<WORD029_SPEC>`"]
pub type WORD029 = crate::Reg<word029::WORD029_SPEC>;
#[doc = "Message Buffer 29 WORD0 Register"]
pub mod word029;
#[doc = "WORD129 (rw) register accessor: an alias for `Reg<WORD129_SPEC>`"]
pub type WORD129 = crate::Reg<word129::WORD129_SPEC>;
#[doc = "Message Buffer 29 WORD1 Register"]
pub mod word129;
#[doc = "CS30 (rw) register accessor: an alias for `Reg<CS30_SPEC>`"]
pub type CS30 = crate::Reg<cs30::CS30_SPEC>;
#[doc = "Message Buffer 30 CS Register"]
pub mod cs30;
#[doc = "ID30 (rw) register accessor: an alias for `Reg<ID30_SPEC>`"]
pub type ID30 = crate::Reg<id30::ID30_SPEC>;
#[doc = "Message Buffer 30 ID Register"]
pub mod id30;
#[doc = "WORD030 (rw) register accessor: an alias for `Reg<WORD030_SPEC>`"]
pub type WORD030 = crate::Reg<word030::WORD030_SPEC>;
#[doc = "Message Buffer 30 WORD0 Register"]
pub mod word030;
#[doc = "WORD130 (rw) register accessor: an alias for `Reg<WORD130_SPEC>`"]
pub type WORD130 = crate::Reg<word130::WORD130_SPEC>;
#[doc = "Message Buffer 30 WORD1 Register"]
pub mod word130;
#[doc = "CS31 (rw) register accessor: an alias for `Reg<CS31_SPEC>`"]
pub type CS31 = crate::Reg<cs31::CS31_SPEC>;
#[doc = "Message Buffer 31 CS Register"]
pub mod cs31;
#[doc = "ID31 (rw) register accessor: an alias for `Reg<ID31_SPEC>`"]
pub type ID31 = crate::Reg<id31::ID31_SPEC>;
#[doc = "Message Buffer 31 ID Register"]
pub mod id31;
#[doc = "WORD031 (rw) register accessor: an alias for `Reg<WORD031_SPEC>`"]
pub type WORD031 = crate::Reg<word031::WORD031_SPEC>;
#[doc = "Message Buffer 31 WORD0 Register"]
pub mod word031;
#[doc = "WORD131 (rw) register accessor: an alias for `Reg<WORD131_SPEC>`"]
pub type WORD131 = crate::Reg<word131::WORD131_SPEC>;
#[doc = "Message Buffer 31 WORD1 Register"]
pub mod word131;
#[doc = "CS32 (rw) register accessor: an alias for `Reg<CS32_SPEC>`"]
pub type CS32 = crate::Reg<cs32::CS32_SPEC>;
#[doc = "Message Buffer 32 CS Register"]
pub mod cs32;
#[doc = "ID32 (rw) register accessor: an alias for `Reg<ID32_SPEC>`"]
pub type ID32 = crate::Reg<id32::ID32_SPEC>;
#[doc = "Message Buffer 32 ID Register"]
pub mod id32;
#[doc = "WORD032 (rw) register accessor: an alias for `Reg<WORD032_SPEC>`"]
pub type WORD032 = crate::Reg<word032::WORD032_SPEC>;
#[doc = "Message Buffer 32 WORD0 Register"]
pub mod word032;
#[doc = "WORD132 (rw) register accessor: an alias for `Reg<WORD132_SPEC>`"]
pub type WORD132 = crate::Reg<word132::WORD132_SPEC>;
#[doc = "Message Buffer 32 WORD1 Register"]
pub mod word132;
#[doc = "CS33 (rw) register accessor: an alias for `Reg<CS33_SPEC>`"]
pub type CS33 = crate::Reg<cs33::CS33_SPEC>;
#[doc = "Message Buffer 33 CS Register"]
pub mod cs33;
#[doc = "ID33 (rw) register accessor: an alias for `Reg<ID33_SPEC>`"]
pub type ID33 = crate::Reg<id33::ID33_SPEC>;
#[doc = "Message Buffer 33 ID Register"]
pub mod id33;
#[doc = "WORD033 (rw) register accessor: an alias for `Reg<WORD033_SPEC>`"]
pub type WORD033 = crate::Reg<word033::WORD033_SPEC>;
#[doc = "Message Buffer 33 WORD0 Register"]
pub mod word033;
#[doc = "WORD133 (rw) register accessor: an alias for `Reg<WORD133_SPEC>`"]
pub type WORD133 = crate::Reg<word133::WORD133_SPEC>;
#[doc = "Message Buffer 33 WORD1 Register"]
pub mod word133;
#[doc = "CS34 (rw) register accessor: an alias for `Reg<CS34_SPEC>`"]
pub type CS34 = crate::Reg<cs34::CS34_SPEC>;
#[doc = "Message Buffer 34 CS Register"]
pub mod cs34;
#[doc = "ID34 (rw) register accessor: an alias for `Reg<ID34_SPEC>`"]
pub type ID34 = crate::Reg<id34::ID34_SPEC>;
#[doc = "Message Buffer 34 ID Register"]
pub mod id34;
#[doc = "WORD034 (rw) register accessor: an alias for `Reg<WORD034_SPEC>`"]
pub type WORD034 = crate::Reg<word034::WORD034_SPEC>;
#[doc = "Message Buffer 34 WORD0 Register"]
pub mod word034;
#[doc = "WORD134 (rw) register accessor: an alias for `Reg<WORD134_SPEC>`"]
pub type WORD134 = crate::Reg<word134::WORD134_SPEC>;
#[doc = "Message Buffer 34 WORD1 Register"]
pub mod word134;
#[doc = "CS35 (rw) register accessor: an alias for `Reg<CS35_SPEC>`"]
pub type CS35 = crate::Reg<cs35::CS35_SPEC>;
#[doc = "Message Buffer 35 CS Register"]
pub mod cs35;
#[doc = "ID35 (rw) register accessor: an alias for `Reg<ID35_SPEC>`"]
pub type ID35 = crate::Reg<id35::ID35_SPEC>;
#[doc = "Message Buffer 35 ID Register"]
pub mod id35;
#[doc = "WORD035 (rw) register accessor: an alias for `Reg<WORD035_SPEC>`"]
pub type WORD035 = crate::Reg<word035::WORD035_SPEC>;
#[doc = "Message Buffer 35 WORD0 Register"]
pub mod word035;
#[doc = "WORD135 (rw) register accessor: an alias for `Reg<WORD135_SPEC>`"]
pub type WORD135 = crate::Reg<word135::WORD135_SPEC>;
#[doc = "Message Buffer 35 WORD1 Register"]
pub mod word135;
#[doc = "CS36 (rw) register accessor: an alias for `Reg<CS36_SPEC>`"]
pub type CS36 = crate::Reg<cs36::CS36_SPEC>;
#[doc = "Message Buffer 36 CS Register"]
pub mod cs36;
#[doc = "ID36 (rw) register accessor: an alias for `Reg<ID36_SPEC>`"]
pub type ID36 = crate::Reg<id36::ID36_SPEC>;
#[doc = "Message Buffer 36 ID Register"]
pub mod id36;
#[doc = "WORD036 (rw) register accessor: an alias for `Reg<WORD036_SPEC>`"]
pub type WORD036 = crate::Reg<word036::WORD036_SPEC>;
#[doc = "Message Buffer 36 WORD0 Register"]
pub mod word036;
#[doc = "WORD136 (rw) register accessor: an alias for `Reg<WORD136_SPEC>`"]
pub type WORD136 = crate::Reg<word136::WORD136_SPEC>;
#[doc = "Message Buffer 36 WORD1 Register"]
pub mod word136;
#[doc = "CS37 (rw) register accessor: an alias for `Reg<CS37_SPEC>`"]
pub type CS37 = crate::Reg<cs37::CS37_SPEC>;
#[doc = "Message Buffer 37 CS Register"]
pub mod cs37;
#[doc = "ID37 (rw) register accessor: an alias for `Reg<ID37_SPEC>`"]
pub type ID37 = crate::Reg<id37::ID37_SPEC>;
#[doc = "Message Buffer 37 ID Register"]
pub mod id37;
#[doc = "WORD037 (rw) register accessor: an alias for `Reg<WORD037_SPEC>`"]
pub type WORD037 = crate::Reg<word037::WORD037_SPEC>;
#[doc = "Message Buffer 37 WORD0 Register"]
pub mod word037;
#[doc = "WORD137 (rw) register accessor: an alias for `Reg<WORD137_SPEC>`"]
pub type WORD137 = crate::Reg<word137::WORD137_SPEC>;
#[doc = "Message Buffer 37 WORD1 Register"]
pub mod word137;
#[doc = "CS38 (rw) register accessor: an alias for `Reg<CS38_SPEC>`"]
pub type CS38 = crate::Reg<cs38::CS38_SPEC>;
#[doc = "Message Buffer 38 CS Register"]
pub mod cs38;
#[doc = "ID38 (rw) register accessor: an alias for `Reg<ID38_SPEC>`"]
pub type ID38 = crate::Reg<id38::ID38_SPEC>;
#[doc = "Message Buffer 38 ID Register"]
pub mod id38;
#[doc = "WORD038 (rw) register accessor: an alias for `Reg<WORD038_SPEC>`"]
pub type WORD038 = crate::Reg<word038::WORD038_SPEC>;
#[doc = "Message Buffer 38 WORD0 Register"]
pub mod word038;
#[doc = "WORD138 (rw) register accessor: an alias for `Reg<WORD138_SPEC>`"]
pub type WORD138 = crate::Reg<word138::WORD138_SPEC>;
#[doc = "Message Buffer 38 WORD1 Register"]
pub mod word138;
#[doc = "CS39 (rw) register accessor: an alias for `Reg<CS39_SPEC>`"]
pub type CS39 = crate::Reg<cs39::CS39_SPEC>;
#[doc = "Message Buffer 39 CS Register"]
pub mod cs39;
#[doc = "ID39 (rw) register accessor: an alias for `Reg<ID39_SPEC>`"]
pub type ID39 = crate::Reg<id39::ID39_SPEC>;
#[doc = "Message Buffer 39 ID Register"]
pub mod id39;
#[doc = "WORD039 (rw) register accessor: an alias for `Reg<WORD039_SPEC>`"]
pub type WORD039 = crate::Reg<word039::WORD039_SPEC>;
#[doc = "Message Buffer 39 WORD0 Register"]
pub mod word039;
#[doc = "WORD139 (rw) register accessor: an alias for `Reg<WORD139_SPEC>`"]
pub type WORD139 = crate::Reg<word139::WORD139_SPEC>;
#[doc = "Message Buffer 39 WORD1 Register"]
pub mod word139;
#[doc = "CS40 (rw) register accessor: an alias for `Reg<CS40_SPEC>`"]
pub type CS40 = crate::Reg<cs40::CS40_SPEC>;
#[doc = "Message Buffer 40 CS Register"]
pub mod cs40;
#[doc = "ID40 (rw) register accessor: an alias for `Reg<ID40_SPEC>`"]
pub type ID40 = crate::Reg<id40::ID40_SPEC>;
#[doc = "Message Buffer 40 ID Register"]
pub mod id40;
#[doc = "WORD040 (rw) register accessor: an alias for `Reg<WORD040_SPEC>`"]
pub type WORD040 = crate::Reg<word040::WORD040_SPEC>;
#[doc = "Message Buffer 40 WORD0 Register"]
pub mod word040;
#[doc = "WORD140 (rw) register accessor: an alias for `Reg<WORD140_SPEC>`"]
pub type WORD140 = crate::Reg<word140::WORD140_SPEC>;
#[doc = "Message Buffer 40 WORD1 Register"]
pub mod word140;
#[doc = "CS41 (rw) register accessor: an alias for `Reg<CS41_SPEC>`"]
pub type CS41 = crate::Reg<cs41::CS41_SPEC>;
#[doc = "Message Buffer 41 CS Register"]
pub mod cs41;
#[doc = "ID41 (rw) register accessor: an alias for `Reg<ID41_SPEC>`"]
pub type ID41 = crate::Reg<id41::ID41_SPEC>;
#[doc = "Message Buffer 41 ID Register"]
pub mod id41;
#[doc = "WORD041 (rw) register accessor: an alias for `Reg<WORD041_SPEC>`"]
pub type WORD041 = crate::Reg<word041::WORD041_SPEC>;
#[doc = "Message Buffer 41 WORD0 Register"]
pub mod word041;
#[doc = "WORD141 (rw) register accessor: an alias for `Reg<WORD141_SPEC>`"]
pub type WORD141 = crate::Reg<word141::WORD141_SPEC>;
#[doc = "Message Buffer 41 WORD1 Register"]
pub mod word141;
#[doc = "CS42 (rw) register accessor: an alias for `Reg<CS42_SPEC>`"]
pub type CS42 = crate::Reg<cs42::CS42_SPEC>;
#[doc = "Message Buffer 42 CS Register"]
pub mod cs42;
#[doc = "ID42 (rw) register accessor: an alias for `Reg<ID42_SPEC>`"]
pub type ID42 = crate::Reg<id42::ID42_SPEC>;
#[doc = "Message Buffer 42 ID Register"]
pub mod id42;
#[doc = "WORD042 (rw) register accessor: an alias for `Reg<WORD042_SPEC>`"]
pub type WORD042 = crate::Reg<word042::WORD042_SPEC>;
#[doc = "Message Buffer 42 WORD0 Register"]
pub mod word042;
#[doc = "WORD142 (rw) register accessor: an alias for `Reg<WORD142_SPEC>`"]
pub type WORD142 = crate::Reg<word142::WORD142_SPEC>;
#[doc = "Message Buffer 42 WORD1 Register"]
pub mod word142;
#[doc = "CS43 (rw) register accessor: an alias for `Reg<CS43_SPEC>`"]
pub type CS43 = crate::Reg<cs43::CS43_SPEC>;
#[doc = "Message Buffer 43 CS Register"]
pub mod cs43;
#[doc = "ID43 (rw) register accessor: an alias for `Reg<ID43_SPEC>`"]
pub type ID43 = crate::Reg<id43::ID43_SPEC>;
#[doc = "Message Buffer 43 ID Register"]
pub mod id43;
#[doc = "WORD043 (rw) register accessor: an alias for `Reg<WORD043_SPEC>`"]
pub type WORD043 = crate::Reg<word043::WORD043_SPEC>;
#[doc = "Message Buffer 43 WORD0 Register"]
pub mod word043;
#[doc = "WORD143 (rw) register accessor: an alias for `Reg<WORD143_SPEC>`"]
pub type WORD143 = crate::Reg<word143::WORD143_SPEC>;
#[doc = "Message Buffer 43 WORD1 Register"]
pub mod word143;
#[doc = "CS44 (rw) register accessor: an alias for `Reg<CS44_SPEC>`"]
pub type CS44 = crate::Reg<cs44::CS44_SPEC>;
#[doc = "Message Buffer 44 CS Register"]
pub mod cs44;
#[doc = "ID44 (rw) register accessor: an alias for `Reg<ID44_SPEC>`"]
pub type ID44 = crate::Reg<id44::ID44_SPEC>;
#[doc = "Message Buffer 44 ID Register"]
pub mod id44;
#[doc = "WORD044 (rw) register accessor: an alias for `Reg<WORD044_SPEC>`"]
pub type WORD044 = crate::Reg<word044::WORD044_SPEC>;
#[doc = "Message Buffer 44 WORD0 Register"]
pub mod word044;
#[doc = "WORD144 (rw) register accessor: an alias for `Reg<WORD144_SPEC>`"]
pub type WORD144 = crate::Reg<word144::WORD144_SPEC>;
#[doc = "Message Buffer 44 WORD1 Register"]
pub mod word144;
#[doc = "CS45 (rw) register accessor: an alias for `Reg<CS45_SPEC>`"]
pub type CS45 = crate::Reg<cs45::CS45_SPEC>;
#[doc = "Message Buffer 45 CS Register"]
pub mod cs45;
#[doc = "ID45 (rw) register accessor: an alias for `Reg<ID45_SPEC>`"]
pub type ID45 = crate::Reg<id45::ID45_SPEC>;
#[doc = "Message Buffer 45 ID Register"]
pub mod id45;
#[doc = "WORD045 (rw) register accessor: an alias for `Reg<WORD045_SPEC>`"]
pub type WORD045 = crate::Reg<word045::WORD045_SPEC>;
#[doc = "Message Buffer 45 WORD0 Register"]
pub mod word045;
#[doc = "WORD145 (rw) register accessor: an alias for `Reg<WORD145_SPEC>`"]
pub type WORD145 = crate::Reg<word145::WORD145_SPEC>;
#[doc = "Message Buffer 45 WORD1 Register"]
pub mod word145;
#[doc = "CS46 (rw) register accessor: an alias for `Reg<CS46_SPEC>`"]
pub type CS46 = crate::Reg<cs46::CS46_SPEC>;
#[doc = "Message Buffer 46 CS Register"]
pub mod cs46;
#[doc = "ID46 (rw) register accessor: an alias for `Reg<ID46_SPEC>`"]
pub type ID46 = crate::Reg<id46::ID46_SPEC>;
#[doc = "Message Buffer 46 ID Register"]
pub mod id46;
#[doc = "WORD046 (rw) register accessor: an alias for `Reg<WORD046_SPEC>`"]
pub type WORD046 = crate::Reg<word046::WORD046_SPEC>;
#[doc = "Message Buffer 46 WORD0 Register"]
pub mod word046;
#[doc = "WORD146 (rw) register accessor: an alias for `Reg<WORD146_SPEC>`"]
pub type WORD146 = crate::Reg<word146::WORD146_SPEC>;
#[doc = "Message Buffer 46 WORD1 Register"]
pub mod word146;
#[doc = "CS47 (rw) register accessor: an alias for `Reg<CS47_SPEC>`"]
pub type CS47 = crate::Reg<cs47::CS47_SPEC>;
#[doc = "Message Buffer 47 CS Register"]
pub mod cs47;
#[doc = "ID47 (rw) register accessor: an alias for `Reg<ID47_SPEC>`"]
pub type ID47 = crate::Reg<id47::ID47_SPEC>;
#[doc = "Message Buffer 47 ID Register"]
pub mod id47;
#[doc = "WORD047 (rw) register accessor: an alias for `Reg<WORD047_SPEC>`"]
pub type WORD047 = crate::Reg<word047::WORD047_SPEC>;
#[doc = "Message Buffer 47 WORD0 Register"]
pub mod word047;
#[doc = "WORD147 (rw) register accessor: an alias for `Reg<WORD147_SPEC>`"]
pub type WORD147 = crate::Reg<word147::WORD147_SPEC>;
#[doc = "Message Buffer 47 WORD1 Register"]
pub mod word147;
#[doc = "CS48 (rw) register accessor: an alias for `Reg<CS48_SPEC>`"]
pub type CS48 = crate::Reg<cs48::CS48_SPEC>;
#[doc = "Message Buffer 48 CS Register"]
pub mod cs48;
#[doc = "ID48 (rw) register accessor: an alias for `Reg<ID48_SPEC>`"]
pub type ID48 = crate::Reg<id48::ID48_SPEC>;
#[doc = "Message Buffer 48 ID Register"]
pub mod id48;
#[doc = "WORD048 (rw) register accessor: an alias for `Reg<WORD048_SPEC>`"]
pub type WORD048 = crate::Reg<word048::WORD048_SPEC>;
#[doc = "Message Buffer 48 WORD0 Register"]
pub mod word048;
#[doc = "WORD148 (rw) register accessor: an alias for `Reg<WORD148_SPEC>`"]
pub type WORD148 = crate::Reg<word148::WORD148_SPEC>;
#[doc = "Message Buffer 48 WORD1 Register"]
pub mod word148;
#[doc = "CS49 (rw) register accessor: an alias for `Reg<CS49_SPEC>`"]
pub type CS49 = crate::Reg<cs49::CS49_SPEC>;
#[doc = "Message Buffer 49 CS Register"]
pub mod cs49;
#[doc = "ID49 (rw) register accessor: an alias for `Reg<ID49_SPEC>`"]
pub type ID49 = crate::Reg<id49::ID49_SPEC>;
#[doc = "Message Buffer 49 ID Register"]
pub mod id49;
#[doc = "WORD049 (rw) register accessor: an alias for `Reg<WORD049_SPEC>`"]
pub type WORD049 = crate::Reg<word049::WORD049_SPEC>;
#[doc = "Message Buffer 49 WORD0 Register"]
pub mod word049;
#[doc = "WORD149 (rw) register accessor: an alias for `Reg<WORD149_SPEC>`"]
pub type WORD149 = crate::Reg<word149::WORD149_SPEC>;
#[doc = "Message Buffer 49 WORD1 Register"]
pub mod word149;
#[doc = "CS50 (rw) register accessor: an alias for `Reg<CS50_SPEC>`"]
pub type CS50 = crate::Reg<cs50::CS50_SPEC>;
#[doc = "Message Buffer 50 CS Register"]
pub mod cs50;
#[doc = "ID50 (rw) register accessor: an alias for `Reg<ID50_SPEC>`"]
pub type ID50 = crate::Reg<id50::ID50_SPEC>;
#[doc = "Message Buffer 50 ID Register"]
pub mod id50;
#[doc = "WORD050 (rw) register accessor: an alias for `Reg<WORD050_SPEC>`"]
pub type WORD050 = crate::Reg<word050::WORD050_SPEC>;
#[doc = "Message Buffer 50 WORD0 Register"]
pub mod word050;
#[doc = "WORD150 (rw) register accessor: an alias for `Reg<WORD150_SPEC>`"]
pub type WORD150 = crate::Reg<word150::WORD150_SPEC>;
#[doc = "Message Buffer 50 WORD1 Register"]
pub mod word150;
#[doc = "CS51 (rw) register accessor: an alias for `Reg<CS51_SPEC>`"]
pub type CS51 = crate::Reg<cs51::CS51_SPEC>;
#[doc = "Message Buffer 51 CS Register"]
pub mod cs51;
#[doc = "ID51 (rw) register accessor: an alias for `Reg<ID51_SPEC>`"]
pub type ID51 = crate::Reg<id51::ID51_SPEC>;
#[doc = "Message Buffer 51 ID Register"]
pub mod id51;
#[doc = "WORD051 (rw) register accessor: an alias for `Reg<WORD051_SPEC>`"]
pub type WORD051 = crate::Reg<word051::WORD051_SPEC>;
#[doc = "Message Buffer 51 WORD0 Register"]
pub mod word051;
#[doc = "WORD151 (rw) register accessor: an alias for `Reg<WORD151_SPEC>`"]
pub type WORD151 = crate::Reg<word151::WORD151_SPEC>;
#[doc = "Message Buffer 51 WORD1 Register"]
pub mod word151;
#[doc = "CS52 (rw) register accessor: an alias for `Reg<CS52_SPEC>`"]
pub type CS52 = crate::Reg<cs52::CS52_SPEC>;
#[doc = "Message Buffer 52 CS Register"]
pub mod cs52;
#[doc = "ID52 (rw) register accessor: an alias for `Reg<ID52_SPEC>`"]
pub type ID52 = crate::Reg<id52::ID52_SPEC>;
#[doc = "Message Buffer 52 ID Register"]
pub mod id52;
#[doc = "WORD052 (rw) register accessor: an alias for `Reg<WORD052_SPEC>`"]
pub type WORD052 = crate::Reg<word052::WORD052_SPEC>;
#[doc = "Message Buffer 52 WORD0 Register"]
pub mod word052;
#[doc = "WORD152 (rw) register accessor: an alias for `Reg<WORD152_SPEC>`"]
pub type WORD152 = crate::Reg<word152::WORD152_SPEC>;
#[doc = "Message Buffer 52 WORD1 Register"]
pub mod word152;
#[doc = "CS53 (rw) register accessor: an alias for `Reg<CS53_SPEC>`"]
pub type CS53 = crate::Reg<cs53::CS53_SPEC>;
#[doc = "Message Buffer 53 CS Register"]
pub mod cs53;
#[doc = "ID53 (rw) register accessor: an alias for `Reg<ID53_SPEC>`"]
pub type ID53 = crate::Reg<id53::ID53_SPEC>;
#[doc = "Message Buffer 53 ID Register"]
pub mod id53;
#[doc = "WORD053 (rw) register accessor: an alias for `Reg<WORD053_SPEC>`"]
pub type WORD053 = crate::Reg<word053::WORD053_SPEC>;
#[doc = "Message Buffer 53 WORD0 Register"]
pub mod word053;
#[doc = "WORD153 (rw) register accessor: an alias for `Reg<WORD153_SPEC>`"]
pub type WORD153 = crate::Reg<word153::WORD153_SPEC>;
#[doc = "Message Buffer 53 WORD1 Register"]
pub mod word153;
#[doc = "CS54 (rw) register accessor: an alias for `Reg<CS54_SPEC>`"]
pub type CS54 = crate::Reg<cs54::CS54_SPEC>;
#[doc = "Message Buffer 54 CS Register"]
pub mod cs54;
#[doc = "ID54 (rw) register accessor: an alias for `Reg<ID54_SPEC>`"]
pub type ID54 = crate::Reg<id54::ID54_SPEC>;
#[doc = "Message Buffer 54 ID Register"]
pub mod id54;
#[doc = "WORD054 (rw) register accessor: an alias for `Reg<WORD054_SPEC>`"]
pub type WORD054 = crate::Reg<word054::WORD054_SPEC>;
#[doc = "Message Buffer 54 WORD0 Register"]
pub mod word054;
#[doc = "WORD154 (rw) register accessor: an alias for `Reg<WORD154_SPEC>`"]
pub type WORD154 = crate::Reg<word154::WORD154_SPEC>;
#[doc = "Message Buffer 54 WORD1 Register"]
pub mod word154;
#[doc = "CS55 (rw) register accessor: an alias for `Reg<CS55_SPEC>`"]
pub type CS55 = crate::Reg<cs55::CS55_SPEC>;
#[doc = "Message Buffer 55 CS Register"]
pub mod cs55;
#[doc = "ID55 (rw) register accessor: an alias for `Reg<ID55_SPEC>`"]
pub type ID55 = crate::Reg<id55::ID55_SPEC>;
#[doc = "Message Buffer 55 ID Register"]
pub mod id55;
#[doc = "WORD055 (rw) register accessor: an alias for `Reg<WORD055_SPEC>`"]
pub type WORD055 = crate::Reg<word055::WORD055_SPEC>;
#[doc = "Message Buffer 55 WORD0 Register"]
pub mod word055;
#[doc = "WORD155 (rw) register accessor: an alias for `Reg<WORD155_SPEC>`"]
pub type WORD155 = crate::Reg<word155::WORD155_SPEC>;
#[doc = "Message Buffer 55 WORD1 Register"]
pub mod word155;
#[doc = "CS56 (rw) register accessor: an alias for `Reg<CS56_SPEC>`"]
pub type CS56 = crate::Reg<cs56::CS56_SPEC>;
#[doc = "Message Buffer 56 CS Register"]
pub mod cs56;
#[doc = "ID56 (rw) register accessor: an alias for `Reg<ID56_SPEC>`"]
pub type ID56 = crate::Reg<id56::ID56_SPEC>;
#[doc = "Message Buffer 56 ID Register"]
pub mod id56;
#[doc = "WORD056 (rw) register accessor: an alias for `Reg<WORD056_SPEC>`"]
pub type WORD056 = crate::Reg<word056::WORD056_SPEC>;
#[doc = "Message Buffer 56 WORD0 Register"]
pub mod word056;
#[doc = "WORD156 (rw) register accessor: an alias for `Reg<WORD156_SPEC>`"]
pub type WORD156 = crate::Reg<word156::WORD156_SPEC>;
#[doc = "Message Buffer 56 WORD1 Register"]
pub mod word156;
#[doc = "CS57 (rw) register accessor: an alias for `Reg<CS57_SPEC>`"]
pub type CS57 = crate::Reg<cs57::CS57_SPEC>;
#[doc = "Message Buffer 57 CS Register"]
pub mod cs57;
#[doc = "ID57 (rw) register accessor: an alias for `Reg<ID57_SPEC>`"]
pub type ID57 = crate::Reg<id57::ID57_SPEC>;
#[doc = "Message Buffer 57 ID Register"]
pub mod id57;
#[doc = "WORD057 (rw) register accessor: an alias for `Reg<WORD057_SPEC>`"]
pub type WORD057 = crate::Reg<word057::WORD057_SPEC>;
#[doc = "Message Buffer 57 WORD0 Register"]
pub mod word057;
#[doc = "WORD157 (rw) register accessor: an alias for `Reg<WORD157_SPEC>`"]
pub type WORD157 = crate::Reg<word157::WORD157_SPEC>;
#[doc = "Message Buffer 57 WORD1 Register"]
pub mod word157;
#[doc = "CS58 (rw) register accessor: an alias for `Reg<CS58_SPEC>`"]
pub type CS58 = crate::Reg<cs58::CS58_SPEC>;
#[doc = "Message Buffer 58 CS Register"]
pub mod cs58;
#[doc = "ID58 (rw) register accessor: an alias for `Reg<ID58_SPEC>`"]
pub type ID58 = crate::Reg<id58::ID58_SPEC>;
#[doc = "Message Buffer 58 ID Register"]
pub mod id58;
#[doc = "WORD058 (rw) register accessor: an alias for `Reg<WORD058_SPEC>`"]
pub type WORD058 = crate::Reg<word058::WORD058_SPEC>;
#[doc = "Message Buffer 58 WORD0 Register"]
pub mod word058;
#[doc = "WORD158 (rw) register accessor: an alias for `Reg<WORD158_SPEC>`"]
pub type WORD158 = crate::Reg<word158::WORD158_SPEC>;
#[doc = "Message Buffer 58 WORD1 Register"]
pub mod word158;
#[doc = "CS59 (rw) register accessor: an alias for `Reg<CS59_SPEC>`"]
pub type CS59 = crate::Reg<cs59::CS59_SPEC>;
#[doc = "Message Buffer 59 CS Register"]
pub mod cs59;
#[doc = "ID59 (rw) register accessor: an alias for `Reg<ID59_SPEC>`"]
pub type ID59 = crate::Reg<id59::ID59_SPEC>;
#[doc = "Message Buffer 59 ID Register"]
pub mod id59;
#[doc = "WORD059 (rw) register accessor: an alias for `Reg<WORD059_SPEC>`"]
pub type WORD059 = crate::Reg<word059::WORD059_SPEC>;
#[doc = "Message Buffer 59 WORD0 Register"]
pub mod word059;
#[doc = "WORD159 (rw) register accessor: an alias for `Reg<WORD159_SPEC>`"]
pub type WORD159 = crate::Reg<word159::WORD159_SPEC>;
#[doc = "Message Buffer 59 WORD1 Register"]
pub mod word159;
#[doc = "CS60 (rw) register accessor: an alias for `Reg<CS60_SPEC>`"]
pub type CS60 = crate::Reg<cs60::CS60_SPEC>;
#[doc = "Message Buffer 60 CS Register"]
pub mod cs60;
#[doc = "ID60 (rw) register accessor: an alias for `Reg<ID60_SPEC>`"]
pub type ID60 = crate::Reg<id60::ID60_SPEC>;
#[doc = "Message Buffer 60 ID Register"]
pub mod id60;
#[doc = "WORD060 (rw) register accessor: an alias for `Reg<WORD060_SPEC>`"]
pub type WORD060 = crate::Reg<word060::WORD060_SPEC>;
#[doc = "Message Buffer 60 WORD0 Register"]
pub mod word060;
#[doc = "WORD160 (rw) register accessor: an alias for `Reg<WORD160_SPEC>`"]
pub type WORD160 = crate::Reg<word160::WORD160_SPEC>;
#[doc = "Message Buffer 60 WORD1 Register"]
pub mod word160;
#[doc = "CS61 (rw) register accessor: an alias for `Reg<CS61_SPEC>`"]
pub type CS61 = crate::Reg<cs61::CS61_SPEC>;
#[doc = "Message Buffer 61 CS Register"]
pub mod cs61;
#[doc = "ID61 (rw) register accessor: an alias for `Reg<ID61_SPEC>`"]
pub type ID61 = crate::Reg<id61::ID61_SPEC>;
#[doc = "Message Buffer 61 ID Register"]
pub mod id61;
#[doc = "WORD061 (rw) register accessor: an alias for `Reg<WORD061_SPEC>`"]
pub type WORD061 = crate::Reg<word061::WORD061_SPEC>;
#[doc = "Message Buffer 61 WORD0 Register"]
pub mod word061;
#[doc = "WORD161 (rw) register accessor: an alias for `Reg<WORD161_SPEC>`"]
pub type WORD161 = crate::Reg<word161::WORD161_SPEC>;
#[doc = "Message Buffer 61 WORD1 Register"]
pub mod word161;
#[doc = "CS62 (rw) register accessor: an alias for `Reg<CS62_SPEC>`"]
pub type CS62 = crate::Reg<cs62::CS62_SPEC>;
#[doc = "Message Buffer 62 CS Register"]
pub mod cs62;
#[doc = "ID62 (rw) register accessor: an alias for `Reg<ID62_SPEC>`"]
pub type ID62 = crate::Reg<id62::ID62_SPEC>;
#[doc = "Message Buffer 62 ID Register"]
pub mod id62;
#[doc = "WORD062 (rw) register accessor: an alias for `Reg<WORD062_SPEC>`"]
pub type WORD062 = crate::Reg<word062::WORD062_SPEC>;
#[doc = "Message Buffer 62 WORD0 Register"]
pub mod word062;
#[doc = "WORD162 (rw) register accessor: an alias for `Reg<WORD162_SPEC>`"]
pub type WORD162 = crate::Reg<word162::WORD162_SPEC>;
#[doc = "Message Buffer 62 WORD1 Register"]
pub mod word162;
#[doc = "CS63 (rw) register accessor: an alias for `Reg<CS63_SPEC>`"]
pub type CS63 = crate::Reg<cs63::CS63_SPEC>;
#[doc = "Message Buffer 63 CS Register"]
pub mod cs63;
#[doc = "ID63 (rw) register accessor: an alias for `Reg<ID63_SPEC>`"]
pub type ID63 = crate::Reg<id63::ID63_SPEC>;
#[doc = "Message Buffer 63 ID Register"]
pub mod id63;
#[doc = "WORD063 (rw) register accessor: an alias for `Reg<WORD063_SPEC>`"]
pub type WORD063 = crate::Reg<word063::WORD063_SPEC>;
#[doc = "Message Buffer 63 WORD0 Register"]
pub mod word063;
#[doc = "WORD163 (rw) register accessor: an alias for `Reg<WORD163_SPEC>`"]
pub type WORD163 = crate::Reg<word163::WORD163_SPEC>;
#[doc = "Message Buffer 63 WORD1 Register"]
pub mod word163;
#[doc = "RXIMR (rw) register accessor: an alias for `Reg<RXIMR_SPEC>`"]
pub type RXIMR = crate::Reg<rximr::RXIMR_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
#[doc = "GFWR (rw) register accessor: an alias for `Reg<GFWR_SPEC>`"]
pub type GFWR = crate::Reg<gfwr::GFWR_SPEC>;
#[doc = "Glitch Filter Width Registers"]
pub mod gfwr;
