#[doc = "Register `IPCR1` reader"]
pub struct R(crate::R<IPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCR1` writer"]
pub struct W(crate::W<IPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCR1_SPEC>;
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
impl From<crate::W<IPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDATSZ` reader - Flash Read/Program Data Size (in Bytes) for IP command."]
pub type IDATSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IDATSZ` writer - Flash Read/Program Data Size (in Bytes) for IP command."]
pub type IDATSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `ISEQID` reader - Sequence Index in LUT for IP command."]
pub type ISEQID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISEQID` writer - Sequence Index in LUT for IP command."]
pub type ISEQID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ISEQNUM` reader - Sequence Number for IP command: ISEQNUM+1."]
pub type ISEQNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISEQNUM` writer - Sequence Number for IP command: ISEQNUM+1."]
pub type ISEQNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `IPAREN` reader - Parallel mode Enabled for IP command."]
pub type IPAREN_R = crate::BitReader<IPAREN_A>;
#[doc = "Parallel mode Enabled for IP command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPAREN_A {
    #[doc = "0: Flash will be accessed in Individual mode."]
    DISABLE = 0,
    #[doc = "1: Flash will be accessed in Parallel mode."]
    ENABLE = 1,
}
impl From<IPAREN_A> for bool {
    #[inline(always)]
    fn from(variant: IPAREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPAREN_A {
        match self.bits {
            false => IPAREN_A::DISABLE,
            true => IPAREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IPAREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IPAREN_A::ENABLE
    }
}
#[doc = "Field `IPAREN` writer - Parallel mode Enabled for IP command."]
pub type IPAREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCR1_SPEC, IPAREN_A, O>;
impl<'a, const O: u8> IPAREN_W<'a, O> {
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IPAREN_A::DISABLE)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IPAREN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    pub fn idatsz(&self) -> IDATSZ_R {
        IDATSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub fn iseqid(&self) -> ISEQID_R {
        ISEQID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub fn iseqnum(&self) -> ISEQNUM_R {
        ISEQNUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline(always)]
    pub fn iparen(&self) -> IPAREN_R {
        IPAREN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn idatsz(&mut self) -> IDATSZ_W<0> {
        IDATSZ_W::new(self)
    }
    #[doc = "Bits 16:19 - Sequence Index in LUT for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn iseqid(&mut self) -> ISEQID_W<16> {
        ISEQID_W::new(self)
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    #[must_use]
    pub fn iseqnum(&mut self) -> ISEQNUM_W<24> {
        ISEQNUM_W::new(self)
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn iparen(&mut self) -> IPAREN_W<31> {
        IPAREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr1](index.html) module"]
pub struct IPCR1_SPEC;
impl crate::RegisterSpec for IPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcr1::R](R) reader structure"]
impl crate::Readable for IPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcr1::W](W) writer structure"]
impl crate::Writable for IPCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCR1 to value 0"]
impl crate::Resettable for IPCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
