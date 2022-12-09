#[doc = "Register `MMC_BOOT` reader"]
pub struct R(crate::R<MMC_BOOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_BOOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_BOOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_BOOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_BOOT` writer"]
pub struct W(crate::W<MMC_BOOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_BOOT_SPEC>;
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
impl From<crate::W<MMC_BOOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_BOOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTOCV_ACK` reader - Boot ACK time out"]
pub type DTOCV_ACK_R = crate::FieldReader<u8, DTOCV_ACK_A>;
#[doc = "Boot ACK time out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTOCV_ACK_A {
    #[doc = "0: SDCLK x 2^14"]
    DTOCV_ACK_0 = 0,
    #[doc = "1: SDCLK x 2^15"]
    DTOCV_ACK_1 = 1,
    #[doc = "2: SDCLK x 2^16"]
    DTOCV_ACK_2 = 2,
    #[doc = "3: SDCLK x 2^17"]
    DTOCV_ACK_3 = 3,
    #[doc = "4: SDCLK x 2^18"]
    DTOCV_ACK_4 = 4,
    #[doc = "5: SDCLK x 2^19"]
    DTOCV_ACK_5 = 5,
    #[doc = "6: SDCLK x 2^20"]
    DTOCV_ACK_6 = 6,
    #[doc = "7: SDCLK x 2^21"]
    DTOCV_ACK_7 = 7,
    #[doc = "14: SDCLK x 2^28"]
    DTOCV_ACK_14 = 14,
    #[doc = "15: SDCLK x 2^29"]
    DTOCV_ACK_15 = 15,
}
impl From<DTOCV_ACK_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCV_ACK_A) -> Self {
        variant as _
    }
}
impl DTOCV_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTOCV_ACK_A> {
        match self.bits {
            0 => Some(DTOCV_ACK_A::DTOCV_ACK_0),
            1 => Some(DTOCV_ACK_A::DTOCV_ACK_1),
            2 => Some(DTOCV_ACK_A::DTOCV_ACK_2),
            3 => Some(DTOCV_ACK_A::DTOCV_ACK_3),
            4 => Some(DTOCV_ACK_A::DTOCV_ACK_4),
            5 => Some(DTOCV_ACK_A::DTOCV_ACK_5),
            6 => Some(DTOCV_ACK_A::DTOCV_ACK_6),
            7 => Some(DTOCV_ACK_A::DTOCV_ACK_7),
            14 => Some(DTOCV_ACK_A::DTOCV_ACK_14),
            15 => Some(DTOCV_ACK_A::DTOCV_ACK_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_0`"]
    #[inline(always)]
    pub fn is_dtocv_ack_0(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_0
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_1`"]
    #[inline(always)]
    pub fn is_dtocv_ack_1(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_1
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_2`"]
    #[inline(always)]
    pub fn is_dtocv_ack_2(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_2
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_3`"]
    #[inline(always)]
    pub fn is_dtocv_ack_3(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_3
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_4`"]
    #[inline(always)]
    pub fn is_dtocv_ack_4(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_4
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_5`"]
    #[inline(always)]
    pub fn is_dtocv_ack_5(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_5
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_6`"]
    #[inline(always)]
    pub fn is_dtocv_ack_6(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_6
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_7`"]
    #[inline(always)]
    pub fn is_dtocv_ack_7(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_7
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_14`"]
    #[inline(always)]
    pub fn is_dtocv_ack_14(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_14
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_15`"]
    #[inline(always)]
    pub fn is_dtocv_ack_15(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_15
    }
}
#[doc = "Field `DTOCV_ACK` writer - Boot ACK time out"]
pub type DTOCV_ACK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MMC_BOOT_SPEC, u8, DTOCV_ACK_A, 4, O>;
impl<'a, const O: u8> DTOCV_ACK_W<'a, O> {
    #[doc = "SDCLK x 2^14"]
    #[inline(always)]
    pub fn dtocv_ack_0(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_0)
    }
    #[doc = "SDCLK x 2^15"]
    #[inline(always)]
    pub fn dtocv_ack_1(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_1)
    }
    #[doc = "SDCLK x 2^16"]
    #[inline(always)]
    pub fn dtocv_ack_2(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_2)
    }
    #[doc = "SDCLK x 2^17"]
    #[inline(always)]
    pub fn dtocv_ack_3(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_3)
    }
    #[doc = "SDCLK x 2^18"]
    #[inline(always)]
    pub fn dtocv_ack_4(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_4)
    }
    #[doc = "SDCLK x 2^19"]
    #[inline(always)]
    pub fn dtocv_ack_5(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_5)
    }
    #[doc = "SDCLK x 2^20"]
    #[inline(always)]
    pub fn dtocv_ack_6(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_6)
    }
    #[doc = "SDCLK x 2^21"]
    #[inline(always)]
    pub fn dtocv_ack_7(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_7)
    }
    #[doc = "SDCLK x 2^28"]
    #[inline(always)]
    pub fn dtocv_ack_14(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_14)
    }
    #[doc = "SDCLK x 2^29"]
    #[inline(always)]
    pub fn dtocv_ack_15(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_15)
    }
}
#[doc = "Field `BOOT_ACK` reader - BOOT ACK"]
pub type BOOT_ACK_R = crate::BitReader<BOOT_ACK_A>;
#[doc = "BOOT ACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_ACK_A {
    #[doc = "0: No ack"]
    BOOT_ACK_0 = 0,
    #[doc = "1: Ack"]
    BOOT_ACK_1 = 1,
}
impl From<BOOT_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOT_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_ACK_A {
        match self.bits {
            false => BOOT_ACK_A::BOOT_ACK_0,
            true => BOOT_ACK_A::BOOT_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_ACK_0`"]
    #[inline(always)]
    pub fn is_boot_ack_0(&self) -> bool {
        *self == BOOT_ACK_A::BOOT_ACK_0
    }
    #[doc = "Checks if the value of the field is `BOOT_ACK_1`"]
    #[inline(always)]
    pub fn is_boot_ack_1(&self) -> bool {
        *self == BOOT_ACK_A::BOOT_ACK_1
    }
}
#[doc = "Field `BOOT_ACK` writer - BOOT ACK"]
pub type BOOT_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_BOOT_SPEC, BOOT_ACK_A, O>;
impl<'a, const O: u8> BOOT_ACK_W<'a, O> {
    #[doc = "No ack"]
    #[inline(always)]
    pub fn boot_ack_0(self) -> &'a mut W {
        self.variant(BOOT_ACK_A::BOOT_ACK_0)
    }
    #[doc = "Ack"]
    #[inline(always)]
    pub fn boot_ack_1(self) -> &'a mut W {
        self.variant(BOOT_ACK_A::BOOT_ACK_1)
    }
}
#[doc = "Field `BOOT_MODE` reader - Boot mode"]
pub type BOOT_MODE_R = crate::BitReader<BOOT_MODE_A>;
#[doc = "Boot mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_MODE_A {
    #[doc = "0: Normal boot"]
    BOOT_MODE_0 = 0,
    #[doc = "1: Alternative boot"]
    BOOT_MODE_1 = 1,
}
impl From<BOOT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_MODE_A {
        match self.bits {
            false => BOOT_MODE_A::BOOT_MODE_0,
            true => BOOT_MODE_A::BOOT_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_MODE_0`"]
    #[inline(always)]
    pub fn is_boot_mode_0(&self) -> bool {
        *self == BOOT_MODE_A::BOOT_MODE_0
    }
    #[doc = "Checks if the value of the field is `BOOT_MODE_1`"]
    #[inline(always)]
    pub fn is_boot_mode_1(&self) -> bool {
        *self == BOOT_MODE_A::BOOT_MODE_1
    }
}
#[doc = "Field `BOOT_MODE` writer - Boot mode"]
pub type BOOT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_BOOT_SPEC, BOOT_MODE_A, O>;
impl<'a, const O: u8> BOOT_MODE_W<'a, O> {
    #[doc = "Normal boot"]
    #[inline(always)]
    pub fn boot_mode_0(self) -> &'a mut W {
        self.variant(BOOT_MODE_A::BOOT_MODE_0)
    }
    #[doc = "Alternative boot"]
    #[inline(always)]
    pub fn boot_mode_1(self) -> &'a mut W {
        self.variant(BOOT_MODE_A::BOOT_MODE_1)
    }
}
#[doc = "Field `BOOT_EN` reader - Boot enable"]
pub type BOOT_EN_R = crate::BitReader<BOOT_EN_A>;
#[doc = "Boot enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_EN_A {
    #[doc = "0: Fast boot disable"]
    BOOT_EN_0 = 0,
    #[doc = "1: Fast boot enable"]
    BOOT_EN_1 = 1,
}
impl From<BOOT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_EN_A {
        match self.bits {
            false => BOOT_EN_A::BOOT_EN_0,
            true => BOOT_EN_A::BOOT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_EN_0`"]
    #[inline(always)]
    pub fn is_boot_en_0(&self) -> bool {
        *self == BOOT_EN_A::BOOT_EN_0
    }
    #[doc = "Checks if the value of the field is `BOOT_EN_1`"]
    #[inline(always)]
    pub fn is_boot_en_1(&self) -> bool {
        *self == BOOT_EN_A::BOOT_EN_1
    }
}
#[doc = "Field `BOOT_EN` writer - Boot enable"]
pub type BOOT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_BOOT_SPEC, BOOT_EN_A, O>;
impl<'a, const O: u8> BOOT_EN_W<'a, O> {
    #[doc = "Fast boot disable"]
    #[inline(always)]
    pub fn boot_en_0(self) -> &'a mut W {
        self.variant(BOOT_EN_A::BOOT_EN_0)
    }
    #[doc = "Fast boot enable"]
    #[inline(always)]
    pub fn boot_en_1(self) -> &'a mut W {
        self.variant(BOOT_EN_A::BOOT_EN_1)
    }
}
#[doc = "Field `AUTO_SABG_EN` reader - Auto stop at block gap"]
pub type AUTO_SABG_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_SABG_EN` writer - Auto stop at block gap"]
pub type AUTO_SABG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_BOOT_SPEC, bool, O>;
#[doc = "Field `DISABLE_TIME_OUT` reader - Time out"]
pub type DISABLE_TIME_OUT_R = crate::BitReader<DISABLE_TIME_OUT_A>;
#[doc = "Time out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLE_TIME_OUT_A {
    #[doc = "0: Enable time out"]
    DISABLE_TIME_OUT_0 = 0,
    #[doc = "1: Disable time out"]
    DISABLE_TIME_OUT_1 = 1,
}
impl From<DISABLE_TIME_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_TIME_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLE_TIME_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_TIME_OUT_A {
        match self.bits {
            false => DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_0,
            true => DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_TIME_OUT_0`"]
    #[inline(always)]
    pub fn is_disable_time_out_0(&self) -> bool {
        *self == DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_0
    }
    #[doc = "Checks if the value of the field is `DISABLE_TIME_OUT_1`"]
    #[inline(always)]
    pub fn is_disable_time_out_1(&self) -> bool {
        *self == DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_1
    }
}
#[doc = "Field `DISABLE_TIME_OUT` writer - Time out"]
pub type DISABLE_TIME_OUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_BOOT_SPEC, DISABLE_TIME_OUT_A, O>;
impl<'a, const O: u8> DISABLE_TIME_OUT_W<'a, O> {
    #[doc = "Enable time out"]
    #[inline(always)]
    pub fn disable_time_out_0(self) -> &'a mut W {
        self.variant(DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_0)
    }
    #[doc = "Disable time out"]
    #[inline(always)]
    pub fn disable_time_out_1(self) -> &'a mut W {
        self.variant(DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_1)
    }
}
#[doc = "Field `BOOT_BLK_CNT` reader - Stop At Block Gap value of automatic mode"]
pub type BOOT_BLK_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOOT_BLK_CNT` writer - Stop At Block Gap value of automatic mode"]
pub type BOOT_BLK_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MMC_BOOT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - Boot ACK time out"]
    #[inline(always)]
    pub fn dtocv_ack(&self) -> DTOCV_ACK_R {
        DTOCV_ACK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - BOOT ACK"]
    #[inline(always)]
    pub fn boot_ack(&self) -> BOOT_ACK_R {
        BOOT_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Boot mode"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Boot enable"]
    #[inline(always)]
    pub fn boot_en(&self) -> BOOT_EN_R {
        BOOT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto stop at block gap"]
    #[inline(always)]
    pub fn auto_sabg_en(&self) -> AUTO_SABG_EN_R {
        AUTO_SABG_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time out"]
    #[inline(always)]
    pub fn disable_time_out(&self) -> DISABLE_TIME_OUT_R {
        DISABLE_TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Stop At Block Gap value of automatic mode"]
    #[inline(always)]
    pub fn boot_blk_cnt(&self) -> BOOT_BLK_CNT_R {
        BOOT_BLK_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boot ACK time out"]
    #[inline(always)]
    #[must_use]
    pub fn dtocv_ack(&mut self) -> DTOCV_ACK_W<0> {
        DTOCV_ACK_W::new(self)
    }
    #[doc = "Bit 4 - BOOT ACK"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack(&mut self) -> BOOT_ACK_W<4> {
        BOOT_ACK_W::new(self)
    }
    #[doc = "Bit 5 - Boot mode"]
    #[inline(always)]
    #[must_use]
    pub fn boot_mode(&mut self) -> BOOT_MODE_W<5> {
        BOOT_MODE_W::new(self)
    }
    #[doc = "Bit 6 - Boot enable"]
    #[inline(always)]
    #[must_use]
    pub fn boot_en(&mut self) -> BOOT_EN_W<6> {
        BOOT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Auto stop at block gap"]
    #[inline(always)]
    #[must_use]
    pub fn auto_sabg_en(&mut self) -> AUTO_SABG_EN_W<7> {
        AUTO_SABG_EN_W::new(self)
    }
    #[doc = "Bit 8 - Time out"]
    #[inline(always)]
    #[must_use]
    pub fn disable_time_out(&mut self) -> DISABLE_TIME_OUT_W<8> {
        DISABLE_TIME_OUT_W::new(self)
    }
    #[doc = "Bits 16:31 - Stop At Block Gap value of automatic mode"]
    #[inline(always)]
    #[must_use]
    pub fn boot_blk_cnt(&mut self) -> BOOT_BLK_CNT_W<16> {
        BOOT_BLK_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Boot\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_boot](index.html) module"]
pub struct MMC_BOOT_SPEC;
impl crate::RegisterSpec for MMC_BOOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_boot::R](R) reader structure"]
impl crate::Readable for MMC_BOOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_boot::W](W) writer structure"]
impl crate::Writable for MMC_BOOT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_BOOT to value 0"]
impl crate::Resettable for MMC_BOOT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
