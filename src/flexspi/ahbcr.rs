#[doc = "Register `AHBCR` reader"]
pub struct R(crate::R<AHBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCR` writer"]
pub struct W(crate::W<AHBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCR_SPEC>;
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
impl From<crate::W<AHBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APAREN` reader - Parallel mode enabled for AHB triggered Command (both read and write) ."]
pub type APAREN_R = crate::BitReader<APAREN_A>;
#[doc = "Parallel mode enabled for AHB triggered Command (both read and write) .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APAREN_A {
    #[doc = "0: Flash will be accessed in Individual mode."]
    INDIVIDUAL = 0,
    #[doc = "1: Flash will be accessed in Parallel mode."]
    ENABLE = 1,
}
impl From<APAREN_A> for bool {
    #[inline(always)]
    fn from(variant: APAREN_A) -> Self {
        variant as u8 != 0
    }
}
impl APAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APAREN_A {
        match self.bits {
            false => APAREN_A::INDIVIDUAL,
            true => APAREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL`"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        *self == APAREN_A::INDIVIDUAL
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == APAREN_A::ENABLE
    }
}
#[doc = "Field `APAREN` writer - Parallel mode enabled for AHB triggered Command (both read and write) ."]
pub type APAREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, APAREN_A, O>;
impl<'a, const O: u8> APAREN_W<'a, O> {
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn individual(self) -> &'a mut W {
        self.variant(APAREN_A::INDIVIDUAL)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(APAREN_A::ENABLE)
    }
}
#[doc = "Field `CLRAHBRXBUF` reader - Clear the status/pointers of AHB RX Buffer. Auto-cleared."]
pub type CLRAHBRXBUF_R = crate::BitReader<bool>;
#[doc = "Field `CLRAHBRXBUF` writer - Clear the status/pointers of AHB RX Buffer. Auto-cleared."]
pub type CLRAHBRXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, bool, O>;
#[doc = "Field `CLRAHBTXBUF` reader - Clear the status/pointers of AHB TX Buffer. Auto-cleared. For internal use only."]
pub type CLRAHBTXBUF_R = crate::BitReader<bool>;
#[doc = "Field `CLRAHBTXBUF` writer - Clear the status/pointers of AHB TX Buffer. Auto-cleared. For internal use only."]
pub type CLRAHBTXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, bool, O>;
#[doc = "Field `CACHABLEEN` reader - Enable AHB bus cachable read access support."]
pub type CACHABLEEN_R = crate::BitReader<CACHABLEEN_A>;
#[doc = "Enable AHB bus cachable read access support.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHABLEEN_A {
    #[doc = "0: Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    CACHABLEEN_0 = 0,
    #[doc = "1: Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    CACHABLEEN_1 = 1,
}
impl From<CACHABLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHABLEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHABLEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHABLEEN_A {
        match self.bits {
            false => CACHABLEEN_A::CACHABLEEN_0,
            true => CACHABLEEN_A::CACHABLEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHABLEEN_0`"]
    #[inline(always)]
    pub fn is_cachableen_0(&self) -> bool {
        *self == CACHABLEEN_A::CACHABLEEN_0
    }
    #[doc = "Checks if the value of the field is `CACHABLEEN_1`"]
    #[inline(always)]
    pub fn is_cachableen_1(&self) -> bool {
        *self == CACHABLEEN_A::CACHABLEEN_1
    }
}
#[doc = "Field `CACHABLEEN` writer - Enable AHB bus cachable read access support."]
pub type CACHABLEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, CACHABLEEN_A, O>;
impl<'a, const O: u8> CACHABLEEN_W<'a, O> {
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    #[inline(always)]
    pub fn cachableen_0(self) -> &'a mut W {
        self.variant(CACHABLEEN_A::CACHABLEEN_0)
    }
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    #[inline(always)]
    pub fn cachableen_1(self) -> &'a mut W {
        self.variant(CACHABLEEN_A::CACHABLEEN_1)
    }
}
#[doc = "Field `BUFFERABLEEN` reader - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
pub type BUFFERABLEEN_R = crate::BitReader<BUFFERABLEEN_A>;
#[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFFERABLEEN_A {
    #[doc = "0: Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    BUFFERABLEEN_0 = 0,
    #[doc = "1: Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    BUFFERABLEEN_1 = 1,
}
impl From<BUFFERABLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFERABLEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFFERABLEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFERABLEEN_A {
        match self.bits {
            false => BUFFERABLEEN_A::BUFFERABLEEN_0,
            true => BUFFERABLEEN_A::BUFFERABLEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUFFERABLEEN_0`"]
    #[inline(always)]
    pub fn is_bufferableen_0(&self) -> bool {
        *self == BUFFERABLEEN_A::BUFFERABLEEN_0
    }
    #[doc = "Checks if the value of the field is `BUFFERABLEEN_1`"]
    #[inline(always)]
    pub fn is_bufferableen_1(&self) -> bool {
        *self == BUFFERABLEEN_A::BUFFERABLEEN_1
    }
}
#[doc = "Field `BUFFERABLEEN` writer - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
pub type BUFFERABLEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, BUFFERABLEEN_A, O>;
impl<'a, const O: u8> BUFFERABLEEN_W<'a, O> {
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    #[inline(always)]
    pub fn bufferableen_0(self) -> &'a mut W {
        self.variant(BUFFERABLEEN_A::BUFFERABLEEN_0)
    }
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    #[inline(always)]
    pub fn bufferableen_1(self) -> &'a mut W {
        self.variant(BUFFERABLEEN_A::BUFFERABLEEN_1)
    }
}
#[doc = "Field `PREFETCHEN` reader - AHB Read Prefetch Enable."]
pub type PREFETCHEN_R = crate::BitReader<bool>;
#[doc = "Field `PREFETCHEN` writer - AHB Read Prefetch Enable."]
pub type PREFETCHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, bool, O>;
#[doc = "Field `READADDROPT` reader - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
pub type READADDROPT_R = crate::BitReader<READADDROPT_A>;
#[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READADDROPT_A {
    #[doc = "0: There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is word-addressable."]
    READADDROPT_0 = 0,
    #[doc = "1: There is no AHB read burst start address alignment limitation. FlexSPI will fetch more data than AHB burst required to meet the alignment requirement."]
    READADDROPT_1 = 1,
}
impl From<READADDROPT_A> for bool {
    #[inline(always)]
    fn from(variant: READADDROPT_A) -> Self {
        variant as u8 != 0
    }
}
impl READADDROPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READADDROPT_A {
        match self.bits {
            false => READADDROPT_A::READADDROPT_0,
            true => READADDROPT_A::READADDROPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `READADDROPT_0`"]
    #[inline(always)]
    pub fn is_readaddropt_0(&self) -> bool {
        *self == READADDROPT_A::READADDROPT_0
    }
    #[doc = "Checks if the value of the field is `READADDROPT_1`"]
    #[inline(always)]
    pub fn is_readaddropt_1(&self) -> bool {
        *self == READADDROPT_A::READADDROPT_1
    }
}
#[doc = "Field `READADDROPT` writer - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
pub type READADDROPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, READADDROPT_A, O>;
impl<'a, const O: u8> READADDROPT_W<'a, O> {
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is word-addressable."]
    #[inline(always)]
    pub fn readaddropt_0(self) -> &'a mut W {
        self.variant(READADDROPT_A::READADDROPT_0)
    }
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more data than AHB burst required to meet the alignment requirement."]
    #[inline(always)]
    pub fn readaddropt_1(self) -> &'a mut W {
        self.variant(READADDROPT_A::READADDROPT_1)
    }
}
#[doc = "Field `READSZALIGN` reader - AHB Read Size Alignment"]
pub type READSZALIGN_R = crate::BitReader<READSZALIGN_A>;
#[doc = "AHB Read Size Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READSZALIGN_A {
    #[doc = "0: AHB read size will be decided by other register setting like PREFETCH_EN"]
    READSZALIGN_0 = 0,
    #[doc = "1: AHB read size to up size to 8 bytes aligned, no prefetching"]
    READSZALIGN_1 = 1,
}
impl From<READSZALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: READSZALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl READSZALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READSZALIGN_A {
        match self.bits {
            false => READSZALIGN_A::READSZALIGN_0,
            true => READSZALIGN_A::READSZALIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `READSZALIGN_0`"]
    #[inline(always)]
    pub fn is_readszalign_0(&self) -> bool {
        *self == READSZALIGN_A::READSZALIGN_0
    }
    #[doc = "Checks if the value of the field is `READSZALIGN_1`"]
    #[inline(always)]
    pub fn is_readszalign_1(&self) -> bool {
        *self == READSZALIGN_A::READSZALIGN_1
    }
}
#[doc = "Field `READSZALIGN` writer - AHB Read Size Alignment"]
pub type READSZALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCR_SPEC, READSZALIGN_A, O>;
impl<'a, const O: u8> READSZALIGN_W<'a, O> {
    #[doc = "AHB read size will be decided by other register setting like PREFETCH_EN"]
    #[inline(always)]
    pub fn readszalign_0(self) -> &'a mut W {
        self.variant(READSZALIGN_A::READSZALIGN_0)
    }
    #[doc = "AHB read size to up size to 8 bytes aligned, no prefetching"]
    #[inline(always)]
    pub fn readszalign_1(self) -> &'a mut W {
        self.variant(READSZALIGN_A::READSZALIGN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub fn aparen(&self) -> APAREN_R {
        APAREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear the status/pointers of AHB RX Buffer. Auto-cleared."]
    #[inline(always)]
    pub fn clrahbrxbuf(&self) -> CLRAHBRXBUF_R {
        CLRAHBRXBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the status/pointers of AHB TX Buffer. Auto-cleared. For internal use only."]
    #[inline(always)]
    pub fn clrahbtxbuf(&self) -> CLRAHBTXBUF_R {
        CLRAHBTXBUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub fn cachableen(&self) -> CACHABLEEN_R {
        CACHABLEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub fn bufferableen(&self) -> BUFFERABLEEN_R {
        BUFFERABLEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline(always)]
    pub fn prefetchen(&self) -> PREFETCHEN_R {
        PREFETCHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub fn readaddropt(&self) -> READADDROPT_R {
        READADDROPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - AHB Read Size Alignment"]
    #[inline(always)]
    pub fn readszalign(&self) -> READSZALIGN_R {
        READSZALIGN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    #[must_use]
    pub fn aparen(&mut self) -> APAREN_W<0> {
        APAREN_W::new(self)
    }
    #[doc = "Bit 1 - Clear the status/pointers of AHB RX Buffer. Auto-cleared."]
    #[inline(always)]
    #[must_use]
    pub fn clrahbrxbuf(&mut self) -> CLRAHBRXBUF_W<1> {
        CLRAHBRXBUF_W::new(self)
    }
    #[doc = "Bit 2 - Clear the status/pointers of AHB TX Buffer. Auto-cleared. For internal use only."]
    #[inline(always)]
    #[must_use]
    pub fn clrahbtxbuf(&mut self) -> CLRAHBTXBUF_W<2> {
        CLRAHBTXBUF_W::new(self)
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline(always)]
    #[must_use]
    pub fn cachableen(&mut self) -> CACHABLEEN_W<3> {
        CACHABLEEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    #[must_use]
    pub fn bufferableen(&mut self) -> BUFFERABLEEN_W<4> {
        BUFFERABLEEN_W::new(self)
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline(always)]
    #[must_use]
    pub fn prefetchen(&mut self) -> PREFETCHEN_W<5> {
        PREFETCHEN_W::new(self)
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    #[must_use]
    pub fn readaddropt(&mut self) -> READADDROPT_W<6> {
        READADDROPT_W::new(self)
    }
    #[doc = "Bit 10 - AHB Read Size Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn readszalign(&mut self) -> READSZALIGN_W<10> {
        READSZALIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbcr](index.html) module"]
pub struct AHBCR_SPEC;
impl crate::RegisterSpec for AHBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbcr::R](R) reader structure"]
impl crate::Readable for AHBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbcr::W](W) writer structure"]
impl crate::Writable for AHBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBCR to value 0x18"]
impl crate::Resettable for AHBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
