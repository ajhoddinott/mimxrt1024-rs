#[doc = "Register `SRCD` reader"]
pub struct R(crate::R<SRCD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCD` writer"]
pub struct W(crate::W<SRCD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCD_SPEC>;
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
impl From<crate::W<SRCD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USyncMode` reader - no description available"]
pub type USYNC_MODE_R = crate::BitReader<USYNC_MODE_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USYNC_MODE_A {
    #[doc = "0: Non-CD data"]
    USYNC_MODE_0 = 0,
    #[doc = "1: CD user channel subcode"]
    USYNC_MODE_1 = 1,
}
impl From<USYNC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: USYNC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl USYNC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USYNC_MODE_A {
        match self.bits {
            false => USYNC_MODE_A::USYNC_MODE_0,
            true => USYNC_MODE_A::USYNC_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `USYNC_MODE_0`"]
    #[inline(always)]
    pub fn is_usync_mode_0(&self) -> bool {
        *self == USYNC_MODE_A::USYNC_MODE_0
    }
    #[doc = "Checks if the value of the field is `USYNC_MODE_1`"]
    #[inline(always)]
    pub fn is_usync_mode_1(&self) -> bool {
        *self == USYNC_MODE_A::USYNC_MODE_1
    }
}
#[doc = "Field `USyncMode` writer - no description available"]
pub type USYNC_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCD_SPEC, USYNC_MODE_A, O>;
impl<'a, const O: u8> USYNC_MODE_W<'a, O> {
    #[doc = "Non-CD data"]
    #[inline(always)]
    pub fn usync_mode_0(self) -> &'a mut W {
        self.variant(USYNC_MODE_A::USYNC_MODE_0)
    }
    #[doc = "CD user channel subcode"]
    #[inline(always)]
    pub fn usync_mode_1(self) -> &'a mut W {
        self.variant(USYNC_MODE_A::USYNC_MODE_1)
    }
}
impl R {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn usync_mode(&self) -> USYNC_MODE_R {
        USYNC_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn usync_mode(&mut self) -> USYNC_MODE_W<1> {
        USYNC_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CDText Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcd](index.html) module"]
pub struct SRCD_SPEC;
impl crate::RegisterSpec for SRCD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcd::R](R) reader structure"]
impl crate::Readable for SRCD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcd::W](W) writer structure"]
impl crate::Writable for SRCD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCD to value 0"]
impl crate::Resettable for SRCD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
