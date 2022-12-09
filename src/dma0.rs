#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub cr: CR,
    #[doc = "0x04 - Error Status"]
    pub es: ES,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Enable Request"]
    pub erq: ERQ,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Enable Error Interrupt"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request"]
    pub cint: CINT,
    _reserved12: [u8; 0x04],
    #[doc = "0x24 - Interrupt Request"]
    pub int: INT,
    _reserved13: [u8; 0x04],
    #[doc = "0x2c - Error"]
    pub err: ERR,
    _reserved14: [u8; 0x04],
    #[doc = "0x34 - Hardware Request Status"]
    pub hrs: HRS,
    _reserved15: [u8; 0x0c],
    #[doc = "0x44 - Enable Asynchronous Request in Stop"]
    pub ears: EARS,
    _reserved16: [u8; 0xb8],
    #[doc = "0x100 - Channel Priority"]
    pub dchpri3: DCHPRI3,
    #[doc = "0x101 - Channel Priority"]
    pub dchpri2: DCHPRI2,
    #[doc = "0x102 - Channel Priority"]
    pub dchpri1: DCHPRI1,
    #[doc = "0x103 - Channel Priority"]
    pub dchpri0: DCHPRI0,
    #[doc = "0x104 - Channel Priority"]
    pub dchpri7: DCHPRI7,
    #[doc = "0x105 - Channel Priority"]
    pub dchpri6: DCHPRI6,
    #[doc = "0x106 - Channel Priority"]
    pub dchpri5: DCHPRI5,
    #[doc = "0x107 - Channel Priority"]
    pub dchpri4: DCHPRI4,
    #[doc = "0x108 - Channel Priority"]
    pub dchpri11: DCHPRI11,
    #[doc = "0x109 - Channel Priority"]
    pub dchpri10: DCHPRI10,
    #[doc = "0x10a - Channel Priority"]
    pub dchpri9: DCHPRI9,
    #[doc = "0x10b - Channel Priority"]
    pub dchpri8: DCHPRI8,
    #[doc = "0x10c - Channel Priority"]
    pub dchpri15: DCHPRI15,
    #[doc = "0x10d - Channel Priority"]
    pub dchpri14: DCHPRI14,
    #[doc = "0x10e - Channel Priority"]
    pub dchpri13: DCHPRI13,
    #[doc = "0x10f - Channel Priority"]
    pub dchpri12: DCHPRI12,
    #[doc = "0x110 - Channel Priority"]
    pub dchpri19: DCHPRI19,
    #[doc = "0x111 - Channel Priority"]
    pub dchpri18: DCHPRI18,
    #[doc = "0x112 - Channel Priority"]
    pub dchpri17: DCHPRI17,
    #[doc = "0x113 - Channel Priority"]
    pub dchpri16: DCHPRI16,
    #[doc = "0x114 - Channel Priority"]
    pub dchpri23: DCHPRI23,
    #[doc = "0x115 - Channel Priority"]
    pub dchpri22: DCHPRI22,
    #[doc = "0x116 - Channel Priority"]
    pub dchpri21: DCHPRI21,
    #[doc = "0x117 - Channel Priority"]
    pub dchpri20: DCHPRI20,
    #[doc = "0x118 - Channel Priority"]
    pub dchpri27: DCHPRI27,
    #[doc = "0x119 - Channel Priority"]
    pub dchpri26: DCHPRI26,
    #[doc = "0x11a - Channel Priority"]
    pub dchpri25: DCHPRI25,
    #[doc = "0x11b - Channel Priority"]
    pub dchpri24: DCHPRI24,
    #[doc = "0x11c - Channel Priority"]
    pub dchpri31: DCHPRI31,
    #[doc = "0x11d - Channel Priority"]
    pub dchpri30: DCHPRI30,
    #[doc = "0x11e - Channel Priority"]
    pub dchpri29: DCHPRI29,
    #[doc = "0x11f - Channel Priority"]
    pub dchpri28: DCHPRI28,
    _reserved48: [u8; 0x0ee0],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD0_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD0_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD0_ATTR,
    _reserved_51_tcd_nbytes_tcd0_nbytes: [u8; 0x04],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD0_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD0_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD0_DOFF,
    _reserved_55_tcd_citer_elink_tcd0_citer: [u8; 0x02],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD0_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD0_CSR,
    _reserved_58_tcd_biter_elink_tcd0_biter: [u8; 0x02],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD1_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD1_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD1_ATTR,
    _reserved_62_tcd_nbytes_tcd1_nbytes: [u8; 0x04],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD1_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD1_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD1_DOFF,
    _reserved_66_tcd_citer_elink_tcd1_citer: [u8; 0x02],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD1_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD1_CSR,
    _reserved_69_tcd_biter_elink_tcd1_biter: [u8; 0x02],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD2_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD2_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD2_ATTR,
    _reserved_73_tcd_nbytes_tcd2_nbytes: [u8; 0x04],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD2_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD2_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD2_DOFF,
    _reserved_77_tcd_citer_elink_tcd2_citer: [u8; 0x02],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD2_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD2_CSR,
    _reserved_80_tcd_biter_elink_tcd2_biter: [u8; 0x02],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD3_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD3_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD3_ATTR,
    _reserved_84_tcd_nbytes_tcd3_nbytes: [u8; 0x04],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD3_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD3_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD3_DOFF,
    _reserved_88_tcd_citer_elink_tcd3_citer: [u8; 0x02],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD3_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD3_CSR,
    _reserved_91_tcd_biter_elink_tcd3_biter: [u8; 0x02],
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD4_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD4_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD4_ATTR,
    _reserved_95_tcd_nbytes_tcd4_nbytes: [u8; 0x04],
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD4_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD4_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD4_DOFF,
    _reserved_99_tcd_citer_elink_tcd4_citer: [u8; 0x02],
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD4_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD4_CSR,
    _reserved_102_tcd_biter_elink_tcd4_biter: [u8; 0x02],
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD5_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD5_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD5_ATTR,
    _reserved_106_tcd_nbytes_tcd5_nbytes: [u8; 0x04],
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD5_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD5_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD5_DOFF,
    _reserved_110_tcd_citer_elink_tcd5_citer: [u8; 0x02],
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD5_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD5_CSR,
    _reserved_113_tcd_biter_elink_tcd5_biter: [u8; 0x02],
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD6_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD6_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD6_ATTR,
    _reserved_117_tcd_nbytes_tcd6_nbytes: [u8; 0x04],
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD6_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD6_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD6_DOFF,
    _reserved_121_tcd_citer_elink_tcd6_citer: [u8; 0x02],
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD6_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD6_CSR,
    _reserved_124_tcd_biter_elink_tcd6_biter: [u8; 0x02],
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD7_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD7_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD7_ATTR,
    _reserved_128_tcd_nbytes_tcd7_nbytes: [u8; 0x04],
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD7_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD7_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD7_DOFF,
    _reserved_132_tcd_citer_elink_tcd7_citer: [u8; 0x02],
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD7_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD7_CSR,
    _reserved_135_tcd_biter_elink_tcd7_biter: [u8; 0x02],
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: TCD8_SADDR,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: TCD8_SOFF,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: TCD8_ATTR,
    _reserved_139_tcd_nbytes_tcd8_nbytes: [u8; 0x04],
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: TCD8_SLAST,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: TCD8_DADDR,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: TCD8_DOFF,
    _reserved_143_tcd_citer_elink_tcd8_citer: [u8; 0x02],
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD8_DLASTSGA,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: TCD8_CSR,
    _reserved_146_tcd_biter_elink_tcd8_biter: [u8; 0x02],
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: TCD9_SADDR,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: TCD9_SOFF,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: TCD9_ATTR,
    _reserved_150_tcd_nbytes_tcd9_nbytes: [u8; 0x04],
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: TCD9_SLAST,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: TCD9_DADDR,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: TCD9_DOFF,
    _reserved_154_tcd_citer_elink_tcd9_citer: [u8; 0x02],
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD9_DLASTSGA,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: TCD9_CSR,
    _reserved_157_tcd_biter_elink_tcd9_biter: [u8; 0x02],
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: TCD10_SADDR,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: TCD10_SOFF,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: TCD10_ATTR,
    _reserved_161_tcd_nbytes_tcd10_nbytes: [u8; 0x04],
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: TCD10_SLAST,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: TCD10_DADDR,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: TCD10_DOFF,
    _reserved_165_tcd_citer_elink_tcd10_citer: [u8; 0x02],
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD10_DLASTSGA,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: TCD10_CSR,
    _reserved_168_tcd_biter_elink_tcd10_biter: [u8; 0x02],
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: TCD11_SADDR,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: TCD11_SOFF,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: TCD11_ATTR,
    _reserved_172_tcd_nbytes_tcd11_nbytes: [u8; 0x04],
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: TCD11_SLAST,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: TCD11_DADDR,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: TCD11_DOFF,
    _reserved_176_tcd_citer_elink_tcd11_citer: [u8; 0x02],
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD11_DLASTSGA,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: TCD11_CSR,
    _reserved_179_tcd_biter_elink_tcd11_biter: [u8; 0x02],
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: TCD12_SADDR,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: TCD12_SOFF,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: TCD12_ATTR,
    _reserved_183_tcd_nbytes_tcd12_nbytes: [u8; 0x04],
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: TCD12_SLAST,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: TCD12_DADDR,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: TCD12_DOFF,
    _reserved_187_tcd_citer_elink_tcd12_citer: [u8; 0x02],
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD12_DLASTSGA,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: TCD12_CSR,
    _reserved_190_tcd_biter_elink_tcd12_biter: [u8; 0x02],
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: TCD13_SADDR,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: TCD13_SOFF,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: TCD13_ATTR,
    _reserved_194_tcd_nbytes_tcd13_nbytes: [u8; 0x04],
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: TCD13_SLAST,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: TCD13_DADDR,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: TCD13_DOFF,
    _reserved_198_tcd_citer_elink_tcd13_citer: [u8; 0x02],
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD13_DLASTSGA,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: TCD13_CSR,
    _reserved_201_tcd_biter_elink_tcd13_biter: [u8; 0x02],
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: TCD14_SADDR,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: TCD14_SOFF,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: TCD14_ATTR,
    _reserved_205_tcd_nbytes_tcd14_nbytes: [u8; 0x04],
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: TCD14_SLAST,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: TCD14_DADDR,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: TCD14_DOFF,
    _reserved_209_tcd_citer_elink_tcd14_citer: [u8; 0x02],
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD14_DLASTSGA,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: TCD14_CSR,
    _reserved_212_tcd_biter_elink_tcd14_biter: [u8; 0x02],
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: TCD15_SADDR,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: TCD15_SOFF,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: TCD15_ATTR,
    _reserved_216_tcd_nbytes_tcd15_nbytes: [u8; 0x04],
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: TCD15_SLAST,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: TCD15_DADDR,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: TCD15_DOFF,
    _reserved_220_tcd_citer_elink_tcd15_citer: [u8; 0x02],
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD15_DLASTSGA,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: TCD15_CSR,
    _reserved_223_tcd_biter_elink_tcd15_biter: [u8; 0x02],
    #[doc = "0x1200 - TCD Source Address"]
    pub tcd16_saddr: TCD16_SADDR,
    #[doc = "0x1204 - TCD Signed Source Address Offset"]
    pub tcd16_soff: TCD16_SOFF,
    #[doc = "0x1206 - TCD Transfer Attributes"]
    pub tcd16_attr: TCD16_ATTR,
    _reserved_227_tcd_nbytes_tcd16_nbytes: [u8; 0x04],
    #[doc = "0x120c - TCD Last Source Address Adjustment"]
    pub tcd16_slast: TCD16_SLAST,
    #[doc = "0x1210 - TCD Destination Address"]
    pub tcd16_daddr: TCD16_DADDR,
    #[doc = "0x1214 - TCD Signed Destination Address Offset"]
    pub tcd16_doff: TCD16_DOFF,
    _reserved_231_tcd_citer_elink_tcd16_citer: [u8; 0x02],
    #[doc = "0x1218 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd16_dlastsga: TCD16_DLASTSGA,
    #[doc = "0x121c - TCD Control and Status"]
    pub tcd16_csr: TCD16_CSR,
    _reserved_234_tcd_biter_elink_tcd16_biter: [u8; 0x02],
    #[doc = "0x1220 - TCD Source Address"]
    pub tcd17_saddr: TCD17_SADDR,
    #[doc = "0x1224 - TCD Signed Source Address Offset"]
    pub tcd17_soff: TCD17_SOFF,
    #[doc = "0x1226 - TCD Transfer Attributes"]
    pub tcd17_attr: TCD17_ATTR,
    _reserved_238_tcd_nbytes_tcd17_nbytes: [u8; 0x04],
    #[doc = "0x122c - TCD Last Source Address Adjustment"]
    pub tcd17_slast: TCD17_SLAST,
    #[doc = "0x1230 - TCD Destination Address"]
    pub tcd17_daddr: TCD17_DADDR,
    #[doc = "0x1234 - TCD Signed Destination Address Offset"]
    pub tcd17_doff: TCD17_DOFF,
    _reserved_242_tcd_citer_elink_tcd17_citer: [u8; 0x02],
    #[doc = "0x1238 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd17_dlastsga: TCD17_DLASTSGA,
    #[doc = "0x123c - TCD Control and Status"]
    pub tcd17_csr: TCD17_CSR,
    _reserved_245_tcd_biter_elink_tcd17_biter: [u8; 0x02],
    #[doc = "0x1240 - TCD Source Address"]
    pub tcd18_saddr: TCD18_SADDR,
    #[doc = "0x1244 - TCD Signed Source Address Offset"]
    pub tcd18_soff: TCD18_SOFF,
    #[doc = "0x1246 - TCD Transfer Attributes"]
    pub tcd18_attr: TCD18_ATTR,
    _reserved_249_tcd_nbytes_tcd18_nbytes: [u8; 0x04],
    #[doc = "0x124c - TCD Last Source Address Adjustment"]
    pub tcd18_slast: TCD18_SLAST,
    #[doc = "0x1250 - TCD Destination Address"]
    pub tcd18_daddr: TCD18_DADDR,
    #[doc = "0x1254 - TCD Signed Destination Address Offset"]
    pub tcd18_doff: TCD18_DOFF,
    _reserved_253_tcd_citer_elink_tcd18_citer: [u8; 0x02],
    #[doc = "0x1258 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd18_dlastsga: TCD18_DLASTSGA,
    #[doc = "0x125c - TCD Control and Status"]
    pub tcd18_csr: TCD18_CSR,
    _reserved_256_tcd_biter_elink_tcd18_biter: [u8; 0x02],
    #[doc = "0x1260 - TCD Source Address"]
    pub tcd19_saddr: TCD19_SADDR,
    #[doc = "0x1264 - TCD Signed Source Address Offset"]
    pub tcd19_soff: TCD19_SOFF,
    #[doc = "0x1266 - TCD Transfer Attributes"]
    pub tcd19_attr: TCD19_ATTR,
    _reserved_260_tcd_nbytes_tcd19_nbytes: [u8; 0x04],
    #[doc = "0x126c - TCD Last Source Address Adjustment"]
    pub tcd19_slast: TCD19_SLAST,
    #[doc = "0x1270 - TCD Destination Address"]
    pub tcd19_daddr: TCD19_DADDR,
    #[doc = "0x1274 - TCD Signed Destination Address Offset"]
    pub tcd19_doff: TCD19_DOFF,
    _reserved_264_tcd_citer_elink_tcd19_citer: [u8; 0x02],
    #[doc = "0x1278 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd19_dlastsga: TCD19_DLASTSGA,
    #[doc = "0x127c - TCD Control and Status"]
    pub tcd19_csr: TCD19_CSR,
    _reserved_267_tcd_biter_elink_tcd19_biter: [u8; 0x02],
    #[doc = "0x1280 - TCD Source Address"]
    pub tcd20_saddr: TCD20_SADDR,
    #[doc = "0x1284 - TCD Signed Source Address Offset"]
    pub tcd20_soff: TCD20_SOFF,
    #[doc = "0x1286 - TCD Transfer Attributes"]
    pub tcd20_attr: TCD20_ATTR,
    _reserved_271_tcd_nbytes_tcd20_nbytes: [u8; 0x04],
    #[doc = "0x128c - TCD Last Source Address Adjustment"]
    pub tcd20_slast: TCD20_SLAST,
    #[doc = "0x1290 - TCD Destination Address"]
    pub tcd20_daddr: TCD20_DADDR,
    #[doc = "0x1294 - TCD Signed Destination Address Offset"]
    pub tcd20_doff: TCD20_DOFF,
    _reserved_275_tcd_citer_elink_tcd20_citer: [u8; 0x02],
    #[doc = "0x1298 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd20_dlastsga: TCD20_DLASTSGA,
    #[doc = "0x129c - TCD Control and Status"]
    pub tcd20_csr: TCD20_CSR,
    _reserved_278_tcd_biter_elink_tcd20_biter: [u8; 0x02],
    #[doc = "0x12a0 - TCD Source Address"]
    pub tcd21_saddr: TCD21_SADDR,
    #[doc = "0x12a4 - TCD Signed Source Address Offset"]
    pub tcd21_soff: TCD21_SOFF,
    #[doc = "0x12a6 - TCD Transfer Attributes"]
    pub tcd21_attr: TCD21_ATTR,
    _reserved_282_tcd_nbytes_tcd21_nbytes: [u8; 0x04],
    #[doc = "0x12ac - TCD Last Source Address Adjustment"]
    pub tcd21_slast: TCD21_SLAST,
    #[doc = "0x12b0 - TCD Destination Address"]
    pub tcd21_daddr: TCD21_DADDR,
    #[doc = "0x12b4 - TCD Signed Destination Address Offset"]
    pub tcd21_doff: TCD21_DOFF,
    _reserved_286_tcd_citer_elink_tcd21_citer: [u8; 0x02],
    #[doc = "0x12b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd21_dlastsga: TCD21_DLASTSGA,
    #[doc = "0x12bc - TCD Control and Status"]
    pub tcd21_csr: TCD21_CSR,
    _reserved_289_tcd_biter_elink_tcd21_biter: [u8; 0x02],
    #[doc = "0x12c0 - TCD Source Address"]
    pub tcd22_saddr: TCD22_SADDR,
    #[doc = "0x12c4 - TCD Signed Source Address Offset"]
    pub tcd22_soff: TCD22_SOFF,
    #[doc = "0x12c6 - TCD Transfer Attributes"]
    pub tcd22_attr: TCD22_ATTR,
    _reserved_293_tcd_nbytes_tcd22_nbytes: [u8; 0x04],
    #[doc = "0x12cc - TCD Last Source Address Adjustment"]
    pub tcd22_slast: TCD22_SLAST,
    #[doc = "0x12d0 - TCD Destination Address"]
    pub tcd22_daddr: TCD22_DADDR,
    #[doc = "0x12d4 - TCD Signed Destination Address Offset"]
    pub tcd22_doff: TCD22_DOFF,
    _reserved_297_tcd_citer_elink_tcd22_citer: [u8; 0x02],
    #[doc = "0x12d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd22_dlastsga: TCD22_DLASTSGA,
    #[doc = "0x12dc - TCD Control and Status"]
    pub tcd22_csr: TCD22_CSR,
    _reserved_300_tcd_biter_elink_tcd22_biter: [u8; 0x02],
    #[doc = "0x12e0 - TCD Source Address"]
    pub tcd23_saddr: TCD23_SADDR,
    #[doc = "0x12e4 - TCD Signed Source Address Offset"]
    pub tcd23_soff: TCD23_SOFF,
    #[doc = "0x12e6 - TCD Transfer Attributes"]
    pub tcd23_attr: TCD23_ATTR,
    _reserved_304_tcd_nbytes_tcd23_nbytes: [u8; 0x04],
    #[doc = "0x12ec - TCD Last Source Address Adjustment"]
    pub tcd23_slast: TCD23_SLAST,
    #[doc = "0x12f0 - TCD Destination Address"]
    pub tcd23_daddr: TCD23_DADDR,
    #[doc = "0x12f4 - TCD Signed Destination Address Offset"]
    pub tcd23_doff: TCD23_DOFF,
    _reserved_308_tcd_citer_elink_tcd23_citer: [u8; 0x02],
    #[doc = "0x12f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd23_dlastsga: TCD23_DLASTSGA,
    #[doc = "0x12fc - TCD Control and Status"]
    pub tcd23_csr: TCD23_CSR,
    _reserved_311_tcd_biter_elink_tcd23_biter: [u8; 0x02],
    #[doc = "0x1300 - TCD Source Address"]
    pub tcd24_saddr: TCD24_SADDR,
    #[doc = "0x1304 - TCD Signed Source Address Offset"]
    pub tcd24_soff: TCD24_SOFF,
    #[doc = "0x1306 - TCD Transfer Attributes"]
    pub tcd24_attr: TCD24_ATTR,
    _reserved_315_tcd_nbytes_tcd24_nbytes: [u8; 0x04],
    #[doc = "0x130c - TCD Last Source Address Adjustment"]
    pub tcd24_slast: TCD24_SLAST,
    #[doc = "0x1310 - TCD Destination Address"]
    pub tcd24_daddr: TCD24_DADDR,
    #[doc = "0x1314 - TCD Signed Destination Address Offset"]
    pub tcd24_doff: TCD24_DOFF,
    _reserved_319_tcd_citer_elink_tcd24_citer: [u8; 0x02],
    #[doc = "0x1318 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd24_dlastsga: TCD24_DLASTSGA,
    #[doc = "0x131c - TCD Control and Status"]
    pub tcd24_csr: TCD24_CSR,
    _reserved_322_tcd_biter_elink_tcd24_biter: [u8; 0x02],
    #[doc = "0x1320 - TCD Source Address"]
    pub tcd25_saddr: TCD25_SADDR,
    #[doc = "0x1324 - TCD Signed Source Address Offset"]
    pub tcd25_soff: TCD25_SOFF,
    #[doc = "0x1326 - TCD Transfer Attributes"]
    pub tcd25_attr: TCD25_ATTR,
    _reserved_326_tcd_nbytes_tcd25_nbytes: [u8; 0x04],
    #[doc = "0x132c - TCD Last Source Address Adjustment"]
    pub tcd25_slast: TCD25_SLAST,
    #[doc = "0x1330 - TCD Destination Address"]
    pub tcd25_daddr: TCD25_DADDR,
    #[doc = "0x1334 - TCD Signed Destination Address Offset"]
    pub tcd25_doff: TCD25_DOFF,
    _reserved_330_tcd_citer_elink_tcd25_citer: [u8; 0x02],
    #[doc = "0x1338 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd25_dlastsga: TCD25_DLASTSGA,
    #[doc = "0x133c - TCD Control and Status"]
    pub tcd25_csr: TCD25_CSR,
    _reserved_333_tcd_biter_elink_tcd25_biter: [u8; 0x02],
    #[doc = "0x1340 - TCD Source Address"]
    pub tcd26_saddr: TCD26_SADDR,
    #[doc = "0x1344 - TCD Signed Source Address Offset"]
    pub tcd26_soff: TCD26_SOFF,
    #[doc = "0x1346 - TCD Transfer Attributes"]
    pub tcd26_attr: TCD26_ATTR,
    _reserved_337_tcd_nbytes_tcd26_nbytes: [u8; 0x04],
    #[doc = "0x134c - TCD Last Source Address Adjustment"]
    pub tcd26_slast: TCD26_SLAST,
    #[doc = "0x1350 - TCD Destination Address"]
    pub tcd26_daddr: TCD26_DADDR,
    #[doc = "0x1354 - TCD Signed Destination Address Offset"]
    pub tcd26_doff: TCD26_DOFF,
    _reserved_341_tcd_citer_elink_tcd26_citer: [u8; 0x02],
    #[doc = "0x1358 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd26_dlastsga: TCD26_DLASTSGA,
    #[doc = "0x135c - TCD Control and Status"]
    pub tcd26_csr: TCD26_CSR,
    _reserved_344_tcd_biter_elink_tcd26_biter: [u8; 0x02],
    #[doc = "0x1360 - TCD Source Address"]
    pub tcd27_saddr: TCD27_SADDR,
    #[doc = "0x1364 - TCD Signed Source Address Offset"]
    pub tcd27_soff: TCD27_SOFF,
    #[doc = "0x1366 - TCD Transfer Attributes"]
    pub tcd27_attr: TCD27_ATTR,
    _reserved_348_tcd_nbytes_tcd27_nbytes: [u8; 0x04],
    #[doc = "0x136c - TCD Last Source Address Adjustment"]
    pub tcd27_slast: TCD27_SLAST,
    #[doc = "0x1370 - TCD Destination Address"]
    pub tcd27_daddr: TCD27_DADDR,
    #[doc = "0x1374 - TCD Signed Destination Address Offset"]
    pub tcd27_doff: TCD27_DOFF,
    _reserved_352_tcd_citer_elink_tcd27_citer: [u8; 0x02],
    #[doc = "0x1378 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd27_dlastsga: TCD27_DLASTSGA,
    #[doc = "0x137c - TCD Control and Status"]
    pub tcd27_csr: TCD27_CSR,
    _reserved_355_tcd_biter_elink_tcd27_biter: [u8; 0x02],
    #[doc = "0x1380 - TCD Source Address"]
    pub tcd28_saddr: TCD28_SADDR,
    #[doc = "0x1384 - TCD Signed Source Address Offset"]
    pub tcd28_soff: TCD28_SOFF,
    #[doc = "0x1386 - TCD Transfer Attributes"]
    pub tcd28_attr: TCD28_ATTR,
    _reserved_359_tcd_nbytes_tcd28_nbytes: [u8; 0x04],
    #[doc = "0x138c - TCD Last Source Address Adjustment"]
    pub tcd28_slast: TCD28_SLAST,
    #[doc = "0x1390 - TCD Destination Address"]
    pub tcd28_daddr: TCD28_DADDR,
    #[doc = "0x1394 - TCD Signed Destination Address Offset"]
    pub tcd28_doff: TCD28_DOFF,
    _reserved_363_tcd_citer_elink_tcd28_citer: [u8; 0x02],
    #[doc = "0x1398 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd28_dlastsga: TCD28_DLASTSGA,
    #[doc = "0x139c - TCD Control and Status"]
    pub tcd28_csr: TCD28_CSR,
    _reserved_366_tcd_biter_elink_tcd28_biter: [u8; 0x02],
    #[doc = "0x13a0 - TCD Source Address"]
    pub tcd29_saddr: TCD29_SADDR,
    #[doc = "0x13a4 - TCD Signed Source Address Offset"]
    pub tcd29_soff: TCD29_SOFF,
    #[doc = "0x13a6 - TCD Transfer Attributes"]
    pub tcd29_attr: TCD29_ATTR,
    _reserved_370_tcd_nbytes_tcd29_nbytes: [u8; 0x04],
    #[doc = "0x13ac - TCD Last Source Address Adjustment"]
    pub tcd29_slast: TCD29_SLAST,
    #[doc = "0x13b0 - TCD Destination Address"]
    pub tcd29_daddr: TCD29_DADDR,
    #[doc = "0x13b4 - TCD Signed Destination Address Offset"]
    pub tcd29_doff: TCD29_DOFF,
    _reserved_374_tcd_citer_elink_tcd29_citer: [u8; 0x02],
    #[doc = "0x13b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd29_dlastsga: TCD29_DLASTSGA,
    #[doc = "0x13bc - TCD Control and Status"]
    pub tcd29_csr: TCD29_CSR,
    _reserved_377_tcd_biter_elink_tcd29_biter: [u8; 0x02],
    #[doc = "0x13c0 - TCD Source Address"]
    pub tcd30_saddr: TCD30_SADDR,
    #[doc = "0x13c4 - TCD Signed Source Address Offset"]
    pub tcd30_soff: TCD30_SOFF,
    #[doc = "0x13c6 - TCD Transfer Attributes"]
    pub tcd30_attr: TCD30_ATTR,
    _reserved_381_tcd_nbytes_tcd30_nbytes: [u8; 0x04],
    #[doc = "0x13cc - TCD Last Source Address Adjustment"]
    pub tcd30_slast: TCD30_SLAST,
    #[doc = "0x13d0 - TCD Destination Address"]
    pub tcd30_daddr: TCD30_DADDR,
    #[doc = "0x13d4 - TCD Signed Destination Address Offset"]
    pub tcd30_doff: TCD30_DOFF,
    _reserved_385_tcd_citer_elink_tcd30_citer: [u8; 0x02],
    #[doc = "0x13d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd30_dlastsga: TCD30_DLASTSGA,
    #[doc = "0x13dc - TCD Control and Status"]
    pub tcd30_csr: TCD30_CSR,
    _reserved_388_tcd_biter_elink_tcd30_biter: [u8; 0x02],
    #[doc = "0x13e0 - TCD Source Address"]
    pub tcd31_saddr: TCD31_SADDR,
    #[doc = "0x13e4 - TCD Signed Source Address Offset"]
    pub tcd31_soff: TCD31_SOFF,
    #[doc = "0x13e6 - TCD Transfer Attributes"]
    pub tcd31_attr: TCD31_ATTR,
    _reserved_392_tcd_nbytes_tcd31_nbytes: [u8; 0x04],
    #[doc = "0x13ec - TCD Last Source Address Adjustment"]
    pub tcd31_slast: TCD31_SLAST,
    #[doc = "0x13f0 - TCD Destination Address"]
    pub tcd31_daddr: TCD31_DADDR,
    #[doc = "0x13f4 - TCD Signed Destination Address Offset"]
    pub tcd31_doff: TCD31_DOFF,
    _reserved_396_tcd_citer_elink_tcd31_citer: [u8; 0x02],
    #[doc = "0x13f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd31_dlastsga: TCD31_DLASTSGA,
    #[doc = "0x13fc - TCD Control and Status"]
    pub tcd31_csr: TCD31_CSR,
    _reserved_399_tcd_biter_elink_tcd31_biter: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd0_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD0_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd0_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD0_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd0_nbytes_mlno(&self) -> &TCD_NBYTES_TCD0_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd0_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD0_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4118usize).cast() }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd0_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD0_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4118usize).cast() }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd0_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD0_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4126usize).cast() }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd0_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD0_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4126usize).cast() }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd1_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD1_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd1_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD1_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd1_nbytes_mlno(&self) -> &TCD_NBYTES_TCD1_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd1_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD1_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4150usize).cast() }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd1_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD1_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4150usize).cast() }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd1_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD1_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4158usize).cast() }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd1_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD1_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4158usize).cast() }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd2_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD2_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd2_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD2_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd2_nbytes_mlno(&self) -> &TCD_NBYTES_TCD2_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd2_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD2_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4182usize).cast() }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd2_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD2_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4182usize).cast() }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd2_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD2_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4190usize).cast() }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd2_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD2_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4190usize).cast() }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd3_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD3_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd3_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD3_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd3_nbytes_mlno(&self) -> &TCD_NBYTES_TCD3_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd3_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD3_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4214usize).cast() }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd3_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD3_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4214usize).cast() }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd3_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD3_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4222usize).cast() }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd3_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD3_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4222usize).cast() }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd4_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD4_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4232usize).cast() }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd4_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD4_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4232usize).cast() }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd4_nbytes_mlno(&self) -> &TCD_NBYTES_TCD4_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4232usize).cast() }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd4_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD4_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4246usize).cast() }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd4_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD4_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4246usize).cast() }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd4_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD4_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4254usize).cast() }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd4_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD4_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4254usize).cast() }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd5_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD5_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4264usize).cast() }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd5_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD5_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4264usize).cast() }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd5_nbytes_mlno(&self) -> &TCD_NBYTES_TCD5_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4264usize).cast() }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd5_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD5_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4278usize).cast() }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd5_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD5_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4278usize).cast() }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd5_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD5_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4286usize).cast() }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd5_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD5_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4286usize).cast() }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd6_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD6_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4296usize).cast() }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd6_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD6_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4296usize).cast() }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd6_nbytes_mlno(&self) -> &TCD_NBYTES_TCD6_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4296usize).cast() }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd6_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD6_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4310usize).cast() }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd6_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD6_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4310usize).cast() }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd6_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD6_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4318usize).cast() }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd6_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD6_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4318usize).cast() }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd7_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD7_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4328usize).cast() }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd7_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD7_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4328usize).cast() }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd7_nbytes_mlno(&self) -> &TCD_NBYTES_TCD7_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4328usize).cast() }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd7_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD7_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4342usize).cast() }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd7_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD7_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4342usize).cast() }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd7_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD7_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4350usize).cast() }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd7_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD7_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4350usize).cast() }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd8_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD8_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4360usize).cast() }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd8_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD8_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4360usize).cast() }
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd8_nbytes_mlno(&self) -> &TCD_NBYTES_TCD8_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4360usize).cast() }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd8_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD8_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4374usize).cast() }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd8_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD8_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4374usize).cast() }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd8_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD8_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4382usize).cast() }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd8_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD8_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4382usize).cast() }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd9_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD9_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4392usize).cast() }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd9_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD9_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4392usize).cast() }
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd9_nbytes_mlno(&self) -> &TCD_NBYTES_TCD9_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4392usize).cast() }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd9_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD9_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4406usize).cast() }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd9_citer_elinkno(&self) -> &TCD_CITER_ELINK_TCD9_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4406usize).cast() }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd9_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD9_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4414usize).cast() }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd9_biter_elinkno(&self) -> &TCD_BITER_ELINK_TCD9_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4414usize).cast() }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd10_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD10_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4424usize).cast() }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd10_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD10_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4424usize).cast() }
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd10_nbytes_mlno(&self) -> &TCD_NBYTES_TCD10_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4424usize).cast() }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd10_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD10_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4438usize).cast() }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd10_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD10_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4438usize).cast() }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd10_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD10_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4446usize).cast() }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd10_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD10_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4446usize).cast() }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd11_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD11_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4456usize).cast() }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd11_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD11_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4456usize).cast() }
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd11_nbytes_mlno(&self) -> &TCD_NBYTES_TCD11_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4456usize).cast() }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd11_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD11_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4470usize).cast() }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd11_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD11_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4470usize).cast() }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd11_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD11_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4478usize).cast() }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd11_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD11_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4478usize).cast() }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd12_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD12_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4488usize).cast() }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd12_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD12_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4488usize).cast() }
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd12_nbytes_mlno(&self) -> &TCD_NBYTES_TCD12_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4488usize).cast() }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd12_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD12_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4502usize).cast() }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd12_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD12_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4502usize).cast() }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd12_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD12_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4510usize).cast() }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd12_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD12_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4510usize).cast() }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd13_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD13_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4520usize).cast() }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd13_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD13_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4520usize).cast() }
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd13_nbytes_mlno(&self) -> &TCD_NBYTES_TCD13_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4520usize).cast() }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd13_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD13_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4534usize).cast() }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd13_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD13_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4534usize).cast() }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd13_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD13_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4542usize).cast() }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd13_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD13_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4542usize).cast() }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd14_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD14_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4552usize).cast() }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd14_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD14_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4552usize).cast() }
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd14_nbytes_mlno(&self) -> &TCD_NBYTES_TCD14_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4552usize).cast() }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd14_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD14_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4566usize).cast() }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd14_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD14_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4566usize).cast() }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd14_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD14_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4574usize).cast() }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd14_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD14_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4574usize).cast() }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd15_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD15_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4584usize).cast() }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd15_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD15_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4584usize).cast() }
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd15_nbytes_mlno(&self) -> &TCD_NBYTES_TCD15_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4584usize).cast() }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd15_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD15_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4598usize).cast() }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd15_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD15_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4598usize).cast() }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd15_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD15_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4606usize).cast() }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd15_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD15_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4606usize).cast() }
    }
    #[doc = "0x1208 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd16_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD16_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4616usize).cast() }
    }
    #[doc = "0x1208 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd16_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD16_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4616usize).cast() }
    }
    #[doc = "0x1208 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd16_nbytes_mlno(&self) -> &TCD_NBYTES_TCD16_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4616usize).cast() }
    }
    #[doc = "0x1216 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd16_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD16_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4630usize).cast() }
    }
    #[doc = "0x1216 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd16_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD16_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4630usize).cast() }
    }
    #[doc = "0x121e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd16_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD16_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4638usize).cast() }
    }
    #[doc = "0x121e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd16_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD16_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4638usize).cast() }
    }
    #[doc = "0x1228 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd17_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD17_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4648usize).cast() }
    }
    #[doc = "0x1228 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd17_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD17_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4648usize).cast() }
    }
    #[doc = "0x1228 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd17_nbytes_mlno(&self) -> &TCD_NBYTES_TCD17_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4648usize).cast() }
    }
    #[doc = "0x1236 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd17_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD17_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4662usize).cast() }
    }
    #[doc = "0x1236 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd17_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD17_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4662usize).cast() }
    }
    #[doc = "0x123e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd17_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD17_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4670usize).cast() }
    }
    #[doc = "0x123e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd17_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD17_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4670usize).cast() }
    }
    #[doc = "0x1248 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd18_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD18_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4680usize).cast() }
    }
    #[doc = "0x1248 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd18_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD18_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4680usize).cast() }
    }
    #[doc = "0x1248 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd18_nbytes_mlno(&self) -> &TCD_NBYTES_TCD18_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4680usize).cast() }
    }
    #[doc = "0x1256 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd18_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD18_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4694usize).cast() }
    }
    #[doc = "0x1256 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd18_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD18_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4694usize).cast() }
    }
    #[doc = "0x125e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd18_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD18_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4702usize).cast() }
    }
    #[doc = "0x125e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd18_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD18_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4702usize).cast() }
    }
    #[doc = "0x1268 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd19_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD19_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4712usize).cast() }
    }
    #[doc = "0x1268 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd19_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD19_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4712usize).cast() }
    }
    #[doc = "0x1268 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd19_nbytes_mlno(&self) -> &TCD_NBYTES_TCD19_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4712usize).cast() }
    }
    #[doc = "0x1276 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd19_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD19_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4726usize).cast() }
    }
    #[doc = "0x1276 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd19_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD19_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4726usize).cast() }
    }
    #[doc = "0x127e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd19_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD19_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4734usize).cast() }
    }
    #[doc = "0x127e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd19_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD19_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4734usize).cast() }
    }
    #[doc = "0x1288 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd20_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD20_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4744usize).cast() }
    }
    #[doc = "0x1288 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd20_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD20_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4744usize).cast() }
    }
    #[doc = "0x1288 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd20_nbytes_mlno(&self) -> &TCD_NBYTES_TCD20_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4744usize).cast() }
    }
    #[doc = "0x1296 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd20_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD20_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4758usize).cast() }
    }
    #[doc = "0x1296 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd20_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD20_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4758usize).cast() }
    }
    #[doc = "0x129e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd20_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD20_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4766usize).cast() }
    }
    #[doc = "0x129e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd20_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD20_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4766usize).cast() }
    }
    #[doc = "0x12a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd21_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD21_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4776usize).cast() }
    }
    #[doc = "0x12a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd21_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD21_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4776usize).cast() }
    }
    #[doc = "0x12a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd21_nbytes_mlno(&self) -> &TCD_NBYTES_TCD21_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4776usize).cast() }
    }
    #[doc = "0x12b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd21_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD21_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4790usize).cast() }
    }
    #[doc = "0x12b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd21_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD21_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4790usize).cast() }
    }
    #[doc = "0x12be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd21_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD21_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4798usize).cast() }
    }
    #[doc = "0x12be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd21_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD21_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4798usize).cast() }
    }
    #[doc = "0x12c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd22_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD22_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4808usize).cast() }
    }
    #[doc = "0x12c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd22_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD22_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4808usize).cast() }
    }
    #[doc = "0x12c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd22_nbytes_mlno(&self) -> &TCD_NBYTES_TCD22_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4808usize).cast() }
    }
    #[doc = "0x12d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd22_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD22_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4822usize).cast() }
    }
    #[doc = "0x12d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd22_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD22_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4822usize).cast() }
    }
    #[doc = "0x12de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd22_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD22_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4830usize).cast() }
    }
    #[doc = "0x12de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd22_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD22_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4830usize).cast() }
    }
    #[doc = "0x12e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd23_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD23_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4840usize).cast() }
    }
    #[doc = "0x12e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd23_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD23_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4840usize).cast() }
    }
    #[doc = "0x12e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd23_nbytes_mlno(&self) -> &TCD_NBYTES_TCD23_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4840usize).cast() }
    }
    #[doc = "0x12f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd23_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD23_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4854usize).cast() }
    }
    #[doc = "0x12f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd23_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD23_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4854usize).cast() }
    }
    #[doc = "0x12fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd23_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD23_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4862usize).cast() }
    }
    #[doc = "0x12fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd23_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD23_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4862usize).cast() }
    }
    #[doc = "0x1308 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd24_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD24_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4872usize).cast() }
    }
    #[doc = "0x1308 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd24_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD24_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4872usize).cast() }
    }
    #[doc = "0x1308 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd24_nbytes_mlno(&self) -> &TCD_NBYTES_TCD24_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4872usize).cast() }
    }
    #[doc = "0x1316 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd24_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD24_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4886usize).cast() }
    }
    #[doc = "0x1316 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd24_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD24_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4886usize).cast() }
    }
    #[doc = "0x131e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd24_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD24_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4894usize).cast() }
    }
    #[doc = "0x131e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd24_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD24_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4894usize).cast() }
    }
    #[doc = "0x1328 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd25_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD25_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4904usize).cast() }
    }
    #[doc = "0x1328 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd25_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD25_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4904usize).cast() }
    }
    #[doc = "0x1328 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd25_nbytes_mlno(&self) -> &TCD_NBYTES_TCD25_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4904usize).cast() }
    }
    #[doc = "0x1336 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd25_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD25_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4918usize).cast() }
    }
    #[doc = "0x1336 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd25_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD25_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4918usize).cast() }
    }
    #[doc = "0x133e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd25_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD25_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4926usize).cast() }
    }
    #[doc = "0x133e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd25_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD25_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4926usize).cast() }
    }
    #[doc = "0x1348 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd26_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD26_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4936usize).cast() }
    }
    #[doc = "0x1348 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd26_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD26_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4936usize).cast() }
    }
    #[doc = "0x1348 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd26_nbytes_mlno(&self) -> &TCD_NBYTES_TCD26_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4936usize).cast() }
    }
    #[doc = "0x1356 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd26_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD26_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4950usize).cast() }
    }
    #[doc = "0x1356 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd26_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD26_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4950usize).cast() }
    }
    #[doc = "0x135e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd26_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD26_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4958usize).cast() }
    }
    #[doc = "0x135e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd26_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD26_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4958usize).cast() }
    }
    #[doc = "0x1368 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd27_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD27_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4968usize).cast() }
    }
    #[doc = "0x1368 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd27_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD27_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4968usize).cast() }
    }
    #[doc = "0x1368 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd27_nbytes_mlno(&self) -> &TCD_NBYTES_TCD27_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4968usize).cast() }
    }
    #[doc = "0x1376 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd27_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD27_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4982usize).cast() }
    }
    #[doc = "0x1376 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd27_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD27_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4982usize).cast() }
    }
    #[doc = "0x137e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd27_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD27_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4990usize).cast() }
    }
    #[doc = "0x137e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd27_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD27_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4990usize).cast() }
    }
    #[doc = "0x1388 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd28_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD28_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5000usize).cast() }
    }
    #[doc = "0x1388 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd28_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD28_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5000usize).cast() }
    }
    #[doc = "0x1388 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd28_nbytes_mlno(&self) -> &TCD_NBYTES_TCD28_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5000usize).cast() }
    }
    #[doc = "0x1396 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd28_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD28_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5014usize).cast() }
    }
    #[doc = "0x1396 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd28_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD28_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5014usize).cast() }
    }
    #[doc = "0x139e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd28_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD28_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5022usize).cast() }
    }
    #[doc = "0x139e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd28_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD28_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5022usize).cast() }
    }
    #[doc = "0x13a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd29_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD29_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5032usize).cast() }
    }
    #[doc = "0x13a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd29_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD29_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5032usize).cast() }
    }
    #[doc = "0x13a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd29_nbytes_mlno(&self) -> &TCD_NBYTES_TCD29_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5032usize).cast() }
    }
    #[doc = "0x13b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd29_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD29_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5046usize).cast() }
    }
    #[doc = "0x13b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd29_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD29_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5046usize).cast() }
    }
    #[doc = "0x13be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd29_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD29_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5054usize).cast() }
    }
    #[doc = "0x13be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd29_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD29_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5054usize).cast() }
    }
    #[doc = "0x13c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd30_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD30_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5064usize).cast() }
    }
    #[doc = "0x13c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd30_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD30_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5064usize).cast() }
    }
    #[doc = "0x13c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd30_nbytes_mlno(&self) -> &TCD_NBYTES_TCD30_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5064usize).cast() }
    }
    #[doc = "0x13d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd30_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD30_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5078usize).cast() }
    }
    #[doc = "0x13d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd30_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD30_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5078usize).cast() }
    }
    #[doc = "0x13de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd30_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD30_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5086usize).cast() }
    }
    #[doc = "0x13de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd30_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD30_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5086usize).cast() }
    }
    #[doc = "0x13e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd31_nbytes_mloffyes(&self) -> &TCD_NBYTES_TCD31_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5096usize).cast() }
    }
    #[doc = "0x13e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd31_nbytes_mloffno(&self) -> &TCD_NBYTES_TCD31_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5096usize).cast() }
    }
    #[doc = "0x13e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd_nbytes_tcd31_nbytes_mlno(&self) -> &TCD_NBYTES_TCD31_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5096usize).cast() }
    }
    #[doc = "0x13f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd31_citer_elinkyes(
        &self,
    ) -> &TCD_CITER_ELINK_TCD31_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5110usize).cast() }
    }
    #[doc = "0x13f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elink_tcd31_citer_elinkno(
        &self,
    ) -> &TCD_CITER_ELINK_TCD31_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5110usize).cast() }
    }
    #[doc = "0x13fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd31_biter_elinkyes(
        &self,
    ) -> &TCD_BITER_ELINK_TCD31_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(5118usize).cast() }
    }
    #[doc = "0x13fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elink_tcd31_biter_elinkno(
        &self,
    ) -> &TCD_BITER_ELINK_TCD31_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(5118usize).cast() }
    }
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control"]
pub mod cr;
#[doc = "ES (r) register accessor: an alias for `Reg<ES_SPEC>`"]
pub type ES = crate::Reg<es::ES_SPEC>;
#[doc = "Error Status"]
pub mod es;
#[doc = "ERQ (rw) register accessor: an alias for `Reg<ERQ_SPEC>`"]
pub type ERQ = crate::Reg<erq::ERQ_SPEC>;
#[doc = "Enable Request"]
pub mod erq;
#[doc = "EEI (rw) register accessor: an alias for `Reg<EEI_SPEC>`"]
pub type EEI = crate::Reg<eei::EEI_SPEC>;
#[doc = "Enable Error Interrupt"]
pub mod eei;
#[doc = "CEEI (rw) register accessor: an alias for `Reg<CEEI_SPEC>`"]
pub type CEEI = crate::Reg<ceei::CEEI_SPEC>;
#[doc = "Clear Enable Error Interrupt"]
pub mod ceei;
#[doc = "SEEI (rw) register accessor: an alias for `Reg<SEEI_SPEC>`"]
pub type SEEI = crate::Reg<seei::SEEI_SPEC>;
#[doc = "Set Enable Error Interrupt"]
pub mod seei;
#[doc = "CERQ (rw) register accessor: an alias for `Reg<CERQ_SPEC>`"]
pub type CERQ = crate::Reg<cerq::CERQ_SPEC>;
#[doc = "Clear Enable Request"]
pub mod cerq;
#[doc = "SERQ (rw) register accessor: an alias for `Reg<SERQ_SPEC>`"]
pub type SERQ = crate::Reg<serq::SERQ_SPEC>;
#[doc = "Set Enable Request"]
pub mod serq;
#[doc = "CDNE (rw) register accessor: an alias for `Reg<CDNE_SPEC>`"]
pub type CDNE = crate::Reg<cdne::CDNE_SPEC>;
#[doc = "Clear DONE Status Bit"]
pub mod cdne;
#[doc = "SSRT (rw) register accessor: an alias for `Reg<SSRT_SPEC>`"]
pub type SSRT = crate::Reg<ssrt::SSRT_SPEC>;
#[doc = "Set START Bit"]
pub mod ssrt;
#[doc = "CERR (rw) register accessor: an alias for `Reg<CERR_SPEC>`"]
pub type CERR = crate::Reg<cerr::CERR_SPEC>;
#[doc = "Clear Error"]
pub mod cerr;
#[doc = "CINT (rw) register accessor: an alias for `Reg<CINT_SPEC>`"]
pub type CINT = crate::Reg<cint::CINT_SPEC>;
#[doc = "Clear Interrupt Request"]
pub mod cint;
#[doc = "INT (rw) register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Interrupt Request"]
pub mod int;
#[doc = "ERR (rw) register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error"]
pub mod err;
#[doc = "HRS (r) register accessor: an alias for `Reg<HRS_SPEC>`"]
pub type HRS = crate::Reg<hrs::HRS_SPEC>;
#[doc = "Hardware Request Status"]
pub mod hrs;
#[doc = "EARS (rw) register accessor: an alias for `Reg<EARS_SPEC>`"]
pub type EARS = crate::Reg<ears::EARS_SPEC>;
#[doc = "Enable Asynchronous Request in Stop"]
pub mod ears;
#[doc = "DCHPRI3 (rw) register accessor: an alias for `Reg<DCHPRI3_SPEC>`"]
pub type DCHPRI3 = crate::Reg<dchpri3::DCHPRI3_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri3;
#[doc = "DCHPRI2 (rw) register accessor: an alias for `Reg<DCHPRI2_SPEC>`"]
pub type DCHPRI2 = crate::Reg<dchpri2::DCHPRI2_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri2;
#[doc = "DCHPRI1 (rw) register accessor: an alias for `Reg<DCHPRI1_SPEC>`"]
pub type DCHPRI1 = crate::Reg<dchpri1::DCHPRI1_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri1;
#[doc = "DCHPRI0 (rw) register accessor: an alias for `Reg<DCHPRI0_SPEC>`"]
pub type DCHPRI0 = crate::Reg<dchpri0::DCHPRI0_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri0;
#[doc = "DCHPRI7 (rw) register accessor: an alias for `Reg<DCHPRI7_SPEC>`"]
pub type DCHPRI7 = crate::Reg<dchpri7::DCHPRI7_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri7;
#[doc = "DCHPRI6 (rw) register accessor: an alias for `Reg<DCHPRI6_SPEC>`"]
pub type DCHPRI6 = crate::Reg<dchpri6::DCHPRI6_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri6;
#[doc = "DCHPRI5 (rw) register accessor: an alias for `Reg<DCHPRI5_SPEC>`"]
pub type DCHPRI5 = crate::Reg<dchpri5::DCHPRI5_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri5;
#[doc = "DCHPRI4 (rw) register accessor: an alias for `Reg<DCHPRI4_SPEC>`"]
pub type DCHPRI4 = crate::Reg<dchpri4::DCHPRI4_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri4;
#[doc = "DCHPRI11 (rw) register accessor: an alias for `Reg<DCHPRI11_SPEC>`"]
pub type DCHPRI11 = crate::Reg<dchpri11::DCHPRI11_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri11;
#[doc = "DCHPRI10 (rw) register accessor: an alias for `Reg<DCHPRI10_SPEC>`"]
pub type DCHPRI10 = crate::Reg<dchpri10::DCHPRI10_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri10;
#[doc = "DCHPRI9 (rw) register accessor: an alias for `Reg<DCHPRI9_SPEC>`"]
pub type DCHPRI9 = crate::Reg<dchpri9::DCHPRI9_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri9;
#[doc = "DCHPRI8 (rw) register accessor: an alias for `Reg<DCHPRI8_SPEC>`"]
pub type DCHPRI8 = crate::Reg<dchpri8::DCHPRI8_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri8;
#[doc = "DCHPRI15 (rw) register accessor: an alias for `Reg<DCHPRI15_SPEC>`"]
pub type DCHPRI15 = crate::Reg<dchpri15::DCHPRI15_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri15;
#[doc = "DCHPRI14 (rw) register accessor: an alias for `Reg<DCHPRI14_SPEC>`"]
pub type DCHPRI14 = crate::Reg<dchpri14::DCHPRI14_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri14;
#[doc = "DCHPRI13 (rw) register accessor: an alias for `Reg<DCHPRI13_SPEC>`"]
pub type DCHPRI13 = crate::Reg<dchpri13::DCHPRI13_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri13;
#[doc = "DCHPRI12 (rw) register accessor: an alias for `Reg<DCHPRI12_SPEC>`"]
pub type DCHPRI12 = crate::Reg<dchpri12::DCHPRI12_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri12;
#[doc = "DCHPRI19 (rw) register accessor: an alias for `Reg<DCHPRI19_SPEC>`"]
pub type DCHPRI19 = crate::Reg<dchpri19::DCHPRI19_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri19;
#[doc = "DCHPRI18 (rw) register accessor: an alias for `Reg<DCHPRI18_SPEC>`"]
pub type DCHPRI18 = crate::Reg<dchpri18::DCHPRI18_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri18;
#[doc = "DCHPRI17 (rw) register accessor: an alias for `Reg<DCHPRI17_SPEC>`"]
pub type DCHPRI17 = crate::Reg<dchpri17::DCHPRI17_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri17;
#[doc = "DCHPRI16 (rw) register accessor: an alias for `Reg<DCHPRI16_SPEC>`"]
pub type DCHPRI16 = crate::Reg<dchpri16::DCHPRI16_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri16;
#[doc = "DCHPRI23 (rw) register accessor: an alias for `Reg<DCHPRI23_SPEC>`"]
pub type DCHPRI23 = crate::Reg<dchpri23::DCHPRI23_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri23;
#[doc = "DCHPRI22 (rw) register accessor: an alias for `Reg<DCHPRI22_SPEC>`"]
pub type DCHPRI22 = crate::Reg<dchpri22::DCHPRI22_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri22;
#[doc = "DCHPRI21 (rw) register accessor: an alias for `Reg<DCHPRI21_SPEC>`"]
pub type DCHPRI21 = crate::Reg<dchpri21::DCHPRI21_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri21;
#[doc = "DCHPRI20 (rw) register accessor: an alias for `Reg<DCHPRI20_SPEC>`"]
pub type DCHPRI20 = crate::Reg<dchpri20::DCHPRI20_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri20;
#[doc = "DCHPRI27 (rw) register accessor: an alias for `Reg<DCHPRI27_SPEC>`"]
pub type DCHPRI27 = crate::Reg<dchpri27::DCHPRI27_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri27;
#[doc = "DCHPRI26 (rw) register accessor: an alias for `Reg<DCHPRI26_SPEC>`"]
pub type DCHPRI26 = crate::Reg<dchpri26::DCHPRI26_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri26;
#[doc = "DCHPRI25 (rw) register accessor: an alias for `Reg<DCHPRI25_SPEC>`"]
pub type DCHPRI25 = crate::Reg<dchpri25::DCHPRI25_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri25;
#[doc = "DCHPRI24 (rw) register accessor: an alias for `Reg<DCHPRI24_SPEC>`"]
pub type DCHPRI24 = crate::Reg<dchpri24::DCHPRI24_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri24;
#[doc = "DCHPRI31 (rw) register accessor: an alias for `Reg<DCHPRI31_SPEC>`"]
pub type DCHPRI31 = crate::Reg<dchpri31::DCHPRI31_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri31;
#[doc = "DCHPRI30 (rw) register accessor: an alias for `Reg<DCHPRI30_SPEC>`"]
pub type DCHPRI30 = crate::Reg<dchpri30::DCHPRI30_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri30;
#[doc = "DCHPRI29 (rw) register accessor: an alias for `Reg<DCHPRI29_SPEC>`"]
pub type DCHPRI29 = crate::Reg<dchpri29::DCHPRI29_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri29;
#[doc = "DCHPRI28 (rw) register accessor: an alias for `Reg<DCHPRI28_SPEC>`"]
pub type DCHPRI28 = crate::Reg<dchpri28::DCHPRI28_SPEC>;
#[doc = "Channel Priority"]
pub mod dchpri28;
#[doc = "TCD0_SADDR (rw) register accessor: an alias for `Reg<TCD0_SADDR_SPEC>`"]
pub type TCD0_SADDR = crate::Reg<tcd0_saddr::TCD0_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd0_saddr;
#[doc = "TCD0_SOFF (rw) register accessor: an alias for `Reg<TCD0_SOFF_SPEC>`"]
pub type TCD0_SOFF = crate::Reg<tcd0_soff::TCD0_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd0_soff;
#[doc = "TCD0_ATTR (rw) register accessor: an alias for `Reg<TCD0_ATTR_SPEC>`"]
pub type TCD0_ATTR = crate::Reg<tcd0_attr::TCD0_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd0_attr;
#[doc = "TCD_NBYTES_TCD0_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD0_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD0_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd0_nbytes_mlno::TCD_NBYTES_TCD0_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd0_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD0_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD0_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD0_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd0_nbytes_mloffno::TCD_NBYTES_TCD0_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd0_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD0_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD0_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD0_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd0_nbytes_mloffyes::TCD_NBYTES_TCD0_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd0_nbytes_mloffyes;
#[doc = "TCD0_SLAST (rw) register accessor: an alias for `Reg<TCD0_SLAST_SPEC>`"]
pub type TCD0_SLAST = crate::Reg<tcd0_slast::TCD0_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd0_slast;
#[doc = "TCD0_DADDR (rw) register accessor: an alias for `Reg<TCD0_DADDR_SPEC>`"]
pub type TCD0_DADDR = crate::Reg<tcd0_daddr::TCD0_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd0_daddr;
#[doc = "TCD0_DOFF (rw) register accessor: an alias for `Reg<TCD0_DOFF_SPEC>`"]
pub type TCD0_DOFF = crate::Reg<tcd0_doff::TCD0_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd0_doff;
#[doc = "TCD_CITER_ELINK_TCD0_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD0_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD0_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd0_citer_elinkno::TCD_CITER_ELINK_TCD0_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd0_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD0_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD0_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD0_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd0_citer_elinkyes::TCD_CITER_ELINK_TCD0_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd0_citer_elinkyes;
#[doc = "TCD0_DLASTSGA (rw) register accessor: an alias for `Reg<TCD0_DLASTSGA_SPEC>`"]
pub type TCD0_DLASTSGA = crate::Reg<tcd0_dlastsga::TCD0_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd0_dlastsga;
#[doc = "TCD0_CSR (rw) register accessor: an alias for `Reg<TCD0_CSR_SPEC>`"]
pub type TCD0_CSR = crate::Reg<tcd0_csr::TCD0_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd0_csr;
#[doc = "TCD_BITER_ELINK_TCD0_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD0_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD0_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd0_biter_elinkno::TCD_BITER_ELINK_TCD0_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd0_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD0_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD0_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD0_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd0_biter_elinkyes::TCD_BITER_ELINK_TCD0_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd0_biter_elinkyes;
#[doc = "TCD1_SADDR (rw) register accessor: an alias for `Reg<TCD1_SADDR_SPEC>`"]
pub type TCD1_SADDR = crate::Reg<tcd1_saddr::TCD1_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd1_saddr;
#[doc = "TCD1_SOFF (rw) register accessor: an alias for `Reg<TCD1_SOFF_SPEC>`"]
pub type TCD1_SOFF = crate::Reg<tcd1_soff::TCD1_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd1_soff;
#[doc = "TCD1_ATTR (rw) register accessor: an alias for `Reg<TCD1_ATTR_SPEC>`"]
pub type TCD1_ATTR = crate::Reg<tcd1_attr::TCD1_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd1_attr;
#[doc = "TCD_NBYTES_TCD1_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD1_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD1_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd1_nbytes_mlno::TCD_NBYTES_TCD1_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd1_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD1_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD1_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD1_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd1_nbytes_mloffno::TCD_NBYTES_TCD1_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd1_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD1_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD1_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD1_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd1_nbytes_mloffyes::TCD_NBYTES_TCD1_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd1_nbytes_mloffyes;
#[doc = "TCD1_SLAST (rw) register accessor: an alias for `Reg<TCD1_SLAST_SPEC>`"]
pub type TCD1_SLAST = crate::Reg<tcd1_slast::TCD1_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd1_slast;
#[doc = "TCD1_DADDR (rw) register accessor: an alias for `Reg<TCD1_DADDR_SPEC>`"]
pub type TCD1_DADDR = crate::Reg<tcd1_daddr::TCD1_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd1_daddr;
#[doc = "TCD1_DOFF (rw) register accessor: an alias for `Reg<TCD1_DOFF_SPEC>`"]
pub type TCD1_DOFF = crate::Reg<tcd1_doff::TCD1_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd1_doff;
#[doc = "TCD_CITER_ELINK_TCD1_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD1_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD1_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd1_citer_elinkno::TCD_CITER_ELINK_TCD1_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd1_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD1_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD1_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD1_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd1_citer_elinkyes::TCD_CITER_ELINK_TCD1_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd1_citer_elinkyes;
#[doc = "TCD1_DLASTSGA (rw) register accessor: an alias for `Reg<TCD1_DLASTSGA_SPEC>`"]
pub type TCD1_DLASTSGA = crate::Reg<tcd1_dlastsga::TCD1_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd1_dlastsga;
#[doc = "TCD1_CSR (rw) register accessor: an alias for `Reg<TCD1_CSR_SPEC>`"]
pub type TCD1_CSR = crate::Reg<tcd1_csr::TCD1_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd1_csr;
#[doc = "TCD_BITER_ELINK_TCD1_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD1_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD1_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd1_biter_elinkno::TCD_BITER_ELINK_TCD1_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd1_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD1_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD1_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD1_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd1_biter_elinkyes::TCD_BITER_ELINK_TCD1_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd1_biter_elinkyes;
#[doc = "TCD2_SADDR (rw) register accessor: an alias for `Reg<TCD2_SADDR_SPEC>`"]
pub type TCD2_SADDR = crate::Reg<tcd2_saddr::TCD2_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd2_saddr;
#[doc = "TCD2_SOFF (rw) register accessor: an alias for `Reg<TCD2_SOFF_SPEC>`"]
pub type TCD2_SOFF = crate::Reg<tcd2_soff::TCD2_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd2_soff;
#[doc = "TCD2_ATTR (rw) register accessor: an alias for `Reg<TCD2_ATTR_SPEC>`"]
pub type TCD2_ATTR = crate::Reg<tcd2_attr::TCD2_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd2_attr;
#[doc = "TCD_NBYTES_TCD2_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD2_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD2_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd2_nbytes_mlno::TCD_NBYTES_TCD2_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd2_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD2_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD2_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD2_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd2_nbytes_mloffno::TCD_NBYTES_TCD2_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd2_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD2_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD2_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD2_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd2_nbytes_mloffyes::TCD_NBYTES_TCD2_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd2_nbytes_mloffyes;
#[doc = "TCD2_SLAST (rw) register accessor: an alias for `Reg<TCD2_SLAST_SPEC>`"]
pub type TCD2_SLAST = crate::Reg<tcd2_slast::TCD2_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd2_slast;
#[doc = "TCD2_DADDR (rw) register accessor: an alias for `Reg<TCD2_DADDR_SPEC>`"]
pub type TCD2_DADDR = crate::Reg<tcd2_daddr::TCD2_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd2_daddr;
#[doc = "TCD2_DOFF (rw) register accessor: an alias for `Reg<TCD2_DOFF_SPEC>`"]
pub type TCD2_DOFF = crate::Reg<tcd2_doff::TCD2_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd2_doff;
#[doc = "TCD_CITER_ELINK_TCD2_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD2_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD2_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd2_citer_elinkno::TCD_CITER_ELINK_TCD2_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd2_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD2_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD2_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD2_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd2_citer_elinkyes::TCD_CITER_ELINK_TCD2_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd2_citer_elinkyes;
#[doc = "TCD2_DLASTSGA (rw) register accessor: an alias for `Reg<TCD2_DLASTSGA_SPEC>`"]
pub type TCD2_DLASTSGA = crate::Reg<tcd2_dlastsga::TCD2_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd2_dlastsga;
#[doc = "TCD2_CSR (rw) register accessor: an alias for `Reg<TCD2_CSR_SPEC>`"]
pub type TCD2_CSR = crate::Reg<tcd2_csr::TCD2_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd2_csr;
#[doc = "TCD_BITER_ELINK_TCD2_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD2_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD2_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd2_biter_elinkno::TCD_BITER_ELINK_TCD2_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd2_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD2_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD2_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD2_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd2_biter_elinkyes::TCD_BITER_ELINK_TCD2_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd2_biter_elinkyes;
#[doc = "TCD3_SADDR (rw) register accessor: an alias for `Reg<TCD3_SADDR_SPEC>`"]
pub type TCD3_SADDR = crate::Reg<tcd3_saddr::TCD3_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd3_saddr;
#[doc = "TCD3_SOFF (rw) register accessor: an alias for `Reg<TCD3_SOFF_SPEC>`"]
pub type TCD3_SOFF = crate::Reg<tcd3_soff::TCD3_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd3_soff;
#[doc = "TCD3_ATTR (rw) register accessor: an alias for `Reg<TCD3_ATTR_SPEC>`"]
pub type TCD3_ATTR = crate::Reg<tcd3_attr::TCD3_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd3_attr;
#[doc = "TCD_NBYTES_TCD3_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD3_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD3_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd3_nbytes_mlno::TCD_NBYTES_TCD3_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd3_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD3_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD3_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD3_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd3_nbytes_mloffno::TCD_NBYTES_TCD3_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd3_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD3_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD3_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD3_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd3_nbytes_mloffyes::TCD_NBYTES_TCD3_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd3_nbytes_mloffyes;
#[doc = "TCD3_SLAST (rw) register accessor: an alias for `Reg<TCD3_SLAST_SPEC>`"]
pub type TCD3_SLAST = crate::Reg<tcd3_slast::TCD3_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd3_slast;
#[doc = "TCD3_DADDR (rw) register accessor: an alias for `Reg<TCD3_DADDR_SPEC>`"]
pub type TCD3_DADDR = crate::Reg<tcd3_daddr::TCD3_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd3_daddr;
#[doc = "TCD3_DOFF (rw) register accessor: an alias for `Reg<TCD3_DOFF_SPEC>`"]
pub type TCD3_DOFF = crate::Reg<tcd3_doff::TCD3_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd3_doff;
#[doc = "TCD_CITER_ELINK_TCD3_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD3_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD3_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd3_citer_elinkno::TCD_CITER_ELINK_TCD3_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd3_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD3_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD3_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD3_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd3_citer_elinkyes::TCD_CITER_ELINK_TCD3_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd3_citer_elinkyes;
#[doc = "TCD3_DLASTSGA (rw) register accessor: an alias for `Reg<TCD3_DLASTSGA_SPEC>`"]
pub type TCD3_DLASTSGA = crate::Reg<tcd3_dlastsga::TCD3_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd3_dlastsga;
#[doc = "TCD3_CSR (rw) register accessor: an alias for `Reg<TCD3_CSR_SPEC>`"]
pub type TCD3_CSR = crate::Reg<tcd3_csr::TCD3_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd3_csr;
#[doc = "TCD_BITER_ELINK_TCD3_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD3_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD3_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd3_biter_elinkno::TCD_BITER_ELINK_TCD3_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd3_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD3_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD3_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD3_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd3_biter_elinkyes::TCD_BITER_ELINK_TCD3_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd3_biter_elinkyes;
#[doc = "TCD4_SADDR (rw) register accessor: an alias for `Reg<TCD4_SADDR_SPEC>`"]
pub type TCD4_SADDR = crate::Reg<tcd4_saddr::TCD4_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd4_saddr;
#[doc = "TCD4_SOFF (rw) register accessor: an alias for `Reg<TCD4_SOFF_SPEC>`"]
pub type TCD4_SOFF = crate::Reg<tcd4_soff::TCD4_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd4_soff;
#[doc = "TCD4_ATTR (rw) register accessor: an alias for `Reg<TCD4_ATTR_SPEC>`"]
pub type TCD4_ATTR = crate::Reg<tcd4_attr::TCD4_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd4_attr;
#[doc = "TCD_NBYTES_TCD4_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD4_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD4_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd4_nbytes_mlno::TCD_NBYTES_TCD4_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd4_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD4_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD4_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD4_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd4_nbytes_mloffno::TCD_NBYTES_TCD4_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd4_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD4_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD4_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD4_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd4_nbytes_mloffyes::TCD_NBYTES_TCD4_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd4_nbytes_mloffyes;
#[doc = "TCD4_SLAST (rw) register accessor: an alias for `Reg<TCD4_SLAST_SPEC>`"]
pub type TCD4_SLAST = crate::Reg<tcd4_slast::TCD4_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd4_slast;
#[doc = "TCD4_DADDR (rw) register accessor: an alias for `Reg<TCD4_DADDR_SPEC>`"]
pub type TCD4_DADDR = crate::Reg<tcd4_daddr::TCD4_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd4_daddr;
#[doc = "TCD4_DOFF (rw) register accessor: an alias for `Reg<TCD4_DOFF_SPEC>`"]
pub type TCD4_DOFF = crate::Reg<tcd4_doff::TCD4_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd4_doff;
#[doc = "TCD_CITER_ELINK_TCD4_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD4_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD4_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd4_citer_elinkno::TCD_CITER_ELINK_TCD4_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd4_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD4_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD4_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD4_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd4_citer_elinkyes::TCD_CITER_ELINK_TCD4_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd4_citer_elinkyes;
#[doc = "TCD4_DLASTSGA (rw) register accessor: an alias for `Reg<TCD4_DLASTSGA_SPEC>`"]
pub type TCD4_DLASTSGA = crate::Reg<tcd4_dlastsga::TCD4_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd4_dlastsga;
#[doc = "TCD4_CSR (rw) register accessor: an alias for `Reg<TCD4_CSR_SPEC>`"]
pub type TCD4_CSR = crate::Reg<tcd4_csr::TCD4_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd4_csr;
#[doc = "TCD_BITER_ELINK_TCD4_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD4_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD4_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd4_biter_elinkno::TCD_BITER_ELINK_TCD4_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd4_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD4_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD4_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD4_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd4_biter_elinkyes::TCD_BITER_ELINK_TCD4_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd4_biter_elinkyes;
#[doc = "TCD5_SADDR (rw) register accessor: an alias for `Reg<TCD5_SADDR_SPEC>`"]
pub type TCD5_SADDR = crate::Reg<tcd5_saddr::TCD5_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd5_saddr;
#[doc = "TCD5_SOFF (rw) register accessor: an alias for `Reg<TCD5_SOFF_SPEC>`"]
pub type TCD5_SOFF = crate::Reg<tcd5_soff::TCD5_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd5_soff;
#[doc = "TCD5_ATTR (rw) register accessor: an alias for `Reg<TCD5_ATTR_SPEC>`"]
pub type TCD5_ATTR = crate::Reg<tcd5_attr::TCD5_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd5_attr;
#[doc = "TCD_NBYTES_TCD5_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD5_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD5_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd5_nbytes_mlno::TCD_NBYTES_TCD5_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd5_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD5_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD5_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD5_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd5_nbytes_mloffno::TCD_NBYTES_TCD5_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd5_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD5_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD5_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD5_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd5_nbytes_mloffyes::TCD_NBYTES_TCD5_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd5_nbytes_mloffyes;
#[doc = "TCD5_SLAST (rw) register accessor: an alias for `Reg<TCD5_SLAST_SPEC>`"]
pub type TCD5_SLAST = crate::Reg<tcd5_slast::TCD5_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd5_slast;
#[doc = "TCD5_DADDR (rw) register accessor: an alias for `Reg<TCD5_DADDR_SPEC>`"]
pub type TCD5_DADDR = crate::Reg<tcd5_daddr::TCD5_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd5_daddr;
#[doc = "TCD5_DOFF (rw) register accessor: an alias for `Reg<TCD5_DOFF_SPEC>`"]
pub type TCD5_DOFF = crate::Reg<tcd5_doff::TCD5_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd5_doff;
#[doc = "TCD_CITER_ELINK_TCD5_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD5_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD5_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd5_citer_elinkno::TCD_CITER_ELINK_TCD5_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd5_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD5_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD5_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD5_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd5_citer_elinkyes::TCD_CITER_ELINK_TCD5_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd5_citer_elinkyes;
#[doc = "TCD5_DLASTSGA (rw) register accessor: an alias for `Reg<TCD5_DLASTSGA_SPEC>`"]
pub type TCD5_DLASTSGA = crate::Reg<tcd5_dlastsga::TCD5_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd5_dlastsga;
#[doc = "TCD5_CSR (rw) register accessor: an alias for `Reg<TCD5_CSR_SPEC>`"]
pub type TCD5_CSR = crate::Reg<tcd5_csr::TCD5_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd5_csr;
#[doc = "TCD_BITER_ELINK_TCD5_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD5_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD5_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd5_biter_elinkno::TCD_BITER_ELINK_TCD5_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd5_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD5_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD5_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD5_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd5_biter_elinkyes::TCD_BITER_ELINK_TCD5_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd5_biter_elinkyes;
#[doc = "TCD6_SADDR (rw) register accessor: an alias for `Reg<TCD6_SADDR_SPEC>`"]
pub type TCD6_SADDR = crate::Reg<tcd6_saddr::TCD6_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd6_saddr;
#[doc = "TCD6_SOFF (rw) register accessor: an alias for `Reg<TCD6_SOFF_SPEC>`"]
pub type TCD6_SOFF = crate::Reg<tcd6_soff::TCD6_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd6_soff;
#[doc = "TCD6_ATTR (rw) register accessor: an alias for `Reg<TCD6_ATTR_SPEC>`"]
pub type TCD6_ATTR = crate::Reg<tcd6_attr::TCD6_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd6_attr;
#[doc = "TCD_NBYTES_TCD6_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD6_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD6_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd6_nbytes_mlno::TCD_NBYTES_TCD6_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd6_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD6_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD6_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD6_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd6_nbytes_mloffno::TCD_NBYTES_TCD6_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd6_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD6_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD6_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD6_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd6_nbytes_mloffyes::TCD_NBYTES_TCD6_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd6_nbytes_mloffyes;
#[doc = "TCD6_SLAST (rw) register accessor: an alias for `Reg<TCD6_SLAST_SPEC>`"]
pub type TCD6_SLAST = crate::Reg<tcd6_slast::TCD6_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd6_slast;
#[doc = "TCD6_DADDR (rw) register accessor: an alias for `Reg<TCD6_DADDR_SPEC>`"]
pub type TCD6_DADDR = crate::Reg<tcd6_daddr::TCD6_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd6_daddr;
#[doc = "TCD6_DOFF (rw) register accessor: an alias for `Reg<TCD6_DOFF_SPEC>`"]
pub type TCD6_DOFF = crate::Reg<tcd6_doff::TCD6_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd6_doff;
#[doc = "TCD_CITER_ELINK_TCD6_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD6_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD6_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd6_citer_elinkno::TCD_CITER_ELINK_TCD6_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd6_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD6_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD6_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD6_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd6_citer_elinkyes::TCD_CITER_ELINK_TCD6_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd6_citer_elinkyes;
#[doc = "TCD6_DLASTSGA (rw) register accessor: an alias for `Reg<TCD6_DLASTSGA_SPEC>`"]
pub type TCD6_DLASTSGA = crate::Reg<tcd6_dlastsga::TCD6_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd6_dlastsga;
#[doc = "TCD6_CSR (rw) register accessor: an alias for `Reg<TCD6_CSR_SPEC>`"]
pub type TCD6_CSR = crate::Reg<tcd6_csr::TCD6_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd6_csr;
#[doc = "TCD_BITER_ELINK_TCD6_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD6_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD6_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd6_biter_elinkno::TCD_BITER_ELINK_TCD6_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd6_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD6_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD6_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD6_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd6_biter_elinkyes::TCD_BITER_ELINK_TCD6_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd6_biter_elinkyes;
#[doc = "TCD7_SADDR (rw) register accessor: an alias for `Reg<TCD7_SADDR_SPEC>`"]
pub type TCD7_SADDR = crate::Reg<tcd7_saddr::TCD7_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd7_saddr;
#[doc = "TCD7_SOFF (rw) register accessor: an alias for `Reg<TCD7_SOFF_SPEC>`"]
pub type TCD7_SOFF = crate::Reg<tcd7_soff::TCD7_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd7_soff;
#[doc = "TCD7_ATTR (rw) register accessor: an alias for `Reg<TCD7_ATTR_SPEC>`"]
pub type TCD7_ATTR = crate::Reg<tcd7_attr::TCD7_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd7_attr;
#[doc = "TCD_NBYTES_TCD7_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD7_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD7_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd7_nbytes_mlno::TCD_NBYTES_TCD7_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd7_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD7_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD7_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD7_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd7_nbytes_mloffno::TCD_NBYTES_TCD7_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd7_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD7_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD7_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD7_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd7_nbytes_mloffyes::TCD_NBYTES_TCD7_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd7_nbytes_mloffyes;
#[doc = "TCD7_SLAST (rw) register accessor: an alias for `Reg<TCD7_SLAST_SPEC>`"]
pub type TCD7_SLAST = crate::Reg<tcd7_slast::TCD7_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd7_slast;
#[doc = "TCD7_DADDR (rw) register accessor: an alias for `Reg<TCD7_DADDR_SPEC>`"]
pub type TCD7_DADDR = crate::Reg<tcd7_daddr::TCD7_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd7_daddr;
#[doc = "TCD7_DOFF (rw) register accessor: an alias for `Reg<TCD7_DOFF_SPEC>`"]
pub type TCD7_DOFF = crate::Reg<tcd7_doff::TCD7_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd7_doff;
#[doc = "TCD_CITER_ELINK_TCD7_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD7_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD7_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd7_citer_elinkno::TCD_CITER_ELINK_TCD7_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd7_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD7_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD7_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD7_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd7_citer_elinkyes::TCD_CITER_ELINK_TCD7_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd7_citer_elinkyes;
#[doc = "TCD7_DLASTSGA (rw) register accessor: an alias for `Reg<TCD7_DLASTSGA_SPEC>`"]
pub type TCD7_DLASTSGA = crate::Reg<tcd7_dlastsga::TCD7_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd7_dlastsga;
#[doc = "TCD7_CSR (rw) register accessor: an alias for `Reg<TCD7_CSR_SPEC>`"]
pub type TCD7_CSR = crate::Reg<tcd7_csr::TCD7_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd7_csr;
#[doc = "TCD_BITER_ELINK_TCD7_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD7_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD7_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd7_biter_elinkno::TCD_BITER_ELINK_TCD7_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd7_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD7_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD7_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD7_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd7_biter_elinkyes::TCD_BITER_ELINK_TCD7_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd7_biter_elinkyes;
#[doc = "TCD8_SADDR (rw) register accessor: an alias for `Reg<TCD8_SADDR_SPEC>`"]
pub type TCD8_SADDR = crate::Reg<tcd8_saddr::TCD8_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd8_saddr;
#[doc = "TCD8_SOFF (rw) register accessor: an alias for `Reg<TCD8_SOFF_SPEC>`"]
pub type TCD8_SOFF = crate::Reg<tcd8_soff::TCD8_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd8_soff;
#[doc = "TCD8_ATTR (rw) register accessor: an alias for `Reg<TCD8_ATTR_SPEC>`"]
pub type TCD8_ATTR = crate::Reg<tcd8_attr::TCD8_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd8_attr;
#[doc = "TCD_NBYTES_TCD8_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD8_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD8_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd8_nbytes_mlno::TCD_NBYTES_TCD8_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd8_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD8_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD8_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD8_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd8_nbytes_mloffno::TCD_NBYTES_TCD8_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd8_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD8_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD8_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD8_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd8_nbytes_mloffyes::TCD_NBYTES_TCD8_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd8_nbytes_mloffyes;
#[doc = "TCD8_SLAST (rw) register accessor: an alias for `Reg<TCD8_SLAST_SPEC>`"]
pub type TCD8_SLAST = crate::Reg<tcd8_slast::TCD8_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd8_slast;
#[doc = "TCD8_DADDR (rw) register accessor: an alias for `Reg<TCD8_DADDR_SPEC>`"]
pub type TCD8_DADDR = crate::Reg<tcd8_daddr::TCD8_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd8_daddr;
#[doc = "TCD8_DOFF (rw) register accessor: an alias for `Reg<TCD8_DOFF_SPEC>`"]
pub type TCD8_DOFF = crate::Reg<tcd8_doff::TCD8_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd8_doff;
#[doc = "TCD_CITER_ELINK_TCD8_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD8_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD8_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd8_citer_elinkno::TCD_CITER_ELINK_TCD8_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd8_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD8_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD8_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD8_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd8_citer_elinkyes::TCD_CITER_ELINK_TCD8_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd8_citer_elinkyes;
#[doc = "TCD8_DLASTSGA (rw) register accessor: an alias for `Reg<TCD8_DLASTSGA_SPEC>`"]
pub type TCD8_DLASTSGA = crate::Reg<tcd8_dlastsga::TCD8_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd8_dlastsga;
#[doc = "TCD8_CSR (rw) register accessor: an alias for `Reg<TCD8_CSR_SPEC>`"]
pub type TCD8_CSR = crate::Reg<tcd8_csr::TCD8_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd8_csr;
#[doc = "TCD_BITER_ELINK_TCD8_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD8_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD8_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd8_biter_elinkno::TCD_BITER_ELINK_TCD8_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd8_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD8_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD8_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD8_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd8_biter_elinkyes::TCD_BITER_ELINK_TCD8_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd8_biter_elinkyes;
#[doc = "TCD9_SADDR (rw) register accessor: an alias for `Reg<TCD9_SADDR_SPEC>`"]
pub type TCD9_SADDR = crate::Reg<tcd9_saddr::TCD9_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd9_saddr;
#[doc = "TCD9_SOFF (rw) register accessor: an alias for `Reg<TCD9_SOFF_SPEC>`"]
pub type TCD9_SOFF = crate::Reg<tcd9_soff::TCD9_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd9_soff;
#[doc = "TCD9_ATTR (rw) register accessor: an alias for `Reg<TCD9_ATTR_SPEC>`"]
pub type TCD9_ATTR = crate::Reg<tcd9_attr::TCD9_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd9_attr;
#[doc = "TCD_NBYTES_TCD9_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD9_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD9_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd9_nbytes_mlno::TCD_NBYTES_TCD9_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd9_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD9_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD9_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD9_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd9_nbytes_mloffno::TCD_NBYTES_TCD9_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd9_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD9_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD9_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD9_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd9_nbytes_mloffyes::TCD_NBYTES_TCD9_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd9_nbytes_mloffyes;
#[doc = "TCD9_SLAST (rw) register accessor: an alias for `Reg<TCD9_SLAST_SPEC>`"]
pub type TCD9_SLAST = crate::Reg<tcd9_slast::TCD9_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd9_slast;
#[doc = "TCD9_DADDR (rw) register accessor: an alias for `Reg<TCD9_DADDR_SPEC>`"]
pub type TCD9_DADDR = crate::Reg<tcd9_daddr::TCD9_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd9_daddr;
#[doc = "TCD9_DOFF (rw) register accessor: an alias for `Reg<TCD9_DOFF_SPEC>`"]
pub type TCD9_DOFF = crate::Reg<tcd9_doff::TCD9_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd9_doff;
#[doc = "TCD_CITER_ELINK_TCD9_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD9_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD9_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd9_citer_elinkno::TCD_CITER_ELINK_TCD9_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd9_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD9_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD9_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD9_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd9_citer_elinkyes::TCD_CITER_ELINK_TCD9_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd9_citer_elinkyes;
#[doc = "TCD9_DLASTSGA (rw) register accessor: an alias for `Reg<TCD9_DLASTSGA_SPEC>`"]
pub type TCD9_DLASTSGA = crate::Reg<tcd9_dlastsga::TCD9_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd9_dlastsga;
#[doc = "TCD9_CSR (rw) register accessor: an alias for `Reg<TCD9_CSR_SPEC>`"]
pub type TCD9_CSR = crate::Reg<tcd9_csr::TCD9_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd9_csr;
#[doc = "TCD_BITER_ELINK_TCD9_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD9_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD9_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd9_biter_elinkno::TCD_BITER_ELINK_TCD9_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd9_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD9_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD9_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD9_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd9_biter_elinkyes::TCD_BITER_ELINK_TCD9_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd9_biter_elinkyes;
#[doc = "TCD10_SADDR (rw) register accessor: an alias for `Reg<TCD10_SADDR_SPEC>`"]
pub type TCD10_SADDR = crate::Reg<tcd10_saddr::TCD10_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd10_saddr;
#[doc = "TCD10_SOFF (rw) register accessor: an alias for `Reg<TCD10_SOFF_SPEC>`"]
pub type TCD10_SOFF = crate::Reg<tcd10_soff::TCD10_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd10_soff;
#[doc = "TCD10_ATTR (rw) register accessor: an alias for `Reg<TCD10_ATTR_SPEC>`"]
pub type TCD10_ATTR = crate::Reg<tcd10_attr::TCD10_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd10_attr;
#[doc = "TCD_NBYTES_TCD10_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD10_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD10_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd10_nbytes_mlno::TCD_NBYTES_TCD10_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd10_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD10_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD10_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD10_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd10_nbytes_mloffno::TCD_NBYTES_TCD10_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd10_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD10_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD10_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD10_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd10_nbytes_mloffyes::TCD_NBYTES_TCD10_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd10_nbytes_mloffyes;
#[doc = "TCD10_SLAST (rw) register accessor: an alias for `Reg<TCD10_SLAST_SPEC>`"]
pub type TCD10_SLAST = crate::Reg<tcd10_slast::TCD10_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd10_slast;
#[doc = "TCD10_DADDR (rw) register accessor: an alias for `Reg<TCD10_DADDR_SPEC>`"]
pub type TCD10_DADDR = crate::Reg<tcd10_daddr::TCD10_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd10_daddr;
#[doc = "TCD10_DOFF (rw) register accessor: an alias for `Reg<TCD10_DOFF_SPEC>`"]
pub type TCD10_DOFF = crate::Reg<tcd10_doff::TCD10_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd10_doff;
#[doc = "TCD_CITER_ELINK_TCD10_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD10_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD10_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd10_citer_elinkno::TCD_CITER_ELINK_TCD10_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd10_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD10_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD10_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD10_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd10_citer_elinkyes::TCD_CITER_ELINK_TCD10_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd10_citer_elinkyes;
#[doc = "TCD10_DLASTSGA (rw) register accessor: an alias for `Reg<TCD10_DLASTSGA_SPEC>`"]
pub type TCD10_DLASTSGA = crate::Reg<tcd10_dlastsga::TCD10_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd10_dlastsga;
#[doc = "TCD10_CSR (rw) register accessor: an alias for `Reg<TCD10_CSR_SPEC>`"]
pub type TCD10_CSR = crate::Reg<tcd10_csr::TCD10_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd10_csr;
#[doc = "TCD_BITER_ELINK_TCD10_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD10_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD10_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd10_biter_elinkno::TCD_BITER_ELINK_TCD10_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd10_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD10_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD10_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD10_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd10_biter_elinkyes::TCD_BITER_ELINK_TCD10_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd10_biter_elinkyes;
#[doc = "TCD11_SADDR (rw) register accessor: an alias for `Reg<TCD11_SADDR_SPEC>`"]
pub type TCD11_SADDR = crate::Reg<tcd11_saddr::TCD11_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd11_saddr;
#[doc = "TCD11_SOFF (rw) register accessor: an alias for `Reg<TCD11_SOFF_SPEC>`"]
pub type TCD11_SOFF = crate::Reg<tcd11_soff::TCD11_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd11_soff;
#[doc = "TCD11_ATTR (rw) register accessor: an alias for `Reg<TCD11_ATTR_SPEC>`"]
pub type TCD11_ATTR = crate::Reg<tcd11_attr::TCD11_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd11_attr;
#[doc = "TCD_NBYTES_TCD11_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD11_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD11_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd11_nbytes_mlno::TCD_NBYTES_TCD11_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd11_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD11_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD11_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD11_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd11_nbytes_mloffno::TCD_NBYTES_TCD11_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd11_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD11_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD11_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD11_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd11_nbytes_mloffyes::TCD_NBYTES_TCD11_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd11_nbytes_mloffyes;
#[doc = "TCD11_SLAST (rw) register accessor: an alias for `Reg<TCD11_SLAST_SPEC>`"]
pub type TCD11_SLAST = crate::Reg<tcd11_slast::TCD11_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd11_slast;
#[doc = "TCD11_DADDR (rw) register accessor: an alias for `Reg<TCD11_DADDR_SPEC>`"]
pub type TCD11_DADDR = crate::Reg<tcd11_daddr::TCD11_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd11_daddr;
#[doc = "TCD11_DOFF (rw) register accessor: an alias for `Reg<TCD11_DOFF_SPEC>`"]
pub type TCD11_DOFF = crate::Reg<tcd11_doff::TCD11_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd11_doff;
#[doc = "TCD_CITER_ELINK_TCD11_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD11_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD11_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd11_citer_elinkno::TCD_CITER_ELINK_TCD11_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd11_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD11_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD11_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD11_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd11_citer_elinkyes::TCD_CITER_ELINK_TCD11_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd11_citer_elinkyes;
#[doc = "TCD11_DLASTSGA (rw) register accessor: an alias for `Reg<TCD11_DLASTSGA_SPEC>`"]
pub type TCD11_DLASTSGA = crate::Reg<tcd11_dlastsga::TCD11_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd11_dlastsga;
#[doc = "TCD11_CSR (rw) register accessor: an alias for `Reg<TCD11_CSR_SPEC>`"]
pub type TCD11_CSR = crate::Reg<tcd11_csr::TCD11_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd11_csr;
#[doc = "TCD_BITER_ELINK_TCD11_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD11_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD11_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd11_biter_elinkno::TCD_BITER_ELINK_TCD11_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd11_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD11_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD11_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD11_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd11_biter_elinkyes::TCD_BITER_ELINK_TCD11_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd11_biter_elinkyes;
#[doc = "TCD12_SADDR (rw) register accessor: an alias for `Reg<TCD12_SADDR_SPEC>`"]
pub type TCD12_SADDR = crate::Reg<tcd12_saddr::TCD12_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd12_saddr;
#[doc = "TCD12_SOFF (rw) register accessor: an alias for `Reg<TCD12_SOFF_SPEC>`"]
pub type TCD12_SOFF = crate::Reg<tcd12_soff::TCD12_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd12_soff;
#[doc = "TCD12_ATTR (rw) register accessor: an alias for `Reg<TCD12_ATTR_SPEC>`"]
pub type TCD12_ATTR = crate::Reg<tcd12_attr::TCD12_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd12_attr;
#[doc = "TCD_NBYTES_TCD12_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD12_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD12_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd12_nbytes_mlno::TCD_NBYTES_TCD12_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd12_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD12_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD12_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD12_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd12_nbytes_mloffno::TCD_NBYTES_TCD12_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd12_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD12_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD12_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD12_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd12_nbytes_mloffyes::TCD_NBYTES_TCD12_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd12_nbytes_mloffyes;
#[doc = "TCD12_SLAST (rw) register accessor: an alias for `Reg<TCD12_SLAST_SPEC>`"]
pub type TCD12_SLAST = crate::Reg<tcd12_slast::TCD12_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd12_slast;
#[doc = "TCD12_DADDR (rw) register accessor: an alias for `Reg<TCD12_DADDR_SPEC>`"]
pub type TCD12_DADDR = crate::Reg<tcd12_daddr::TCD12_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd12_daddr;
#[doc = "TCD12_DOFF (rw) register accessor: an alias for `Reg<TCD12_DOFF_SPEC>`"]
pub type TCD12_DOFF = crate::Reg<tcd12_doff::TCD12_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd12_doff;
#[doc = "TCD_CITER_ELINK_TCD12_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD12_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD12_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd12_citer_elinkno::TCD_CITER_ELINK_TCD12_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd12_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD12_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD12_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD12_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd12_citer_elinkyes::TCD_CITER_ELINK_TCD12_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd12_citer_elinkyes;
#[doc = "TCD12_DLASTSGA (rw) register accessor: an alias for `Reg<TCD12_DLASTSGA_SPEC>`"]
pub type TCD12_DLASTSGA = crate::Reg<tcd12_dlastsga::TCD12_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd12_dlastsga;
#[doc = "TCD12_CSR (rw) register accessor: an alias for `Reg<TCD12_CSR_SPEC>`"]
pub type TCD12_CSR = crate::Reg<tcd12_csr::TCD12_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd12_csr;
#[doc = "TCD_BITER_ELINK_TCD12_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD12_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD12_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd12_biter_elinkno::TCD_BITER_ELINK_TCD12_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd12_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD12_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD12_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD12_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd12_biter_elinkyes::TCD_BITER_ELINK_TCD12_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd12_biter_elinkyes;
#[doc = "TCD13_SADDR (rw) register accessor: an alias for `Reg<TCD13_SADDR_SPEC>`"]
pub type TCD13_SADDR = crate::Reg<tcd13_saddr::TCD13_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd13_saddr;
#[doc = "TCD13_SOFF (rw) register accessor: an alias for `Reg<TCD13_SOFF_SPEC>`"]
pub type TCD13_SOFF = crate::Reg<tcd13_soff::TCD13_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd13_soff;
#[doc = "TCD13_ATTR (rw) register accessor: an alias for `Reg<TCD13_ATTR_SPEC>`"]
pub type TCD13_ATTR = crate::Reg<tcd13_attr::TCD13_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd13_attr;
#[doc = "TCD_NBYTES_TCD13_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD13_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD13_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd13_nbytes_mlno::TCD_NBYTES_TCD13_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd13_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD13_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD13_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD13_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd13_nbytes_mloffno::TCD_NBYTES_TCD13_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd13_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD13_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD13_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD13_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd13_nbytes_mloffyes::TCD_NBYTES_TCD13_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd13_nbytes_mloffyes;
#[doc = "TCD13_SLAST (rw) register accessor: an alias for `Reg<TCD13_SLAST_SPEC>`"]
pub type TCD13_SLAST = crate::Reg<tcd13_slast::TCD13_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd13_slast;
#[doc = "TCD13_DADDR (rw) register accessor: an alias for `Reg<TCD13_DADDR_SPEC>`"]
pub type TCD13_DADDR = crate::Reg<tcd13_daddr::TCD13_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd13_daddr;
#[doc = "TCD13_DOFF (rw) register accessor: an alias for `Reg<TCD13_DOFF_SPEC>`"]
pub type TCD13_DOFF = crate::Reg<tcd13_doff::TCD13_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd13_doff;
#[doc = "TCD_CITER_ELINK_TCD13_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD13_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD13_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd13_citer_elinkno::TCD_CITER_ELINK_TCD13_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd13_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD13_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD13_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD13_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd13_citer_elinkyes::TCD_CITER_ELINK_TCD13_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd13_citer_elinkyes;
#[doc = "TCD13_DLASTSGA (rw) register accessor: an alias for `Reg<TCD13_DLASTSGA_SPEC>`"]
pub type TCD13_DLASTSGA = crate::Reg<tcd13_dlastsga::TCD13_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd13_dlastsga;
#[doc = "TCD13_CSR (rw) register accessor: an alias for `Reg<TCD13_CSR_SPEC>`"]
pub type TCD13_CSR = crate::Reg<tcd13_csr::TCD13_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd13_csr;
#[doc = "TCD_BITER_ELINK_TCD13_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD13_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD13_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd13_biter_elinkno::TCD_BITER_ELINK_TCD13_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd13_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD13_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD13_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD13_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd13_biter_elinkyes::TCD_BITER_ELINK_TCD13_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd13_biter_elinkyes;
#[doc = "TCD14_SADDR (rw) register accessor: an alias for `Reg<TCD14_SADDR_SPEC>`"]
pub type TCD14_SADDR = crate::Reg<tcd14_saddr::TCD14_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd14_saddr;
#[doc = "TCD14_SOFF (rw) register accessor: an alias for `Reg<TCD14_SOFF_SPEC>`"]
pub type TCD14_SOFF = crate::Reg<tcd14_soff::TCD14_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd14_soff;
#[doc = "TCD14_ATTR (rw) register accessor: an alias for `Reg<TCD14_ATTR_SPEC>`"]
pub type TCD14_ATTR = crate::Reg<tcd14_attr::TCD14_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd14_attr;
#[doc = "TCD_NBYTES_TCD14_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD14_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD14_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd14_nbytes_mlno::TCD_NBYTES_TCD14_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd14_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD14_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD14_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD14_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd14_nbytes_mloffno::TCD_NBYTES_TCD14_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd14_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD14_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD14_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD14_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd14_nbytes_mloffyes::TCD_NBYTES_TCD14_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd14_nbytes_mloffyes;
#[doc = "TCD14_SLAST (rw) register accessor: an alias for `Reg<TCD14_SLAST_SPEC>`"]
pub type TCD14_SLAST = crate::Reg<tcd14_slast::TCD14_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd14_slast;
#[doc = "TCD14_DADDR (rw) register accessor: an alias for `Reg<TCD14_DADDR_SPEC>`"]
pub type TCD14_DADDR = crate::Reg<tcd14_daddr::TCD14_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd14_daddr;
#[doc = "TCD14_DOFF (rw) register accessor: an alias for `Reg<TCD14_DOFF_SPEC>`"]
pub type TCD14_DOFF = crate::Reg<tcd14_doff::TCD14_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd14_doff;
#[doc = "TCD_CITER_ELINK_TCD14_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD14_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD14_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd14_citer_elinkno::TCD_CITER_ELINK_TCD14_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd14_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD14_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD14_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD14_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd14_citer_elinkyes::TCD_CITER_ELINK_TCD14_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd14_citer_elinkyes;
#[doc = "TCD14_DLASTSGA (rw) register accessor: an alias for `Reg<TCD14_DLASTSGA_SPEC>`"]
pub type TCD14_DLASTSGA = crate::Reg<tcd14_dlastsga::TCD14_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd14_dlastsga;
#[doc = "TCD14_CSR (rw) register accessor: an alias for `Reg<TCD14_CSR_SPEC>`"]
pub type TCD14_CSR = crate::Reg<tcd14_csr::TCD14_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd14_csr;
#[doc = "TCD_BITER_ELINK_TCD14_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD14_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD14_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd14_biter_elinkno::TCD_BITER_ELINK_TCD14_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd14_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD14_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD14_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD14_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd14_biter_elinkyes::TCD_BITER_ELINK_TCD14_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd14_biter_elinkyes;
#[doc = "TCD15_SADDR (rw) register accessor: an alias for `Reg<TCD15_SADDR_SPEC>`"]
pub type TCD15_SADDR = crate::Reg<tcd15_saddr::TCD15_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd15_saddr;
#[doc = "TCD15_SOFF (rw) register accessor: an alias for `Reg<TCD15_SOFF_SPEC>`"]
pub type TCD15_SOFF = crate::Reg<tcd15_soff::TCD15_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd15_soff;
#[doc = "TCD15_ATTR (rw) register accessor: an alias for `Reg<TCD15_ATTR_SPEC>`"]
pub type TCD15_ATTR = crate::Reg<tcd15_attr::TCD15_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd15_attr;
#[doc = "TCD_NBYTES_TCD15_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD15_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD15_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd15_nbytes_mlno::TCD_NBYTES_TCD15_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd15_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD15_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD15_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD15_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd15_nbytes_mloffno::TCD_NBYTES_TCD15_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd15_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD15_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD15_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD15_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd15_nbytes_mloffyes::TCD_NBYTES_TCD15_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd15_nbytes_mloffyes;
#[doc = "TCD15_SLAST (rw) register accessor: an alias for `Reg<TCD15_SLAST_SPEC>`"]
pub type TCD15_SLAST = crate::Reg<tcd15_slast::TCD15_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd15_slast;
#[doc = "TCD15_DADDR (rw) register accessor: an alias for `Reg<TCD15_DADDR_SPEC>`"]
pub type TCD15_DADDR = crate::Reg<tcd15_daddr::TCD15_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd15_daddr;
#[doc = "TCD15_DOFF (rw) register accessor: an alias for `Reg<TCD15_DOFF_SPEC>`"]
pub type TCD15_DOFF = crate::Reg<tcd15_doff::TCD15_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd15_doff;
#[doc = "TCD_CITER_ELINK_TCD15_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD15_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD15_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd15_citer_elinkno::TCD_CITER_ELINK_TCD15_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd15_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD15_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD15_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD15_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd15_citer_elinkyes::TCD_CITER_ELINK_TCD15_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd15_citer_elinkyes;
#[doc = "TCD15_DLASTSGA (rw) register accessor: an alias for `Reg<TCD15_DLASTSGA_SPEC>`"]
pub type TCD15_DLASTSGA = crate::Reg<tcd15_dlastsga::TCD15_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd15_dlastsga;
#[doc = "TCD15_CSR (rw) register accessor: an alias for `Reg<TCD15_CSR_SPEC>`"]
pub type TCD15_CSR = crate::Reg<tcd15_csr::TCD15_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd15_csr;
#[doc = "TCD_BITER_ELINK_TCD15_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD15_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD15_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd15_biter_elinkno::TCD_BITER_ELINK_TCD15_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd15_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD15_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD15_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD15_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd15_biter_elinkyes::TCD_BITER_ELINK_TCD15_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd15_biter_elinkyes;
#[doc = "TCD16_SADDR (rw) register accessor: an alias for `Reg<TCD16_SADDR_SPEC>`"]
pub type TCD16_SADDR = crate::Reg<tcd16_saddr::TCD16_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd16_saddr;
#[doc = "TCD16_SOFF (rw) register accessor: an alias for `Reg<TCD16_SOFF_SPEC>`"]
pub type TCD16_SOFF = crate::Reg<tcd16_soff::TCD16_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd16_soff;
#[doc = "TCD16_ATTR (rw) register accessor: an alias for `Reg<TCD16_ATTR_SPEC>`"]
pub type TCD16_ATTR = crate::Reg<tcd16_attr::TCD16_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd16_attr;
#[doc = "TCD_NBYTES_TCD16_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD16_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD16_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd16_nbytes_mlno::TCD_NBYTES_TCD16_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd16_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD16_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD16_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD16_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd16_nbytes_mloffno::TCD_NBYTES_TCD16_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd16_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD16_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD16_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD16_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd16_nbytes_mloffyes::TCD_NBYTES_TCD16_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd16_nbytes_mloffyes;
#[doc = "TCD16_SLAST (rw) register accessor: an alias for `Reg<TCD16_SLAST_SPEC>`"]
pub type TCD16_SLAST = crate::Reg<tcd16_slast::TCD16_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd16_slast;
#[doc = "TCD16_DADDR (rw) register accessor: an alias for `Reg<TCD16_DADDR_SPEC>`"]
pub type TCD16_DADDR = crate::Reg<tcd16_daddr::TCD16_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd16_daddr;
#[doc = "TCD16_DOFF (rw) register accessor: an alias for `Reg<TCD16_DOFF_SPEC>`"]
pub type TCD16_DOFF = crate::Reg<tcd16_doff::TCD16_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd16_doff;
#[doc = "TCD_CITER_ELINK_TCD16_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD16_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD16_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd16_citer_elinkno::TCD_CITER_ELINK_TCD16_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd16_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD16_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD16_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD16_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd16_citer_elinkyes::TCD_CITER_ELINK_TCD16_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd16_citer_elinkyes;
#[doc = "TCD16_DLASTSGA (rw) register accessor: an alias for `Reg<TCD16_DLASTSGA_SPEC>`"]
pub type TCD16_DLASTSGA = crate::Reg<tcd16_dlastsga::TCD16_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd16_dlastsga;
#[doc = "TCD16_CSR (rw) register accessor: an alias for `Reg<TCD16_CSR_SPEC>`"]
pub type TCD16_CSR = crate::Reg<tcd16_csr::TCD16_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd16_csr;
#[doc = "TCD_BITER_ELINK_TCD16_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD16_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD16_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd16_biter_elinkno::TCD_BITER_ELINK_TCD16_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd16_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD16_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD16_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD16_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd16_biter_elinkyes::TCD_BITER_ELINK_TCD16_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd16_biter_elinkyes;
#[doc = "TCD17_SADDR (rw) register accessor: an alias for `Reg<TCD17_SADDR_SPEC>`"]
pub type TCD17_SADDR = crate::Reg<tcd17_saddr::TCD17_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd17_saddr;
#[doc = "TCD17_SOFF (rw) register accessor: an alias for `Reg<TCD17_SOFF_SPEC>`"]
pub type TCD17_SOFF = crate::Reg<tcd17_soff::TCD17_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd17_soff;
#[doc = "TCD17_ATTR (rw) register accessor: an alias for `Reg<TCD17_ATTR_SPEC>`"]
pub type TCD17_ATTR = crate::Reg<tcd17_attr::TCD17_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd17_attr;
#[doc = "TCD_NBYTES_TCD17_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD17_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD17_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd17_nbytes_mlno::TCD_NBYTES_TCD17_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd17_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD17_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD17_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD17_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd17_nbytes_mloffno::TCD_NBYTES_TCD17_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd17_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD17_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD17_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD17_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd17_nbytes_mloffyes::TCD_NBYTES_TCD17_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd17_nbytes_mloffyes;
#[doc = "TCD17_SLAST (rw) register accessor: an alias for `Reg<TCD17_SLAST_SPEC>`"]
pub type TCD17_SLAST = crate::Reg<tcd17_slast::TCD17_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd17_slast;
#[doc = "TCD17_DADDR (rw) register accessor: an alias for `Reg<TCD17_DADDR_SPEC>`"]
pub type TCD17_DADDR = crate::Reg<tcd17_daddr::TCD17_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd17_daddr;
#[doc = "TCD17_DOFF (rw) register accessor: an alias for `Reg<TCD17_DOFF_SPEC>`"]
pub type TCD17_DOFF = crate::Reg<tcd17_doff::TCD17_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd17_doff;
#[doc = "TCD_CITER_ELINK_TCD17_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD17_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD17_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd17_citer_elinkno::TCD_CITER_ELINK_TCD17_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd17_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD17_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD17_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD17_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd17_citer_elinkyes::TCD_CITER_ELINK_TCD17_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd17_citer_elinkyes;
#[doc = "TCD17_DLASTSGA (rw) register accessor: an alias for `Reg<TCD17_DLASTSGA_SPEC>`"]
pub type TCD17_DLASTSGA = crate::Reg<tcd17_dlastsga::TCD17_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd17_dlastsga;
#[doc = "TCD17_CSR (rw) register accessor: an alias for `Reg<TCD17_CSR_SPEC>`"]
pub type TCD17_CSR = crate::Reg<tcd17_csr::TCD17_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd17_csr;
#[doc = "TCD_BITER_ELINK_TCD17_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD17_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD17_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd17_biter_elinkno::TCD_BITER_ELINK_TCD17_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd17_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD17_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD17_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD17_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd17_biter_elinkyes::TCD_BITER_ELINK_TCD17_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd17_biter_elinkyes;
#[doc = "TCD18_SADDR (rw) register accessor: an alias for `Reg<TCD18_SADDR_SPEC>`"]
pub type TCD18_SADDR = crate::Reg<tcd18_saddr::TCD18_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd18_saddr;
#[doc = "TCD18_SOFF (rw) register accessor: an alias for `Reg<TCD18_SOFF_SPEC>`"]
pub type TCD18_SOFF = crate::Reg<tcd18_soff::TCD18_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd18_soff;
#[doc = "TCD18_ATTR (rw) register accessor: an alias for `Reg<TCD18_ATTR_SPEC>`"]
pub type TCD18_ATTR = crate::Reg<tcd18_attr::TCD18_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd18_attr;
#[doc = "TCD_NBYTES_TCD18_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD18_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD18_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd18_nbytes_mlno::TCD_NBYTES_TCD18_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd18_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD18_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD18_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD18_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd18_nbytes_mloffno::TCD_NBYTES_TCD18_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd18_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD18_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD18_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD18_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd18_nbytes_mloffyes::TCD_NBYTES_TCD18_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd18_nbytes_mloffyes;
#[doc = "TCD18_SLAST (rw) register accessor: an alias for `Reg<TCD18_SLAST_SPEC>`"]
pub type TCD18_SLAST = crate::Reg<tcd18_slast::TCD18_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd18_slast;
#[doc = "TCD18_DADDR (rw) register accessor: an alias for `Reg<TCD18_DADDR_SPEC>`"]
pub type TCD18_DADDR = crate::Reg<tcd18_daddr::TCD18_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd18_daddr;
#[doc = "TCD18_DOFF (rw) register accessor: an alias for `Reg<TCD18_DOFF_SPEC>`"]
pub type TCD18_DOFF = crate::Reg<tcd18_doff::TCD18_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd18_doff;
#[doc = "TCD_CITER_ELINK_TCD18_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD18_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD18_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd18_citer_elinkno::TCD_CITER_ELINK_TCD18_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd18_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD18_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD18_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD18_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd18_citer_elinkyes::TCD_CITER_ELINK_TCD18_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd18_citer_elinkyes;
#[doc = "TCD18_DLASTSGA (rw) register accessor: an alias for `Reg<TCD18_DLASTSGA_SPEC>`"]
pub type TCD18_DLASTSGA = crate::Reg<tcd18_dlastsga::TCD18_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd18_dlastsga;
#[doc = "TCD18_CSR (rw) register accessor: an alias for `Reg<TCD18_CSR_SPEC>`"]
pub type TCD18_CSR = crate::Reg<tcd18_csr::TCD18_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd18_csr;
#[doc = "TCD_BITER_ELINK_TCD18_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD18_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD18_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd18_biter_elinkno::TCD_BITER_ELINK_TCD18_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd18_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD18_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD18_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD18_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd18_biter_elinkyes::TCD_BITER_ELINK_TCD18_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd18_biter_elinkyes;
#[doc = "TCD19_SADDR (rw) register accessor: an alias for `Reg<TCD19_SADDR_SPEC>`"]
pub type TCD19_SADDR = crate::Reg<tcd19_saddr::TCD19_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd19_saddr;
#[doc = "TCD19_SOFF (rw) register accessor: an alias for `Reg<TCD19_SOFF_SPEC>`"]
pub type TCD19_SOFF = crate::Reg<tcd19_soff::TCD19_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd19_soff;
#[doc = "TCD19_ATTR (rw) register accessor: an alias for `Reg<TCD19_ATTR_SPEC>`"]
pub type TCD19_ATTR = crate::Reg<tcd19_attr::TCD19_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd19_attr;
#[doc = "TCD_NBYTES_TCD19_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD19_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD19_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd19_nbytes_mlno::TCD_NBYTES_TCD19_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd19_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD19_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD19_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD19_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd19_nbytes_mloffno::TCD_NBYTES_TCD19_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd19_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD19_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD19_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD19_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd19_nbytes_mloffyes::TCD_NBYTES_TCD19_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd19_nbytes_mloffyes;
#[doc = "TCD19_SLAST (rw) register accessor: an alias for `Reg<TCD19_SLAST_SPEC>`"]
pub type TCD19_SLAST = crate::Reg<tcd19_slast::TCD19_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd19_slast;
#[doc = "TCD19_DADDR (rw) register accessor: an alias for `Reg<TCD19_DADDR_SPEC>`"]
pub type TCD19_DADDR = crate::Reg<tcd19_daddr::TCD19_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd19_daddr;
#[doc = "TCD19_DOFF (rw) register accessor: an alias for `Reg<TCD19_DOFF_SPEC>`"]
pub type TCD19_DOFF = crate::Reg<tcd19_doff::TCD19_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd19_doff;
#[doc = "TCD_CITER_ELINK_TCD19_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD19_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD19_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd19_citer_elinkno::TCD_CITER_ELINK_TCD19_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd19_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD19_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD19_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD19_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd19_citer_elinkyes::TCD_CITER_ELINK_TCD19_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd19_citer_elinkyes;
#[doc = "TCD19_DLASTSGA (rw) register accessor: an alias for `Reg<TCD19_DLASTSGA_SPEC>`"]
pub type TCD19_DLASTSGA = crate::Reg<tcd19_dlastsga::TCD19_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd19_dlastsga;
#[doc = "TCD19_CSR (rw) register accessor: an alias for `Reg<TCD19_CSR_SPEC>`"]
pub type TCD19_CSR = crate::Reg<tcd19_csr::TCD19_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd19_csr;
#[doc = "TCD_BITER_ELINK_TCD19_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD19_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD19_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd19_biter_elinkno::TCD_BITER_ELINK_TCD19_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd19_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD19_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD19_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD19_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd19_biter_elinkyes::TCD_BITER_ELINK_TCD19_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd19_biter_elinkyes;
#[doc = "TCD20_SADDR (rw) register accessor: an alias for `Reg<TCD20_SADDR_SPEC>`"]
pub type TCD20_SADDR = crate::Reg<tcd20_saddr::TCD20_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd20_saddr;
#[doc = "TCD20_SOFF (rw) register accessor: an alias for `Reg<TCD20_SOFF_SPEC>`"]
pub type TCD20_SOFF = crate::Reg<tcd20_soff::TCD20_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd20_soff;
#[doc = "TCD20_ATTR (rw) register accessor: an alias for `Reg<TCD20_ATTR_SPEC>`"]
pub type TCD20_ATTR = crate::Reg<tcd20_attr::TCD20_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd20_attr;
#[doc = "TCD_NBYTES_TCD20_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD20_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD20_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd20_nbytes_mlno::TCD_NBYTES_TCD20_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd20_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD20_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD20_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD20_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd20_nbytes_mloffno::TCD_NBYTES_TCD20_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd20_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD20_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD20_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD20_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd20_nbytes_mloffyes::TCD_NBYTES_TCD20_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd20_nbytes_mloffyes;
#[doc = "TCD20_SLAST (rw) register accessor: an alias for `Reg<TCD20_SLAST_SPEC>`"]
pub type TCD20_SLAST = crate::Reg<tcd20_slast::TCD20_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd20_slast;
#[doc = "TCD20_DADDR (rw) register accessor: an alias for `Reg<TCD20_DADDR_SPEC>`"]
pub type TCD20_DADDR = crate::Reg<tcd20_daddr::TCD20_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd20_daddr;
#[doc = "TCD20_DOFF (rw) register accessor: an alias for `Reg<TCD20_DOFF_SPEC>`"]
pub type TCD20_DOFF = crate::Reg<tcd20_doff::TCD20_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd20_doff;
#[doc = "TCD_CITER_ELINK_TCD20_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD20_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD20_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd20_citer_elinkno::TCD_CITER_ELINK_TCD20_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd20_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD20_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD20_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD20_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd20_citer_elinkyes::TCD_CITER_ELINK_TCD20_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd20_citer_elinkyes;
#[doc = "TCD20_DLASTSGA (rw) register accessor: an alias for `Reg<TCD20_DLASTSGA_SPEC>`"]
pub type TCD20_DLASTSGA = crate::Reg<tcd20_dlastsga::TCD20_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd20_dlastsga;
#[doc = "TCD20_CSR (rw) register accessor: an alias for `Reg<TCD20_CSR_SPEC>`"]
pub type TCD20_CSR = crate::Reg<tcd20_csr::TCD20_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd20_csr;
#[doc = "TCD_BITER_ELINK_TCD20_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD20_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD20_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd20_biter_elinkno::TCD_BITER_ELINK_TCD20_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd20_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD20_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD20_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD20_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd20_biter_elinkyes::TCD_BITER_ELINK_TCD20_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd20_biter_elinkyes;
#[doc = "TCD21_SADDR (rw) register accessor: an alias for `Reg<TCD21_SADDR_SPEC>`"]
pub type TCD21_SADDR = crate::Reg<tcd21_saddr::TCD21_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd21_saddr;
#[doc = "TCD21_SOFF (rw) register accessor: an alias for `Reg<TCD21_SOFF_SPEC>`"]
pub type TCD21_SOFF = crate::Reg<tcd21_soff::TCD21_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd21_soff;
#[doc = "TCD21_ATTR (rw) register accessor: an alias for `Reg<TCD21_ATTR_SPEC>`"]
pub type TCD21_ATTR = crate::Reg<tcd21_attr::TCD21_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd21_attr;
#[doc = "TCD_NBYTES_TCD21_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD21_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD21_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd21_nbytes_mlno::TCD_NBYTES_TCD21_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd21_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD21_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD21_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD21_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd21_nbytes_mloffno::TCD_NBYTES_TCD21_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd21_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD21_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD21_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD21_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd21_nbytes_mloffyes::TCD_NBYTES_TCD21_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd21_nbytes_mloffyes;
#[doc = "TCD21_SLAST (rw) register accessor: an alias for `Reg<TCD21_SLAST_SPEC>`"]
pub type TCD21_SLAST = crate::Reg<tcd21_slast::TCD21_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd21_slast;
#[doc = "TCD21_DADDR (rw) register accessor: an alias for `Reg<TCD21_DADDR_SPEC>`"]
pub type TCD21_DADDR = crate::Reg<tcd21_daddr::TCD21_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd21_daddr;
#[doc = "TCD21_DOFF (rw) register accessor: an alias for `Reg<TCD21_DOFF_SPEC>`"]
pub type TCD21_DOFF = crate::Reg<tcd21_doff::TCD21_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd21_doff;
#[doc = "TCD_CITER_ELINK_TCD21_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD21_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD21_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd21_citer_elinkno::TCD_CITER_ELINK_TCD21_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd21_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD21_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD21_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD21_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd21_citer_elinkyes::TCD_CITER_ELINK_TCD21_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd21_citer_elinkyes;
#[doc = "TCD21_DLASTSGA (rw) register accessor: an alias for `Reg<TCD21_DLASTSGA_SPEC>`"]
pub type TCD21_DLASTSGA = crate::Reg<tcd21_dlastsga::TCD21_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd21_dlastsga;
#[doc = "TCD21_CSR (rw) register accessor: an alias for `Reg<TCD21_CSR_SPEC>`"]
pub type TCD21_CSR = crate::Reg<tcd21_csr::TCD21_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd21_csr;
#[doc = "TCD_BITER_ELINK_TCD21_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD21_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD21_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd21_biter_elinkno::TCD_BITER_ELINK_TCD21_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd21_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD21_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD21_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD21_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd21_biter_elinkyes::TCD_BITER_ELINK_TCD21_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd21_biter_elinkyes;
#[doc = "TCD22_SADDR (rw) register accessor: an alias for `Reg<TCD22_SADDR_SPEC>`"]
pub type TCD22_SADDR = crate::Reg<tcd22_saddr::TCD22_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd22_saddr;
#[doc = "TCD22_SOFF (rw) register accessor: an alias for `Reg<TCD22_SOFF_SPEC>`"]
pub type TCD22_SOFF = crate::Reg<tcd22_soff::TCD22_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd22_soff;
#[doc = "TCD22_ATTR (rw) register accessor: an alias for `Reg<TCD22_ATTR_SPEC>`"]
pub type TCD22_ATTR = crate::Reg<tcd22_attr::TCD22_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd22_attr;
#[doc = "TCD_NBYTES_TCD22_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD22_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD22_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd22_nbytes_mlno::TCD_NBYTES_TCD22_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd22_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD22_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD22_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD22_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd22_nbytes_mloffno::TCD_NBYTES_TCD22_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd22_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD22_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD22_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD22_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd22_nbytes_mloffyes::TCD_NBYTES_TCD22_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd22_nbytes_mloffyes;
#[doc = "TCD22_SLAST (rw) register accessor: an alias for `Reg<TCD22_SLAST_SPEC>`"]
pub type TCD22_SLAST = crate::Reg<tcd22_slast::TCD22_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd22_slast;
#[doc = "TCD22_DADDR (rw) register accessor: an alias for `Reg<TCD22_DADDR_SPEC>`"]
pub type TCD22_DADDR = crate::Reg<tcd22_daddr::TCD22_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd22_daddr;
#[doc = "TCD22_DOFF (rw) register accessor: an alias for `Reg<TCD22_DOFF_SPEC>`"]
pub type TCD22_DOFF = crate::Reg<tcd22_doff::TCD22_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd22_doff;
#[doc = "TCD_CITER_ELINK_TCD22_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD22_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD22_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd22_citer_elinkno::TCD_CITER_ELINK_TCD22_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd22_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD22_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD22_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD22_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd22_citer_elinkyes::TCD_CITER_ELINK_TCD22_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd22_citer_elinkyes;
#[doc = "TCD22_DLASTSGA (rw) register accessor: an alias for `Reg<TCD22_DLASTSGA_SPEC>`"]
pub type TCD22_DLASTSGA = crate::Reg<tcd22_dlastsga::TCD22_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd22_dlastsga;
#[doc = "TCD22_CSR (rw) register accessor: an alias for `Reg<TCD22_CSR_SPEC>`"]
pub type TCD22_CSR = crate::Reg<tcd22_csr::TCD22_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd22_csr;
#[doc = "TCD_BITER_ELINK_TCD22_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD22_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD22_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd22_biter_elinkno::TCD_BITER_ELINK_TCD22_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd22_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD22_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD22_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD22_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd22_biter_elinkyes::TCD_BITER_ELINK_TCD22_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd22_biter_elinkyes;
#[doc = "TCD23_SADDR (rw) register accessor: an alias for `Reg<TCD23_SADDR_SPEC>`"]
pub type TCD23_SADDR = crate::Reg<tcd23_saddr::TCD23_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd23_saddr;
#[doc = "TCD23_SOFF (rw) register accessor: an alias for `Reg<TCD23_SOFF_SPEC>`"]
pub type TCD23_SOFF = crate::Reg<tcd23_soff::TCD23_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd23_soff;
#[doc = "TCD23_ATTR (rw) register accessor: an alias for `Reg<TCD23_ATTR_SPEC>`"]
pub type TCD23_ATTR = crate::Reg<tcd23_attr::TCD23_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd23_attr;
#[doc = "TCD_NBYTES_TCD23_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD23_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD23_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd23_nbytes_mlno::TCD_NBYTES_TCD23_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd23_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD23_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD23_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD23_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd23_nbytes_mloffno::TCD_NBYTES_TCD23_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd23_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD23_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD23_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD23_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd23_nbytes_mloffyes::TCD_NBYTES_TCD23_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd23_nbytes_mloffyes;
#[doc = "TCD23_SLAST (rw) register accessor: an alias for `Reg<TCD23_SLAST_SPEC>`"]
pub type TCD23_SLAST = crate::Reg<tcd23_slast::TCD23_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd23_slast;
#[doc = "TCD23_DADDR (rw) register accessor: an alias for `Reg<TCD23_DADDR_SPEC>`"]
pub type TCD23_DADDR = crate::Reg<tcd23_daddr::TCD23_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd23_daddr;
#[doc = "TCD23_DOFF (rw) register accessor: an alias for `Reg<TCD23_DOFF_SPEC>`"]
pub type TCD23_DOFF = crate::Reg<tcd23_doff::TCD23_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd23_doff;
#[doc = "TCD_CITER_ELINK_TCD23_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD23_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD23_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd23_citer_elinkno::TCD_CITER_ELINK_TCD23_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd23_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD23_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD23_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD23_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd23_citer_elinkyes::TCD_CITER_ELINK_TCD23_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd23_citer_elinkyes;
#[doc = "TCD23_DLASTSGA (rw) register accessor: an alias for `Reg<TCD23_DLASTSGA_SPEC>`"]
pub type TCD23_DLASTSGA = crate::Reg<tcd23_dlastsga::TCD23_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd23_dlastsga;
#[doc = "TCD23_CSR (rw) register accessor: an alias for `Reg<TCD23_CSR_SPEC>`"]
pub type TCD23_CSR = crate::Reg<tcd23_csr::TCD23_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd23_csr;
#[doc = "TCD_BITER_ELINK_TCD23_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD23_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD23_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd23_biter_elinkno::TCD_BITER_ELINK_TCD23_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd23_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD23_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD23_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD23_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd23_biter_elinkyes::TCD_BITER_ELINK_TCD23_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd23_biter_elinkyes;
#[doc = "TCD24_SADDR (rw) register accessor: an alias for `Reg<TCD24_SADDR_SPEC>`"]
pub type TCD24_SADDR = crate::Reg<tcd24_saddr::TCD24_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd24_saddr;
#[doc = "TCD24_SOFF (rw) register accessor: an alias for `Reg<TCD24_SOFF_SPEC>`"]
pub type TCD24_SOFF = crate::Reg<tcd24_soff::TCD24_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd24_soff;
#[doc = "TCD24_ATTR (rw) register accessor: an alias for `Reg<TCD24_ATTR_SPEC>`"]
pub type TCD24_ATTR = crate::Reg<tcd24_attr::TCD24_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd24_attr;
#[doc = "TCD_NBYTES_TCD24_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD24_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD24_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd24_nbytes_mlno::TCD_NBYTES_TCD24_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd24_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD24_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD24_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD24_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd24_nbytes_mloffno::TCD_NBYTES_TCD24_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd24_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD24_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD24_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD24_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd24_nbytes_mloffyes::TCD_NBYTES_TCD24_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd24_nbytes_mloffyes;
#[doc = "TCD24_SLAST (rw) register accessor: an alias for `Reg<TCD24_SLAST_SPEC>`"]
pub type TCD24_SLAST = crate::Reg<tcd24_slast::TCD24_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd24_slast;
#[doc = "TCD24_DADDR (rw) register accessor: an alias for `Reg<TCD24_DADDR_SPEC>`"]
pub type TCD24_DADDR = crate::Reg<tcd24_daddr::TCD24_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd24_daddr;
#[doc = "TCD24_DOFF (rw) register accessor: an alias for `Reg<TCD24_DOFF_SPEC>`"]
pub type TCD24_DOFF = crate::Reg<tcd24_doff::TCD24_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd24_doff;
#[doc = "TCD_CITER_ELINK_TCD24_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD24_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD24_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd24_citer_elinkno::TCD_CITER_ELINK_TCD24_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd24_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD24_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD24_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD24_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd24_citer_elinkyes::TCD_CITER_ELINK_TCD24_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd24_citer_elinkyes;
#[doc = "TCD24_DLASTSGA (rw) register accessor: an alias for `Reg<TCD24_DLASTSGA_SPEC>`"]
pub type TCD24_DLASTSGA = crate::Reg<tcd24_dlastsga::TCD24_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd24_dlastsga;
#[doc = "TCD24_CSR (rw) register accessor: an alias for `Reg<TCD24_CSR_SPEC>`"]
pub type TCD24_CSR = crate::Reg<tcd24_csr::TCD24_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd24_csr;
#[doc = "TCD_BITER_ELINK_TCD24_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD24_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD24_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd24_biter_elinkno::TCD_BITER_ELINK_TCD24_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd24_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD24_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD24_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD24_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd24_biter_elinkyes::TCD_BITER_ELINK_TCD24_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd24_biter_elinkyes;
#[doc = "TCD25_SADDR (rw) register accessor: an alias for `Reg<TCD25_SADDR_SPEC>`"]
pub type TCD25_SADDR = crate::Reg<tcd25_saddr::TCD25_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd25_saddr;
#[doc = "TCD25_SOFF (rw) register accessor: an alias for `Reg<TCD25_SOFF_SPEC>`"]
pub type TCD25_SOFF = crate::Reg<tcd25_soff::TCD25_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd25_soff;
#[doc = "TCD25_ATTR (rw) register accessor: an alias for `Reg<TCD25_ATTR_SPEC>`"]
pub type TCD25_ATTR = crate::Reg<tcd25_attr::TCD25_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd25_attr;
#[doc = "TCD_NBYTES_TCD25_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD25_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD25_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd25_nbytes_mlno::TCD_NBYTES_TCD25_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd25_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD25_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD25_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD25_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd25_nbytes_mloffno::TCD_NBYTES_TCD25_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd25_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD25_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD25_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD25_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd25_nbytes_mloffyes::TCD_NBYTES_TCD25_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd25_nbytes_mloffyes;
#[doc = "TCD25_SLAST (rw) register accessor: an alias for `Reg<TCD25_SLAST_SPEC>`"]
pub type TCD25_SLAST = crate::Reg<tcd25_slast::TCD25_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd25_slast;
#[doc = "TCD25_DADDR (rw) register accessor: an alias for `Reg<TCD25_DADDR_SPEC>`"]
pub type TCD25_DADDR = crate::Reg<tcd25_daddr::TCD25_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd25_daddr;
#[doc = "TCD25_DOFF (rw) register accessor: an alias for `Reg<TCD25_DOFF_SPEC>`"]
pub type TCD25_DOFF = crate::Reg<tcd25_doff::TCD25_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd25_doff;
#[doc = "TCD_CITER_ELINK_TCD25_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD25_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD25_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd25_citer_elinkno::TCD_CITER_ELINK_TCD25_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd25_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD25_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD25_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD25_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd25_citer_elinkyes::TCD_CITER_ELINK_TCD25_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd25_citer_elinkyes;
#[doc = "TCD25_DLASTSGA (rw) register accessor: an alias for `Reg<TCD25_DLASTSGA_SPEC>`"]
pub type TCD25_DLASTSGA = crate::Reg<tcd25_dlastsga::TCD25_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd25_dlastsga;
#[doc = "TCD25_CSR (rw) register accessor: an alias for `Reg<TCD25_CSR_SPEC>`"]
pub type TCD25_CSR = crate::Reg<tcd25_csr::TCD25_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd25_csr;
#[doc = "TCD_BITER_ELINK_TCD25_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD25_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD25_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd25_biter_elinkno::TCD_BITER_ELINK_TCD25_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd25_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD25_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD25_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD25_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd25_biter_elinkyes::TCD_BITER_ELINK_TCD25_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd25_biter_elinkyes;
#[doc = "TCD26_SADDR (rw) register accessor: an alias for `Reg<TCD26_SADDR_SPEC>`"]
pub type TCD26_SADDR = crate::Reg<tcd26_saddr::TCD26_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd26_saddr;
#[doc = "TCD26_SOFF (rw) register accessor: an alias for `Reg<TCD26_SOFF_SPEC>`"]
pub type TCD26_SOFF = crate::Reg<tcd26_soff::TCD26_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd26_soff;
#[doc = "TCD26_ATTR (rw) register accessor: an alias for `Reg<TCD26_ATTR_SPEC>`"]
pub type TCD26_ATTR = crate::Reg<tcd26_attr::TCD26_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd26_attr;
#[doc = "TCD_NBYTES_TCD26_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD26_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD26_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd26_nbytes_mlno::TCD_NBYTES_TCD26_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd26_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD26_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD26_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD26_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd26_nbytes_mloffno::TCD_NBYTES_TCD26_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd26_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD26_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD26_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD26_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd26_nbytes_mloffyes::TCD_NBYTES_TCD26_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd26_nbytes_mloffyes;
#[doc = "TCD26_SLAST (rw) register accessor: an alias for `Reg<TCD26_SLAST_SPEC>`"]
pub type TCD26_SLAST = crate::Reg<tcd26_slast::TCD26_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd26_slast;
#[doc = "TCD26_DADDR (rw) register accessor: an alias for `Reg<TCD26_DADDR_SPEC>`"]
pub type TCD26_DADDR = crate::Reg<tcd26_daddr::TCD26_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd26_daddr;
#[doc = "TCD26_DOFF (rw) register accessor: an alias for `Reg<TCD26_DOFF_SPEC>`"]
pub type TCD26_DOFF = crate::Reg<tcd26_doff::TCD26_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd26_doff;
#[doc = "TCD_CITER_ELINK_TCD26_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD26_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD26_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd26_citer_elinkno::TCD_CITER_ELINK_TCD26_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd26_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD26_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD26_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD26_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd26_citer_elinkyes::TCD_CITER_ELINK_TCD26_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd26_citer_elinkyes;
#[doc = "TCD26_DLASTSGA (rw) register accessor: an alias for `Reg<TCD26_DLASTSGA_SPEC>`"]
pub type TCD26_DLASTSGA = crate::Reg<tcd26_dlastsga::TCD26_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd26_dlastsga;
#[doc = "TCD26_CSR (rw) register accessor: an alias for `Reg<TCD26_CSR_SPEC>`"]
pub type TCD26_CSR = crate::Reg<tcd26_csr::TCD26_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd26_csr;
#[doc = "TCD_BITER_ELINK_TCD26_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD26_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD26_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd26_biter_elinkno::TCD_BITER_ELINK_TCD26_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd26_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD26_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD26_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD26_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd26_biter_elinkyes::TCD_BITER_ELINK_TCD26_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd26_biter_elinkyes;
#[doc = "TCD27_SADDR (rw) register accessor: an alias for `Reg<TCD27_SADDR_SPEC>`"]
pub type TCD27_SADDR = crate::Reg<tcd27_saddr::TCD27_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd27_saddr;
#[doc = "TCD27_SOFF (rw) register accessor: an alias for `Reg<TCD27_SOFF_SPEC>`"]
pub type TCD27_SOFF = crate::Reg<tcd27_soff::TCD27_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd27_soff;
#[doc = "TCD27_ATTR (rw) register accessor: an alias for `Reg<TCD27_ATTR_SPEC>`"]
pub type TCD27_ATTR = crate::Reg<tcd27_attr::TCD27_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd27_attr;
#[doc = "TCD_NBYTES_TCD27_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD27_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD27_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd27_nbytes_mlno::TCD_NBYTES_TCD27_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd27_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD27_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD27_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD27_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd27_nbytes_mloffno::TCD_NBYTES_TCD27_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd27_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD27_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD27_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD27_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd27_nbytes_mloffyes::TCD_NBYTES_TCD27_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd27_nbytes_mloffyes;
#[doc = "TCD27_SLAST (rw) register accessor: an alias for `Reg<TCD27_SLAST_SPEC>`"]
pub type TCD27_SLAST = crate::Reg<tcd27_slast::TCD27_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd27_slast;
#[doc = "TCD27_DADDR (rw) register accessor: an alias for `Reg<TCD27_DADDR_SPEC>`"]
pub type TCD27_DADDR = crate::Reg<tcd27_daddr::TCD27_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd27_daddr;
#[doc = "TCD27_DOFF (rw) register accessor: an alias for `Reg<TCD27_DOFF_SPEC>`"]
pub type TCD27_DOFF = crate::Reg<tcd27_doff::TCD27_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd27_doff;
#[doc = "TCD_CITER_ELINK_TCD27_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD27_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD27_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd27_citer_elinkno::TCD_CITER_ELINK_TCD27_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd27_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD27_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD27_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD27_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd27_citer_elinkyes::TCD_CITER_ELINK_TCD27_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd27_citer_elinkyes;
#[doc = "TCD27_DLASTSGA (rw) register accessor: an alias for `Reg<TCD27_DLASTSGA_SPEC>`"]
pub type TCD27_DLASTSGA = crate::Reg<tcd27_dlastsga::TCD27_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd27_dlastsga;
#[doc = "TCD27_CSR (rw) register accessor: an alias for `Reg<TCD27_CSR_SPEC>`"]
pub type TCD27_CSR = crate::Reg<tcd27_csr::TCD27_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd27_csr;
#[doc = "TCD_BITER_ELINK_TCD27_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD27_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD27_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd27_biter_elinkno::TCD_BITER_ELINK_TCD27_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd27_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD27_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD27_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD27_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd27_biter_elinkyes::TCD_BITER_ELINK_TCD27_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd27_biter_elinkyes;
#[doc = "TCD28_SADDR (rw) register accessor: an alias for `Reg<TCD28_SADDR_SPEC>`"]
pub type TCD28_SADDR = crate::Reg<tcd28_saddr::TCD28_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd28_saddr;
#[doc = "TCD28_SOFF (rw) register accessor: an alias for `Reg<TCD28_SOFF_SPEC>`"]
pub type TCD28_SOFF = crate::Reg<tcd28_soff::TCD28_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd28_soff;
#[doc = "TCD28_ATTR (rw) register accessor: an alias for `Reg<TCD28_ATTR_SPEC>`"]
pub type TCD28_ATTR = crate::Reg<tcd28_attr::TCD28_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd28_attr;
#[doc = "TCD_NBYTES_TCD28_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD28_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD28_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd28_nbytes_mlno::TCD_NBYTES_TCD28_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd28_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD28_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD28_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD28_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd28_nbytes_mloffno::TCD_NBYTES_TCD28_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd28_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD28_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD28_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD28_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd28_nbytes_mloffyes::TCD_NBYTES_TCD28_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd28_nbytes_mloffyes;
#[doc = "TCD28_SLAST (rw) register accessor: an alias for `Reg<TCD28_SLAST_SPEC>`"]
pub type TCD28_SLAST = crate::Reg<tcd28_slast::TCD28_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd28_slast;
#[doc = "TCD28_DADDR (rw) register accessor: an alias for `Reg<TCD28_DADDR_SPEC>`"]
pub type TCD28_DADDR = crate::Reg<tcd28_daddr::TCD28_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd28_daddr;
#[doc = "TCD28_DOFF (rw) register accessor: an alias for `Reg<TCD28_DOFF_SPEC>`"]
pub type TCD28_DOFF = crate::Reg<tcd28_doff::TCD28_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd28_doff;
#[doc = "TCD_CITER_ELINK_TCD28_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD28_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD28_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd28_citer_elinkno::TCD_CITER_ELINK_TCD28_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd28_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD28_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD28_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD28_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd28_citer_elinkyes::TCD_CITER_ELINK_TCD28_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd28_citer_elinkyes;
#[doc = "TCD28_DLASTSGA (rw) register accessor: an alias for `Reg<TCD28_DLASTSGA_SPEC>`"]
pub type TCD28_DLASTSGA = crate::Reg<tcd28_dlastsga::TCD28_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd28_dlastsga;
#[doc = "TCD28_CSR (rw) register accessor: an alias for `Reg<TCD28_CSR_SPEC>`"]
pub type TCD28_CSR = crate::Reg<tcd28_csr::TCD28_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd28_csr;
#[doc = "TCD_BITER_ELINK_TCD28_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD28_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD28_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd28_biter_elinkno::TCD_BITER_ELINK_TCD28_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd28_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD28_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD28_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD28_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd28_biter_elinkyes::TCD_BITER_ELINK_TCD28_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd28_biter_elinkyes;
#[doc = "TCD29_SADDR (rw) register accessor: an alias for `Reg<TCD29_SADDR_SPEC>`"]
pub type TCD29_SADDR = crate::Reg<tcd29_saddr::TCD29_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd29_saddr;
#[doc = "TCD29_SOFF (rw) register accessor: an alias for `Reg<TCD29_SOFF_SPEC>`"]
pub type TCD29_SOFF = crate::Reg<tcd29_soff::TCD29_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd29_soff;
#[doc = "TCD29_ATTR (rw) register accessor: an alias for `Reg<TCD29_ATTR_SPEC>`"]
pub type TCD29_ATTR = crate::Reg<tcd29_attr::TCD29_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd29_attr;
#[doc = "TCD_NBYTES_TCD29_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD29_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD29_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd29_nbytes_mlno::TCD_NBYTES_TCD29_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd29_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD29_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD29_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD29_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd29_nbytes_mloffno::TCD_NBYTES_TCD29_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd29_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD29_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD29_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD29_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd29_nbytes_mloffyes::TCD_NBYTES_TCD29_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd29_nbytes_mloffyes;
#[doc = "TCD29_SLAST (rw) register accessor: an alias for `Reg<TCD29_SLAST_SPEC>`"]
pub type TCD29_SLAST = crate::Reg<tcd29_slast::TCD29_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd29_slast;
#[doc = "TCD29_DADDR (rw) register accessor: an alias for `Reg<TCD29_DADDR_SPEC>`"]
pub type TCD29_DADDR = crate::Reg<tcd29_daddr::TCD29_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd29_daddr;
#[doc = "TCD29_DOFF (rw) register accessor: an alias for `Reg<TCD29_DOFF_SPEC>`"]
pub type TCD29_DOFF = crate::Reg<tcd29_doff::TCD29_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd29_doff;
#[doc = "TCD_CITER_ELINK_TCD29_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD29_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD29_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd29_citer_elinkno::TCD_CITER_ELINK_TCD29_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd29_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD29_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD29_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD29_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd29_citer_elinkyes::TCD_CITER_ELINK_TCD29_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd29_citer_elinkyes;
#[doc = "TCD29_DLASTSGA (rw) register accessor: an alias for `Reg<TCD29_DLASTSGA_SPEC>`"]
pub type TCD29_DLASTSGA = crate::Reg<tcd29_dlastsga::TCD29_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd29_dlastsga;
#[doc = "TCD29_CSR (rw) register accessor: an alias for `Reg<TCD29_CSR_SPEC>`"]
pub type TCD29_CSR = crate::Reg<tcd29_csr::TCD29_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd29_csr;
#[doc = "TCD_BITER_ELINK_TCD29_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD29_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD29_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd29_biter_elinkno::TCD_BITER_ELINK_TCD29_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd29_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD29_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD29_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD29_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd29_biter_elinkyes::TCD_BITER_ELINK_TCD29_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd29_biter_elinkyes;
#[doc = "TCD30_SADDR (rw) register accessor: an alias for `Reg<TCD30_SADDR_SPEC>`"]
pub type TCD30_SADDR = crate::Reg<tcd30_saddr::TCD30_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd30_saddr;
#[doc = "TCD30_SOFF (rw) register accessor: an alias for `Reg<TCD30_SOFF_SPEC>`"]
pub type TCD30_SOFF = crate::Reg<tcd30_soff::TCD30_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd30_soff;
#[doc = "TCD30_ATTR (rw) register accessor: an alias for `Reg<TCD30_ATTR_SPEC>`"]
pub type TCD30_ATTR = crate::Reg<tcd30_attr::TCD30_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd30_attr;
#[doc = "TCD_NBYTES_TCD30_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD30_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD30_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd30_nbytes_mlno::TCD_NBYTES_TCD30_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd30_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD30_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD30_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD30_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd30_nbytes_mloffno::TCD_NBYTES_TCD30_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd30_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD30_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD30_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD30_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd30_nbytes_mloffyes::TCD_NBYTES_TCD30_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd30_nbytes_mloffyes;
#[doc = "TCD30_SLAST (rw) register accessor: an alias for `Reg<TCD30_SLAST_SPEC>`"]
pub type TCD30_SLAST = crate::Reg<tcd30_slast::TCD30_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd30_slast;
#[doc = "TCD30_DADDR (rw) register accessor: an alias for `Reg<TCD30_DADDR_SPEC>`"]
pub type TCD30_DADDR = crate::Reg<tcd30_daddr::TCD30_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd30_daddr;
#[doc = "TCD30_DOFF (rw) register accessor: an alias for `Reg<TCD30_DOFF_SPEC>`"]
pub type TCD30_DOFF = crate::Reg<tcd30_doff::TCD30_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd30_doff;
#[doc = "TCD_CITER_ELINK_TCD30_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD30_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD30_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd30_citer_elinkno::TCD_CITER_ELINK_TCD30_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd30_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD30_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD30_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD30_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd30_citer_elinkyes::TCD_CITER_ELINK_TCD30_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd30_citer_elinkyes;
#[doc = "TCD30_DLASTSGA (rw) register accessor: an alias for `Reg<TCD30_DLASTSGA_SPEC>`"]
pub type TCD30_DLASTSGA = crate::Reg<tcd30_dlastsga::TCD30_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd30_dlastsga;
#[doc = "TCD30_CSR (rw) register accessor: an alias for `Reg<TCD30_CSR_SPEC>`"]
pub type TCD30_CSR = crate::Reg<tcd30_csr::TCD30_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd30_csr;
#[doc = "TCD_BITER_ELINK_TCD30_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD30_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd30_biter_elinkno::TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd30_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD30_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD30_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD30_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd30_biter_elinkyes::TCD_BITER_ELINK_TCD30_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd30_biter_elinkyes;
#[doc = "TCD31_SADDR (rw) register accessor: an alias for `Reg<TCD31_SADDR_SPEC>`"]
pub type TCD31_SADDR = crate::Reg<tcd31_saddr::TCD31_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd31_saddr;
#[doc = "TCD31_SOFF (rw) register accessor: an alias for `Reg<TCD31_SOFF_SPEC>`"]
pub type TCD31_SOFF = crate::Reg<tcd31_soff::TCD31_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd31_soff;
#[doc = "TCD31_ATTR (rw) register accessor: an alias for `Reg<TCD31_ATTR_SPEC>`"]
pub type TCD31_ATTR = crate::Reg<tcd31_attr::TCD31_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd31_attr;
#[doc = "TCD_NBYTES_TCD31_NBYTES_MLNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD31_NBYTES_MLNO_SPEC>`"]
pub type TCD_NBYTES_TCD31_NBYTES_MLNO =
    crate::Reg<tcd_nbytes_tcd31_nbytes_mlno::TCD_NBYTES_TCD31_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_tcd31_nbytes_mlno;
