#[doc = "Register `TCD14_CSR` reader"]
pub struct R(crate::R<TCD14_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD14_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD14_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD14_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD14_CSR` writer"]
pub struct W(crate::W<TCD14_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD14_CSR_SPEC>;
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
impl From<crate::W<TCD14_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD14_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Channel Start"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Channel Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: Channel is not explicitly started"]
    NO_START = 0,
    #[doc = "1: Channel is explicitly started via a software initiated service request"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NO_START,
            true => START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_START`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NO_START
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Channel Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD14_CSR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Channel is not explicitly started"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NO_START)
    }
    #[doc = "Channel is explicitly started via a software initiated service request"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
}
#[doc = "Field `INTMAJOR` reader - Enable an interrupt when major iteration count completes."]
pub type INTMAJOR_R = crate::BitReader<INTMAJOR_A>;
#[doc = "Enable an interrupt when major iteration count completes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTMAJOR_A {
    #[doc = "0: End of major loop interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: End of major loop interrupt is enabled"]
    ENABLED = 1,
}
impl From<INTMAJOR_A> for bool {
    #[inline(always)]
    fn from(variant: INTMAJOR_A) -> Self {
        variant as u8 != 0
    }
}
impl INTMAJOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMAJOR_A {
        match self.bits {
            false => INTMAJOR_A::DISABLED,
            true => INTMAJOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTMAJOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTMAJOR_A::ENABLED
    }
}
#[doc = "Field `INTMAJOR` writer - Enable an interrupt when major iteration count completes."]
pub type INTMAJOR_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD14_CSR_SPEC, INTMAJOR_A, O>;
impl<'a, const O: u8> INTMAJOR_W<'a, O> {
    #[doc = "End of major loop interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTMAJOR_A::DISABLED)
    }
    #[doc = "End of major loop interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTMAJOR_A::ENABLED)
    }
}
#[doc = "Field `INTHALF` reader - Enable an interrupt when major counter is half complete."]
pub type INTHALF_R = crate::BitReader<INTHALF_A>;
#[doc = "Enable an interrupt when major counter is half complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTHALF_A {
    #[doc = "0: Half-point interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Half-point interrupt is enabled"]
    ENABLED = 1,
}
impl From<INTHALF_A> for bool {
    #[inline(always)]
    fn from(variant: INTHALF_A) -> Self {
        variant as u8 != 0
    }
}
impl INTHALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTHALF_A {
        match self.bits {
            false => INTHALF_A::DISABLED,
            true => INTHALF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTHALF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTHALF_A::ENABLED
    }
}
#[doc = "Field `INTHALF` writer - Enable an interrupt when major counter is half complete."]
pub type INTHALF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD14_CSR_SPEC, INTHALF_A, O>;
impl<'a, const O: u8> INTHALF_W<'a, O> {
    #[doc = "Half-point interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTHALF_A::DISABLED)
    }
    #[doc = "Half-point interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTHALF_A::ENABLED)
    }
}
#[doc = "Field `DREQ` reader - Disable Request"]
pub type DREQ_R = crate::BitReader<DREQ_A>;
#[doc = "Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DREQ_A {
    #[doc = "0: The channel's ERQ field is not affected"]
    NO_CLEAR = 0,
    #[doc = "1: The channel's ERQ field value changes to 0 when the major loop is complete"]
    CLEAR = 1,
}
impl From<DREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DREQ_A {
        match self.bits {
            false => DREQ_A::NO_CLEAR,
            true => DREQ_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == DREQ_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DREQ_A::CLEAR
    }
}
#[doc = "Field `DREQ` writer - Disable Request"]
pub type DREQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD14_CSR_SPEC, DREQ_A, O>;
impl<'a, const O: u8> DREQ_W<'a, O> {
    #[doc = "The channel's ERQ field is not affected"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(DREQ_A::NO_CLEAR)
    }
    #[doc = "The channel's ERQ field value changes to 0 when the major loop is complete"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DREQ_A::CLEAR)
    }
}
#[doc = "Field `ESG` reader - Enable Scatter/Gather Processing"]
pub type ESG_R = crate::BitReader<ESG_A>;
#[doc = "Enable Scatter/Gather Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESG_A {
    #[doc = "0: The current channel's TCD is normal format"]
    NORMAL = 0,
    #[doc = "1: The current channel's TCD specifies a scatter gather format"]
    SCATTER = 1,
}
impl From<ESG_A> for bool {
    #[inline(always)]
    fn from(variant: ESG_A) -> Self {
        variant as u8 != 0
    }
}
impl ESG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESG_A {
        match self.bits {
            false => ESG_A::NORMAL,
            true => ESG_A::SCATTER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ESG_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SCATTER`"]
    #[inline(always)]
    pub fn is_scatter(&self) -> bool {
        *self == ESG_A::SCATTER
    }
}
#[doc = "Field `ESG` writer - Enable Scatter/Gather Processing"]
pub type ESG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD14_CSR_SPEC, ESG_A, O>;
impl<'a, const O: u8> ESG_W<'a, O> {
    #[doc = "The current channel's TCD is normal format"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ESG_A::NORMAL)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    #[inline(always)]
    pub fn scatter(self) -> &'a mut W {
        self.variant(ESG_A::SCATTER)
    }
}
#[doc = "Field `MAJORELINK` reader - Enable channel-to-channel linking on major loop complete"]
pub type MAJORELINK_R = crate::BitReader<MAJORELINK_A>;
#[doc = "Enable channel-to-channel linking on major loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAJORELINK_A {
    #[doc = "0: Channel-to-channel linking is disabled"]
    DISABLED = 0,
    #[doc = "1: Channel-to-channel linking is enabled"]
    ENABLED = 1,
}
impl From<MAJORELINK_A> for bool {
    #[inline(always)]
    fn from(variant: MAJORELINK_A) -> Self {
        variant as u8 != 0
    }
}
impl MAJORELINK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAJORELINK_A {
        match self.bits {
            false => MAJORELINK_A::DISABLED,
            true => MAJORELINK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MAJORELINK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MAJORELINK_A::ENABLED
    }
}
#[doc = "Field `MAJORELINK` writer - Enable channel-to-channel linking on major loop complete"]
pub type MAJORELINK_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD14_CSR_SPEC, MAJORELINK_A, O>;
impl<'a, const O: u8> MAJORELINK_W<'a, O> {
    #[doc = "Channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MAJORELINK_A::DISABLED)
    }
    #[doc = "Channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MAJORELINK_A::ENABLED)
    }
}
#[doc = "Field `ACTIVE` reader - Channel Active"]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` reader - Channel Done"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Channel Done"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD14_CSR_SPEC, bool, O>;
#[doc = "Field `MAJORLINKCH` reader - Major Loop Link Channel Number"]
pub type MAJORLINKCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJORLINKCH` writer - Major Loop Link Channel Number"]
pub type MAJORLINKCH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD14_CSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BWC` reader - Bandwidth Control"]
pub type BWC_R = crate::FieldReader<u8, BWC_A>;
#[doc = "Bandwidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BWC_A {
    #[doc = "0: No eDMA engine stalls"]
    DISABLED = 0,
    #[doc = "2: eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 2,
    #[doc = "3: eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 3,
}
impl From<BWC_A> for u8 {
    #[inline(always)]
    fn from(variant: BWC_A) -> Self {
        variant as _
    }
}
impl BWC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BWC_A> {
        match self.bits {
            0 => Some(BWC_A::DISABLED),
            2 => Some(BWC_A::STALL4),
            3 => Some(BWC_A::STALL8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BWC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STALL4`"]
    #[inline(always)]
    pub fn is_stall4(&self) -> bool {
        *self == BWC_A::STALL4
    }
    #[doc = "Checks if the value of the field is `STALL8`"]
    #[inline(always)]
    pub fn is_stall8(&self) -> bool {
        *self == BWC_A::STALL8
    }
}
#[doc = "Field `BWC` writer - Bandwidth Control"]
pub type BWC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD14_CSR_SPEC, u8, BWC_A, 2, O>;
impl<'a, const O: u8> BWC_W<'a, O> {
    #[doc = "No eDMA engine stalls"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BWC_A::DISABLED)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    #[inline(always)]
    pub fn stall4(self) -> &'a mut W {
        self.variant(BWC_A::STALL4)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    #[inline(always)]
    pub fn stall8(self) -> &'a mut W {
        self.variant(BWC_A::STALL8)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub fn intmajor(&self) -> INTMAJOR_R {
        INTMAJOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&self) -> INTHALF_R {
        INTHALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&self) -> ESG_R {
        ESG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&self) -> MAJORELINK_R {
        MAJORELINK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Major Loop Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&self) -> MAJORLINKCH_R {
        MAJORLINKCH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&self) -> BWC_R {
        BWC_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    #[must_use]
    pub fn intmajor(&mut self) -> INTMAJOR_W<1> {
        INTMAJOR_W::new(self)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    #[must_use]
    pub fn inthalf(&mut self) -> INTHALF_W<2> {
        INTHALF_W::new(self)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<3> {
        DREQ_W::new(self)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    #[must_use]
    pub fn esg(&mut self) -> ESG_W<4> {
        ESG_W::new(self)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    #[must_use]
    pub fn majorelink(&mut self) -> MAJORELINK_W<5> {
        MAJORELINK_W::new(self)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<7> {
        DONE_W::new(self)
    }
    #[doc = "Bits 8:12 - Major Loop Link Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn majorlinkch(&mut self) -> MAJORLINKCH_W<8> {
        MAJORLINKCH_W::new(self)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    #[must_use]
    pub fn bwc(&mut self) -> BWC_W<14> {
        BWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_csr](index.html) module"]
pub struct TCD14_CSR_SPEC;
impl crate::RegisterSpec for TCD14_CSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd14_csr::R](R) reader structure"]
impl crate::Readable for TCD14_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd14_csr::W](W) writer structure"]
impl crate::Writable for TCD14_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD14_CSR to value 0"]
impl crate::Resettable for TCD14_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
