#[doc = "Register `HOST_CTRL_CAP` reader"]
pub struct R(crate::R<HOST_CTRL_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CTRL_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CTRL_CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CTRL_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_CTRL_CAP` writer"]
pub struct W(crate::W<HOST_CTRL_CAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_CTRL_CAP_SPEC>;
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
impl From<crate::W<HOST_CTRL_CAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_CTRL_CAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDR50_SUPPORT` reader - SDR50 support"]
pub type SDR50_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `SDR104_SUPPORT` reader - SDR104 support"]
pub type SDR104_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `DDR50_SUPPORT` reader - DDR50 support"]
pub type DDR50_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `USE_TUNING_SDR50` reader - Use Tuning for SDR50"]
pub type USE_TUNING_SDR50_R = crate::BitReader<USE_TUNING_SDR50_A>;
#[doc = "Use Tuning for SDR50\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USE_TUNING_SDR50_A {
    #[doc = "0: SDR50 does not support tuning"]
    USE_TUNING_SDR50_0 = 0,
    #[doc = "1: SDR50 supports tuning"]
    USE_TUNING_SDR50_1 = 1,
}
impl From<USE_TUNING_SDR50_A> for bool {
    #[inline(always)]
    fn from(variant: USE_TUNING_SDR50_A) -> Self {
        variant as u8 != 0
    }
}
impl USE_TUNING_SDR50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USE_TUNING_SDR50_A {
        match self.bits {
            false => USE_TUNING_SDR50_A::USE_TUNING_SDR50_0,
            true => USE_TUNING_SDR50_A::USE_TUNING_SDR50_1,
        }
    }
    #[doc = "Checks if the value of the field is `USE_TUNING_SDR50_0`"]
    #[inline(always)]
    pub fn is_use_tuning_sdr50_0(&self) -> bool {
        *self == USE_TUNING_SDR50_A::USE_TUNING_SDR50_0
    }
    #[doc = "Checks if the value of the field is `USE_TUNING_SDR50_1`"]
    #[inline(always)]
    pub fn is_use_tuning_sdr50_1(&self) -> bool {
        *self == USE_TUNING_SDR50_A::USE_TUNING_SDR50_1
    }
}
#[doc = "Field `USE_TUNING_SDR50` writer - Use Tuning for SDR50"]
pub type USE_TUNING_SDR50_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HOST_CTRL_CAP_SPEC, USE_TUNING_SDR50_A, O>;
impl<'a, const O: u8> USE_TUNING_SDR50_W<'a, O> {
    #[doc = "SDR50 does not support tuning"]
    #[inline(always)]
    pub fn use_tuning_sdr50_0(self) -> &'a mut W {
        self.variant(USE_TUNING_SDR50_A::USE_TUNING_SDR50_0)
    }
    #[doc = "SDR50 supports tuning"]
    #[inline(always)]
    pub fn use_tuning_sdr50_1(self) -> &'a mut W {
        self.variant(USE_TUNING_SDR50_A::USE_TUNING_SDR50_1)
    }
}
#[doc = "Field `MBL` reader - Max block length"]
pub type MBL_R = crate::FieldReader<u8, MBL_A>;
#[doc = "Max block length\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBL_A {
    #[doc = "0: 512 bytes"]
    MBL_0 = 0,
    #[doc = "1: 1024 bytes"]
    MBL_1 = 1,
    #[doc = "2: 2048 bytes"]
    MBL_2 = 2,
    #[doc = "3: 4096 bytes"]
    MBL_3 = 3,
}
impl From<MBL_A> for u8 {
    #[inline(always)]
    fn from(variant: MBL_A) -> Self {
        variant as _
    }
}
impl MBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MBL_A> {
        match self.bits {
            0 => Some(MBL_A::MBL_0),
            1 => Some(MBL_A::MBL_1),
            2 => Some(MBL_A::MBL_2),
            3 => Some(MBL_A::MBL_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MBL_0`"]
    #[inline(always)]
    pub fn is_mbl_0(&self) -> bool {
        *self == MBL_A::MBL_0
    }
    #[doc = "Checks if the value of the field is `MBL_1`"]
    #[inline(always)]
    pub fn is_mbl_1(&self) -> bool {
        *self == MBL_A::MBL_1
    }
    #[doc = "Checks if the value of the field is `MBL_2`"]
    #[inline(always)]
    pub fn is_mbl_2(&self) -> bool {
        *self == MBL_A::MBL_2
    }
    #[doc = "Checks if the value of the field is `MBL_3`"]
    #[inline(always)]
    pub fn is_mbl_3(&self) -> bool {
        *self == MBL_A::MBL_3
    }
}
#[doc = "Field `ADMAS` reader - ADMA support"]
pub type ADMAS_R = crate::BitReader<ADMAS_A>;
#[doc = "ADMA support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMAS_A {
    #[doc = "0: Advanced DMA not supported"]
    ADMAS_0 = 0,
    #[doc = "1: Advanced DMA supported"]
    ADMAS_1 = 1,
}
impl From<ADMAS_A> for bool {
    #[inline(always)]
    fn from(variant: ADMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMAS_A {
        match self.bits {
            false => ADMAS_A::ADMAS_0,
            true => ADMAS_A::ADMAS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMAS_0`"]
    #[inline(always)]
    pub fn is_admas_0(&self) -> bool {
        *self == ADMAS_A::ADMAS_0
    }
    #[doc = "Checks if the value of the field is `ADMAS_1`"]
    #[inline(always)]
    pub fn is_admas_1(&self) -> bool {
        *self == ADMAS_A::ADMAS_1
    }
}
#[doc = "Field `HSS` reader - High speed support"]
pub type HSS_R = crate::BitReader<HSS_A>;
#[doc = "High speed support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSS_A {
    #[doc = "0: High speed not supported"]
    HSS_0 = 0,
    #[doc = "1: High speed supported"]
    HSS_1 = 1,
}
impl From<HSS_A> for bool {
    #[inline(always)]
    fn from(variant: HSS_A) -> Self {
        variant as u8 != 0
    }
}
impl HSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSS_A {
        match self.bits {
            false => HSS_A::HSS_0,
            true => HSS_A::HSS_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSS_0`"]
    #[inline(always)]
    pub fn is_hss_0(&self) -> bool {
        *self == HSS_A::HSS_0
    }
    #[doc = "Checks if the value of the field is `HSS_1`"]
    #[inline(always)]
    pub fn is_hss_1(&self) -> bool {
        *self == HSS_A::HSS_1
    }
}
#[doc = "Field `DMAS` reader - DMA support"]
pub type DMAS_R = crate::BitReader<DMAS_A>;
#[doc = "DMA support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAS_A {
    #[doc = "0: DMA not supported"]
    DMAS_0 = 0,
    #[doc = "1: DMA supported"]
    DMAS_1 = 1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::DMAS_0,
            true => DMAS_A::DMAS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAS_0`"]
    #[inline(always)]
    pub fn is_dmas_0(&self) -> bool {
        *self == DMAS_A::DMAS_0
    }
    #[doc = "Checks if the value of the field is `DMAS_1`"]
    #[inline(always)]
    pub fn is_dmas_1(&self) -> bool {
        *self == DMAS_A::DMAS_1
    }
}
#[doc = "Field `SRS` reader - Suspend / resume support"]
pub type SRS_R = crate::BitReader<SRS_A>;
#[doc = "Suspend / resume support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRS_A {
    #[doc = "0: Not supported"]
    SRS_0 = 0,
    #[doc = "1: Supported"]
    SRS_1 = 1,
}
impl From<SRS_A> for bool {
    #[inline(always)]
    fn from(variant: SRS_A) -> Self {
        variant as u8 != 0
    }
}
impl SRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS_A {
        match self.bits {
            false => SRS_A::SRS_0,
            true => SRS_A::SRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRS_0`"]
    #[inline(always)]
    pub fn is_srs_0(&self) -> bool {
        *self == SRS_A::SRS_0
    }
    #[doc = "Checks if the value of the field is `SRS_1`"]
    #[inline(always)]
    pub fn is_srs_1(&self) -> bool {
        *self == SRS_A::SRS_1
    }
}
#[doc = "Field `VS33` reader - Voltage support 3.3 V"]
pub type VS33_R = crate::BitReader<VS33_A>;
#[doc = "Voltage support 3.3 V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VS33_A {
    #[doc = "0: 3.3 V not supported"]
    VS33_0 = 0,
    #[doc = "1: 3.3 V supported"]
    VS33_1 = 1,
}
impl From<VS33_A> for bool {
    #[inline(always)]
    fn from(variant: VS33_A) -> Self {
        variant as u8 != 0
    }
}
impl VS33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VS33_A {
        match self.bits {
            false => VS33_A::VS33_0,
            true => VS33_A::VS33_1,
        }
    }
    #[doc = "Checks if the value of the field is `VS33_0`"]
    #[inline(always)]
    pub fn is_vs33_0(&self) -> bool {
        *self == VS33_A::VS33_0
    }
    #[doc = "Checks if the value of the field is `VS33_1`"]
    #[inline(always)]
    pub fn is_vs33_1(&self) -> bool {
        *self == VS33_A::VS33_1
    }
}
#[doc = "Field `VS30` reader - Voltage support 3.0 V"]
pub type VS30_R = crate::BitReader<VS30_A>;
#[doc = "Voltage support 3.0 V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VS30_A {
    #[doc = "0: 3.0 V not supported"]
    VS30_0 = 0,
    #[doc = "1: 3.0 V supported"]
    VS30_1 = 1,
}
impl From<VS30_A> for bool {
    #[inline(always)]
    fn from(variant: VS30_A) -> Self {
        variant as u8 != 0
    }
}
impl VS30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VS30_A {
        match self.bits {
            false => VS30_A::VS30_0,
            true => VS30_A::VS30_1,
        }
    }
    #[doc = "Checks if the value of the field is `VS30_0`"]
    #[inline(always)]
    pub fn is_vs30_0(&self) -> bool {
        *self == VS30_A::VS30_0
    }
    #[doc = "Checks if the value of the field is `VS30_1`"]
    #[inline(always)]
    pub fn is_vs30_1(&self) -> bool {
        *self == VS30_A::VS30_1
    }
}
#[doc = "Field `VS18` reader - Voltage support 1.8 V"]
pub type VS18_R = crate::BitReader<VS18_A>;
#[doc = "Voltage support 1.8 V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VS18_A {
    #[doc = "0: 1.8 V not supported"]
    VS18_0 = 0,
    #[doc = "1: 1.8 V supported"]
    VS18_1 = 1,
}
impl From<VS18_A> for bool {
    #[inline(always)]
    fn from(variant: VS18_A) -> Self {
        variant as u8 != 0
    }
}
impl VS18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VS18_A {
        match self.bits {
            false => VS18_A::VS18_0,
            true => VS18_A::VS18_1,
        }
    }
    #[doc = "Checks if the value of the field is `VS18_0`"]
    #[inline(always)]
    pub fn is_vs18_0(&self) -> bool {
        *self == VS18_A::VS18_0
    }
    #[doc = "Checks if the value of the field is `VS18_1`"]
    #[inline(always)]
    pub fn is_vs18_1(&self) -> bool {
        *self == VS18_A::VS18_1
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 support"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> SDR50_SUPPORT_R {
        SDR50_SUPPORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 support"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> SDR104_SUPPORT_R {
        SDR104_SUPPORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 support"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> DDR50_SUPPORT_R {
        DDR50_SUPPORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> USE_TUNING_SDR50_R {
        USE_TUNING_SDR50_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Max block length"]
    #[inline(always)]
    pub fn mbl(&self) -> MBL_R {
        MBL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - ADMA support"]
    #[inline(always)]
    pub fn admas(&self) -> ADMAS_R {
        ADMAS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - High speed support"]
    #[inline(always)]
    pub fn hss(&self) -> HSS_R {
        HSS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA support"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend / resume support"]
    #[inline(always)]
    pub fn srs(&self) -> SRS_R {
        SRS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage support 3.3 V"]
    #[inline(always)]
    pub fn vs33(&self) -> VS33_R {
        VS33_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage support 3.0 V"]
    #[inline(always)]
    pub fn vs30(&self) -> VS30_R {
        VS30_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage support 1.8 V"]
    #[inline(always)]
    pub fn vs18(&self) -> VS18_R {
        VS18_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    #[must_use]
    pub fn use_tuning_sdr50(&mut self) -> USE_TUNING_SDR50_W<13> {
        USE_TUNING_SDR50_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Capabilities\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctrl_cap](index.html) module"]
pub struct HOST_CTRL_CAP_SPEC;
impl crate::RegisterSpec for HOST_CTRL_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ctrl_cap::R](R) reader structure"]
impl crate::Readable for HOST_CTRL_CAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ctrl_cap::W](W) writer structure"]
impl crate::Writable for HOST_CTRL_CAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_CTRL_CAP to value 0x07f3_b407"]
impl crate::Resettable for HOST_CTRL_CAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x07f3_b407;
}
