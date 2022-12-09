#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BEE_ENABLE` reader - BEE enable bit"]
pub type BEE_ENABLE_R = crate::BitReader<BEE_ENABLE_A>;
#[doc = "BEE enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEE_ENABLE_A {
    #[doc = "0: Disable BEE"]
    BEE_ENABLE_0 = 0,
    #[doc = "1: Enable BEE"]
    BEE_ENABLE_1 = 1,
}
impl From<BEE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BEE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl BEE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEE_ENABLE_A {
        match self.bits {
            false => BEE_ENABLE_A::BEE_ENABLE_0,
            true => BEE_ENABLE_A::BEE_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEE_ENABLE_0`"]
    #[inline(always)]
    pub fn is_bee_enable_0(&self) -> bool {
        *self == BEE_ENABLE_A::BEE_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `BEE_ENABLE_1`"]
    #[inline(always)]
    pub fn is_bee_enable_1(&self) -> bool {
        *self == BEE_ENABLE_A::BEE_ENABLE_1
    }
}
#[doc = "Field `BEE_ENABLE` writer - BEE enable bit"]
pub type BEE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BEE_ENABLE_A, O>;
impl<'a, const O: u8> BEE_ENABLE_W<'a, O> {
    #[doc = "Disable BEE"]
    #[inline(always)]
    pub fn bee_enable_0(self) -> &'a mut W {
        self.variant(BEE_ENABLE_A::BEE_ENABLE_0)
    }
    #[doc = "Enable BEE"]
    #[inline(always)]
    pub fn bee_enable_1(self) -> &'a mut W {
        self.variant(BEE_ENABLE_A::BEE_ENABLE_1)
    }
}
#[doc = "Field `CTRL_CLK_EN` reader - Clock enable input, low inactive"]
pub type CTRL_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_CLK_EN` writer - Clock enable input, low inactive"]
pub type CTRL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CTRL_SFTRST_N` reader - Soft reset input, low active"]
pub type CTRL_SFTRST_N_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_SFTRST_N` writer - Soft reset input, low active"]
pub type CTRL_SFTRST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEY_VALID` reader - AES-128 key is ready Load AES key by changing this bit from 0 to 1."]
pub type KEY_VALID_R = crate::BitReader<bool>;
#[doc = "Field `KEY_VALID` writer - AES-128 key is ready Load AES key by changing this bit from 0 to 1."]
pub type KEY_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEY_REGION_SEL` reader - AES key region select"]
pub type KEY_REGION_SEL_R = crate::BitReader<KEY_REGION_SEL_A>;
#[doc = "AES key region select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEY_REGION_SEL_A {
    #[doc = "0: Load AES key for region0"]
    KEY_REGION_SEL_0 = 0,
    #[doc = "1: Load AES key for region1"]
    KEY_REGION_SEL_1 = 1,
}
impl From<KEY_REGION_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: KEY_REGION_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl KEY_REGION_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY_REGION_SEL_A {
        match self.bits {
            false => KEY_REGION_SEL_A::KEY_REGION_SEL_0,
            true => KEY_REGION_SEL_A::KEY_REGION_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `KEY_REGION_SEL_0`"]
    #[inline(always)]
    pub fn is_key_region_sel_0(&self) -> bool {
        *self == KEY_REGION_SEL_A::KEY_REGION_SEL_0
    }
    #[doc = "Checks if the value of the field is `KEY_REGION_SEL_1`"]
    #[inline(always)]
    pub fn is_key_region_sel_1(&self) -> bool {
        *self == KEY_REGION_SEL_A::KEY_REGION_SEL_1
    }
}
#[doc = "Field `KEY_REGION_SEL` writer - AES key region select"]
pub type KEY_REGION_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, KEY_REGION_SEL_A, O>;
impl<'a, const O: u8> KEY_REGION_SEL_W<'a, O> {
    #[doc = "Load AES key for region0"]
    #[inline(always)]
    pub fn key_region_sel_0(self) -> &'a mut W {
        self.variant(KEY_REGION_SEL_A::KEY_REGION_SEL_0)
    }
    #[doc = "Load AES key for region1"]
    #[inline(always)]
    pub fn key_region_sel_1(self) -> &'a mut W {
        self.variant(KEY_REGION_SEL_A::KEY_REGION_SEL_1)
    }
}
#[doc = "Field `AC_PROT_EN` reader - Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
pub type AC_PROT_EN_R = crate::BitReader<bool>;
#[doc = "Field `AC_PROT_EN` writer - Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
pub type AC_PROT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LITTLE_ENDIAN` reader - Endian swap control for the 16 bytes input and output data of AES core."]
pub type LITTLE_ENDIAN_R = crate::BitReader<LITTLE_ENDIAN_A>;
#[doc = "Endian swap control for the 16 bytes input and output data of AES core.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LITTLE_ENDIAN_A {
    #[doc = "0: The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    LITTLE_ENDIAN_0 = 0,
    #[doc = "1: The input and output data of AES core is not swapped."]
    LITTLE_ENDIAN_1 = 1,
}
impl From<LITTLE_ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: LITTLE_ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
impl LITTLE_ENDIAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LITTLE_ENDIAN_A {
        match self.bits {
            false => LITTLE_ENDIAN_A::LITTLE_ENDIAN_0,
            true => LITTLE_ENDIAN_A::LITTLE_ENDIAN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_0`"]
    #[inline(always)]
    pub fn is_little_endian_0(&self) -> bool {
        *self == LITTLE_ENDIAN_A::LITTLE_ENDIAN_0
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_1`"]
    #[inline(always)]
    pub fn is_little_endian_1(&self) -> bool {
        *self == LITTLE_ENDIAN_A::LITTLE_ENDIAN_1
    }
}
#[doc = "Field `LITTLE_ENDIAN` writer - Endian swap control for the 16 bytes input and output data of AES core."]
pub type LITTLE_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, LITTLE_ENDIAN_A, O>;
impl<'a, const O: u8> LITTLE_ENDIAN_W<'a, O> {
    #[doc = "The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    #[inline(always)]
    pub fn little_endian_0(self) -> &'a mut W {
        self.variant(LITTLE_ENDIAN_A::LITTLE_ENDIAN_0)
    }
    #[doc = "The input and output data of AES core is not swapped."]
    #[inline(always)]
    pub fn little_endian_1(self) -> &'a mut W {
        self.variant(LITTLE_ENDIAN_A::LITTLE_ENDIAN_1)
    }
}
#[doc = "Field `SECURITY_LEVEL_R0` reader - Security level of the allowed access for memory region0"]
pub type SECURITY_LEVEL_R0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY_LEVEL_R0` writer - Security level of the allowed access for memory region0"]
pub type SECURITY_LEVEL_R0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CTRL_AES_MODE_R0` reader - AES mode of region0"]
pub type CTRL_AES_MODE_R0_R = crate::BitReader<CTRL_AES_MODE_R0_A>;
#[doc = "AES mode of region0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRL_AES_MODE_R0_A {
    #[doc = "0: ECB"]
    CTRL_AES_MODE_R0_0 = 0,
    #[doc = "1: CTR"]
    CTRL_AES_MODE_R0_1 = 1,
}
impl From<CTRL_AES_MODE_R0_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_AES_MODE_R0_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRL_AES_MODE_R0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_AES_MODE_R0_A {
        match self.bits {
            false => CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_0,
            true => CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R0_0`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r0_0(&self) -> bool {
        *self == CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_0
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R0_1`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r0_1(&self) -> bool {
        *self == CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_1
    }
}
#[doc = "Field `CTRL_AES_MODE_R0` writer - AES mode of region0"]
pub type CTRL_AES_MODE_R0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, CTRL_AES_MODE_R0_A, O>;
impl<'a, const O: u8> CTRL_AES_MODE_R0_W<'a, O> {
    #[doc = "ECB"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0_0(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_0)
    }
    #[doc = "CTR"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0_1(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_1)
    }
}
#[doc = "Field `SECURITY_LEVEL_R1` reader - Security level of the allowed access for memory region1"]
pub type SECURITY_LEVEL_R1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY_LEVEL_R1` writer - Security level of the allowed access for memory region1"]
pub type SECURITY_LEVEL_R1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CTRL_AES_MODE_R1` reader - AES mode of region1"]
pub type CTRL_AES_MODE_R1_R = crate::BitReader<CTRL_AES_MODE_R1_A>;
#[doc = "AES mode of region1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRL_AES_MODE_R1_A {
    #[doc = "0: ECB"]
    CTRL_AES_MODE_R1_0 = 0,
    #[doc = "1: CTR"]
    CTRL_AES_MODE_R1_1 = 1,
}
impl From<CTRL_AES_MODE_R1_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_AES_MODE_R1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRL_AES_MODE_R1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_AES_MODE_R1_A {
        match self.bits {
            false => CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_0,
            true => CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R1_0`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r1_0(&self) -> bool {
        *self == CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_0
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R1_1`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r1_1(&self) -> bool {
        *self == CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_1
    }
}
#[doc = "Field `CTRL_AES_MODE_R1` writer - AES mode of region1"]
pub type CTRL_AES_MODE_R1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, CTRL_AES_MODE_R1_A, O>;
impl<'a, const O: u8> CTRL_AES_MODE_R1_W<'a, O> {
    #[doc = "ECB"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1_0(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_0)
    }
    #[doc = "CTR"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1_1(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_1)
    }
}
#[doc = "Field `BEE_ENABLE_LOCK` reader - Lock bit for bee_enable"]
pub type BEE_ENABLE_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `BEE_ENABLE_LOCK` writer - Lock bit for bee_enable"]
pub type BEE_ENABLE_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CTRL_CLK_EN_LOCK` reader - Lock bit for ctrl_clk_en"]
pub type CTRL_CLK_EN_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_CLK_EN_LOCK` writer - Lock bit for ctrl_clk_en"]
pub type CTRL_CLK_EN_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CTRL_SFTRST_N_LOCK` reader - Lock bit for ctrl_sftrst"]
pub type CTRL_SFTRST_N_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_SFTRST_N_LOCK` writer - Lock bit for ctrl_sftrst"]
pub type CTRL_SFTRST_N_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REGION1_ADDR_LOCK` reader - Lock bit for region1 address boundary"]
pub type REGION1_ADDR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `REGION1_ADDR_LOCK` writer - Lock bit for region1 address boundary"]
pub type REGION1_ADDR_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEY_VALID_LOCK` reader - Lock bit for key_valid"]
pub type KEY_VALID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `KEY_VALID_LOCK` writer - Lock bit for key_valid"]
pub type KEY_VALID_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEY_REGION_SEL_LOCK` reader - Lock bit for key_region_sel"]
pub type KEY_REGION_SEL_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `KEY_REGION_SEL_LOCK` writer - Lock bit for key_region_sel"]
pub type KEY_REGION_SEL_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AC_PROT_EN_LOCK` reader - Lock bit for ac_prot"]
pub type AC_PROT_EN_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `AC_PROT_EN_LOCK` writer - Lock bit for ac_prot"]
pub type AC_PROT_EN_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LITTLE_ENDIAN_LOCK` reader - Lock bit for little_endian"]
pub type LITTLE_ENDIAN_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LITTLE_ENDIAN_LOCK` writer - Lock bit for little_endian"]
pub type LITTLE_ENDIAN_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SECURITY_LEVEL_R0_LOCK` reader - Lock bits for security_level_r0"]
pub type SECURITY_LEVEL_R0_LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY_LEVEL_R0_LOCK` writer - Lock bits for security_level_r0"]
pub type SECURITY_LEVEL_R0_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CTRL_AES_MODE_R0_LOCK` reader - Lock bit for region0 ctrl_aes_mode"]
pub type CTRL_AES_MODE_R0_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_AES_MODE_R0_LOCK` writer - Lock bit for region0 ctrl_aes_mode"]
pub type CTRL_AES_MODE_R0_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REGION0_KEY_LOCK` reader - Lock bit for region0 AES key"]
pub type REGION0_KEY_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `REGION0_KEY_LOCK` writer - Lock bit for region0 AES key"]
pub type REGION0_KEY_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SECURITY_LEVEL_R1_LOCK` reader - Lock bits for security_level_r1"]
pub type SECURITY_LEVEL_R1_LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY_LEVEL_R1_LOCK` writer - Lock bits for security_level_r1"]
pub type SECURITY_LEVEL_R1_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CTRL_AES_MODE_R1_LOCK` reader - Lock bit for region1 ctrl_aes_mode"]
pub type CTRL_AES_MODE_R1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_AES_MODE_R1_LOCK` writer - Lock bit for region1 ctrl_aes_mode"]
pub type CTRL_AES_MODE_R1_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REGION1_KEY_LOCK` reader - Lock bit for region1 AES key"]
pub type REGION1_KEY_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `REGION1_KEY_LOCK` writer - Lock bit for region1 AES key"]
pub type REGION1_KEY_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BEE enable bit"]
    #[inline(always)]
    pub fn bee_enable(&self) -> BEE_ENABLE_R {
        BEE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock enable input, low inactive"]
    #[inline(always)]
    pub fn ctrl_clk_en(&self) -> CTRL_CLK_EN_R {
        CTRL_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Soft reset input, low active"]
    #[inline(always)]
    pub fn ctrl_sftrst_n(&self) -> CTRL_SFTRST_N_R {
        CTRL_SFTRST_N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - AES-128 key is ready Load AES key by changing this bit from 0 to 1."]
    #[inline(always)]
    pub fn key_valid(&self) -> KEY_VALID_R {
        KEY_VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AES key region select"]
    #[inline(always)]
    pub fn key_region_sel(&self) -> KEY_REGION_SEL_R {
        KEY_REGION_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
    #[inline(always)]
    pub fn ac_prot_en(&self) -> AC_PROT_EN_R {
        AC_PROT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endian swap control for the 16 bytes input and output data of AES core."]
    #[inline(always)]
    pub fn little_endian(&self) -> LITTLE_ENDIAN_R {
        LITTLE_ENDIAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Security level of the allowed access for memory region0"]
    #[inline(always)]
    pub fn security_level_r0(&self) -> SECURITY_LEVEL_R0_R {
        SECURITY_LEVEL_R0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - AES mode of region0"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0(&self) -> CTRL_AES_MODE_R0_R {
        CTRL_AES_MODE_R0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Security level of the allowed access for memory region1"]
    #[inline(always)]
    pub fn security_level_r1(&self) -> SECURITY_LEVEL_R1_R {
        SECURITY_LEVEL_R1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - AES mode of region1"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1(&self) -> CTRL_AES_MODE_R1_R {
        CTRL_AES_MODE_R1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock bit for bee_enable"]
    #[inline(always)]
    pub fn bee_enable_lock(&self) -> BEE_ENABLE_LOCK_R {
        BEE_ENABLE_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock bit for ctrl_clk_en"]
    #[inline(always)]
    pub fn ctrl_clk_en_lock(&self) -> CTRL_CLK_EN_LOCK_R {
        CTRL_CLK_EN_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Lock bit for ctrl_sftrst"]
    #[inline(always)]
    pub fn ctrl_sftrst_n_lock(&self) -> CTRL_SFTRST_N_LOCK_R {
        CTRL_SFTRST_N_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock bit for region1 address boundary"]
    #[inline(always)]
    pub fn region1_addr_lock(&self) -> REGION1_ADDR_LOCK_R {
        REGION1_ADDR_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Lock bit for key_valid"]
    #[inline(always)]
    pub fn key_valid_lock(&self) -> KEY_VALID_LOCK_R {
        KEY_VALID_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock bit for key_region_sel"]
    #[inline(always)]
    pub fn key_region_sel_lock(&self) -> KEY_REGION_SEL_LOCK_R {
        KEY_REGION_SEL_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Lock bit for ac_prot"]
    #[inline(always)]
    pub fn ac_prot_en_lock(&self) -> AC_PROT_EN_LOCK_R {
        AC_PROT_EN_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Lock bit for little_endian"]
    #[inline(always)]
    pub fn little_endian_lock(&self) -> LITTLE_ENDIAN_LOCK_R {
        LITTLE_ENDIAN_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Lock bits for security_level_r0"]
    #[inline(always)]
    pub fn security_level_r0_lock(&self) -> SECURITY_LEVEL_R0_LOCK_R {
        SECURITY_LEVEL_R0_LOCK_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Lock bit for region0 ctrl_aes_mode"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0_lock(&self) -> CTRL_AES_MODE_R0_LOCK_R {
        CTRL_AES_MODE_R0_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lock bit for region0 AES key"]
    #[inline(always)]
    pub fn region0_key_lock(&self) -> REGION0_KEY_LOCK_R {
        REGION0_KEY_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Lock bits for security_level_r1"]
    #[inline(always)]
    pub fn security_level_r1_lock(&self) -> SECURITY_LEVEL_R1_LOCK_R {
        SECURITY_LEVEL_R1_LOCK_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Lock bit for region1 ctrl_aes_mode"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1_lock(&self) -> CTRL_AES_MODE_R1_LOCK_R {
        CTRL_AES_MODE_R1_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock bit for region1 AES key"]
    #[inline(always)]
    pub fn region1_key_lock(&self) -> REGION1_KEY_LOCK_R {
        REGION1_KEY_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BEE enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bee_enable(&mut self) -> BEE_ENABLE_W<0> {
        BEE_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Clock enable input, low inactive"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_clk_en(&mut self) -> CTRL_CLK_EN_W<1> {
        CTRL_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - Soft reset input, low active"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_sftrst_n(&mut self) -> CTRL_SFTRST_N_W<2> {
        CTRL_SFTRST_N_W::new(self)
    }
    #[doc = "Bit 4 - AES-128 key is ready Load AES key by changing this bit from 0 to 1."]
    #[inline(always)]
    #[must_use]
    pub fn key_valid(&mut self) -> KEY_VALID_W<4> {
        KEY_VALID_W::new(self)
    }
    #[doc = "Bit 5 - AES key region select"]
    #[inline(always)]
    #[must_use]
    pub fn key_region_sel(&mut self) -> KEY_REGION_SEL_W<5> {
        KEY_REGION_SEL_W::new(self)
    }
    #[doc = "Bit 6 - Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
    #[inline(always)]
    #[must_use]
    pub fn ac_prot_en(&mut self) -> AC_PROT_EN_W<6> {
        AC_PROT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Endian swap control for the 16 bytes input and output data of AES core."]
    #[inline(always)]
    #[must_use]
    pub fn little_endian(&mut self) -> LITTLE_ENDIAN_W<7> {
        LITTLE_ENDIAN_W::new(self)
    }
    #[doc = "Bits 8:9 - Security level of the allowed access for memory region0"]
    #[inline(always)]
    #[must_use]
    pub fn security_level_r0(&mut self) -> SECURITY_LEVEL_R0_W<8> {
        SECURITY_LEVEL_R0_W::new(self)
    }
    #[doc = "Bit 10 - AES mode of region0"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_aes_mode_r0(&mut self) -> CTRL_AES_MODE_R0_W<10> {
        CTRL_AES_MODE_R0_W::new(self)
    }
    #[doc = "Bits 12:13 - Security level of the allowed access for memory region1"]
    #[inline(always)]
    #[must_use]
    pub fn security_level_r1(&mut self) -> SECURITY_LEVEL_R1_W<12> {
        SECURITY_LEVEL_R1_W::new(self)
    }
    #[doc = "Bit 14 - AES mode of region1"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_aes_mode_r1(&mut self) -> CTRL_AES_MODE_R1_W<14> {
        CTRL_AES_MODE_R1_W::new(self)
    }
    #[doc = "Bit 16 - Lock bit for bee_enable"]
    #[inline(always)]
    #[must_use]
    pub fn bee_enable_lock(&mut self) -> BEE_ENABLE_LOCK_W<16> {
        BEE_ENABLE_LOCK_W::new(self)
    }
    #[doc = "Bit 17 - Lock bit for ctrl_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_clk_en_lock(&mut self) -> CTRL_CLK_EN_LOCK_W<17> {
        CTRL_CLK_EN_LOCK_W::new(self)
    }
    #[doc = "Bit 18 - Lock bit for ctrl_sftrst"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_sftrst_n_lock(&mut self) -> CTRL_SFTRST_N_LOCK_W<18> {
        CTRL_SFTRST_N_LOCK_W::new(self)
    }
    #[doc = "Bit 19 - Lock bit for region1 address boundary"]
    #[inline(always)]
    #[must_use]
    pub fn region1_addr_lock(&mut self) -> REGION1_ADDR_LOCK_W<19> {
        REGION1_ADDR_LOCK_W::new(self)
    }
    #[doc = "Bit 20 - Lock bit for key_valid"]
    #[inline(always)]
    #[must_use]
    pub fn key_valid_lock(&mut self) -> KEY_VALID_LOCK_W<20> {
        KEY_VALID_LOCK_W::new(self)
    }
    #[doc = "Bit 21 - Lock bit for key_region_sel"]
    #[inline(always)]
    #[must_use]
    pub fn key_region_sel_lock(&mut self) -> KEY_REGION_SEL_LOCK_W<21> {
        KEY_REGION_SEL_LOCK_W::new(self)
    }
    #[doc = "Bit 22 - Lock bit for ac_prot"]
    #[inline(always)]
    #[must_use]
    pub fn ac_prot_en_lock(&mut self) -> AC_PROT_EN_LOCK_W<22> {
        AC_PROT_EN_LOCK_W::new(self)
    }
    #[doc = "Bit 23 - Lock bit for little_endian"]
    #[inline(always)]
    #[must_use]
    pub fn little_endian_lock(&mut self) -> LITTLE_ENDIAN_LOCK_W<23> {
        LITTLE_ENDIAN_LOCK_W::new(self)
    }
    #[doc = "Bits 24:25 - Lock bits for security_level_r0"]
    #[inline(always)]
    #[must_use]
    pub fn security_level_r0_lock(&mut self) -> SECURITY_LEVEL_R0_LOCK_W<24> {
        SECURITY_LEVEL_R0_LOCK_W::new(self)
    }
    #[doc = "Bit 26 - Lock bit for region0 ctrl_aes_mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_aes_mode_r0_lock(&mut self) -> CTRL_AES_MODE_R0_LOCK_W<26> {
        CTRL_AES_MODE_R0_LOCK_W::new(self)
    }
    #[doc = "Bit 27 - Lock bit for region0 AES key"]
    #[inline(always)]
    #[must_use]
    pub fn region0_key_lock(&mut self) -> REGION0_KEY_LOCK_W<27> {
        REGION0_KEY_LOCK_W::new(self)
    }
    #[doc = "Bits 28:29 - Lock bits for security_level_r1"]
    #[inline(always)]
    #[must_use]
    pub fn security_level_r1_lock(&mut self) -> SECURITY_LEVEL_R1_LOCK_W<28> {
        SECURITY_LEVEL_R1_LOCK_W::new(self)
    }
    #[doc = "Bit 30 - Lock bit for region1 ctrl_aes_mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_aes_mode_r1_lock(&mut self) -> CTRL_AES_MODE_R1_LOCK_W<30> {
        CTRL_AES_MODE_R1_LOCK_W::new(self)
    }
    #[doc = "Bit 31 - Lock bit for region1 AES key"]
    #[inline(always)]
    #[must_use]
    pub fn region1_key_lock(&mut self) -> REGION1_KEY_LOCK_W<31> {
        REGION1_KEY_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x7700"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7700;
}