#[doc = "TCD_NBYTES_TCD31_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD31_NBYTES_MLOFFNO_SPEC>`"]
pub type TCD_NBYTES_TCD31_NBYTES_MLOFFNO =
    crate::Reg<tcd_nbytes_tcd31_nbytes_mloffno::TCD_NBYTES_TCD31_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_tcd31_nbytes_mloffno;
#[doc = "TCD_NBYTES_TCD31_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<TCD_NBYTES_TCD31_NBYTES_MLOFFYES_SPEC>`"]
pub type TCD_NBYTES_TCD31_NBYTES_MLOFFYES =
    crate::Reg<tcd_nbytes_tcd31_nbytes_mloffyes::TCD_NBYTES_TCD31_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_tcd31_nbytes_mloffyes;
#[doc = "TCD31_SLAST (rw) register accessor: an alias for `Reg<TCD31_SLAST_SPEC>`"]
pub type TCD31_SLAST = crate::Reg<tcd31_slast::TCD31_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd31_slast;
#[doc = "TCD31_DADDR (rw) register accessor: an alias for `Reg<TCD31_DADDR_SPEC>`"]
pub type TCD31_DADDR = crate::Reg<tcd31_daddr::TCD31_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd31_daddr;
#[doc = "TCD31_DOFF (rw) register accessor: an alias for `Reg<TCD31_DOFF_SPEC>`"]
pub type TCD31_DOFF = crate::Reg<tcd31_doff::TCD31_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd31_doff;
#[doc = "TCD_CITER_ELINK_TCD31_CITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD31_CITER_ELINKNO_SPEC>`"]
pub type TCD_CITER_ELINK_TCD31_CITER_ELINKNO =
    crate::Reg<tcd_citer_elink_tcd31_citer_elinkno::TCD_CITER_ELINK_TCD31_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elink_tcd31_citer_elinkno;
