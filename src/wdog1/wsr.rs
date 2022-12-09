#[doc = "Register `WSR` reader"]
pub struct R(crate::R<WSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WSR` writer"]
pub struct W(crate::W<WSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WSR_SPEC>;
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
impl From<crate::W<WSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WSR` reader - WSR"]
pub type WSR_R = crate::FieldReader<u16, WSR_A>;
#[doc = "WSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WSR_A {
    #[doc = "21845: Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_21845 = 21845,
    #[doc = "43690: Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_43690 = 43690,
}
impl From<WSR_A> for u16 {
    #[inline(always)]
    fn from(variant: WSR_A) -> Self {
        variant as _
    }
}
impl WSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSR_A> {
        match self.bits {
            21845 => Some(WSR_A::WSR_21845),
            43690 => Some(WSR_A::WSR_43690),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WSR_21845`"]
    #[inline(always)]
    pub fn is_wsr_21845(&self) -> bool {
        *self == WSR_A::WSR_21845
    }
    #[doc = "Checks if the value of the field is `WSR_43690`"]
    #[inline(always)]
    pub fn is_wsr_43690(&self) -> bool {
        *self == WSR_A::WSR_43690
    }
}
#[doc = "Field `WSR` writer - WSR"]
pub type WSR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WSR_SPEC, u16, WSR_A, 16, O>;
impl<'a, const O: u8> WSR_W<'a, O> {
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    #[inline(always)]
    pub fn wsr_21845(self) -> &'a mut W {
        self.variant(WSR_A::WSR_21845)
    }
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    #[inline(always)]
    pub fn wsr_43690(self) -> &'a mut W {
        self.variant(WSR_A::WSR_43690)
    }
}
impl R {
    #[doc = "Bits 0:15 - WSR"]
    #[inline(always)]
    pub fn wsr(&self) -> WSR_R {
        WSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - WSR"]
    #[inline(always)]
    #[must_use]
    pub fn wsr(&mut self) -> WSR_W<0> {
        WSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Service Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsr](index.html) module"]
pub struct WSR_SPEC;
impl crate::RegisterSpec for WSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wsr::R](R) reader structure"]
impl crate::Readable for WSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wsr::W](W) writer structure"]
impl crate::Writable for WSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WSR to value 0"]
impl crate::Resettable for WSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