#[doc = "TCD_CITER_ELINK_TCD31_CITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_CITER_ELINK_TCD31_CITER_ELINKYES_SPEC>`"]
pub type TCD_CITER_ELINK_TCD31_CITER_ELINKYES =
    crate::Reg<tcd_citer_elink_tcd31_citer_elinkyes::TCD_CITER_ELINK_TCD31_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elink_tcd31_citer_elinkyes;
#[doc = "TCD31_DLASTSGA (rw) register accessor: an alias for `Reg<TCD31_DLASTSGA_SPEC>`"]
pub type TCD31_DLASTSGA = crate::Reg<tcd31_dlastsga::TCD31_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd31_dlastsga;
#[doc = "TCD31_CSR (rw) register accessor: an alias for `Reg<TCD31_CSR_SPEC>`"]
pub type TCD31_CSR = crate::Reg<tcd31_csr::TCD31_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd31_csr;
#[doc = "TCD_BITER_ELINK_TCD31_BITER_ELINKNO (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD31_BITER_ELINKNO_SPEC>`"]
pub type TCD_BITER_ELINK_TCD31_BITER_ELINKNO =
    crate::Reg<tcd_biter_elink_tcd31_biter_elinkno::TCD_BITER_ELINK_TCD31_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elink_tcd31_biter_elinkno;
#[doc = "TCD_BITER_ELINK_TCD31_BITER_ELINKYES (rw) register accessor: an alias for `Reg<TCD_BITER_ELINK_TCD31_BITER_ELINKYES_SPEC>`"]
pub type TCD_BITER_ELINK_TCD31_BITER_ELINKYES =
    crate::Reg<tcd_biter_elink_tcd31_biter_elinkyes::TCD_BITER_ELINK_TCD31_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elink_tcd31_biter_elinkyes;
