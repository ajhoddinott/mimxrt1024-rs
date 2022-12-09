#[doc = "Register `RXMGMASK` reader"]
pub struct R(crate::R<RXMGMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMGMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMGMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMGMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMGMASK` writer"]
pub struct W(crate::W<RXMGMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMGMASK_SPEC>;
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
impl From<crate::W<RXMGMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMGMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MG` reader - These bits mask the Mailbox filter bits as shown in the figure above"]
pub type MG_R = crate::FieldReader<u32, MG_A>;
#[doc = "These bits mask the Mailbox filter bits as shown in the figure above\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum MG_A {
    #[doc = "0: the corresponding bit in the filter is \"don't care\""]
    MG_0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked against the one received"]
    MG_1 = 1,
}
impl From<MG_A> for u32 {
    #[inline(always)]
    fn from(variant: MG_A) -> Self {
        variant as _
    }
}
impl MG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MG_A> {
        match self.bits {
            0 => Some(MG_A::MG_0),
            1 => Some(MG_A::MG_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MG_0`"]
    #[inline(always)]
    pub fn is_mg_0(&self) -> bool {
        *self == MG_A::MG_0
    }
    #[doc = "Checks if the value of the field is `MG_1`"]
    #[inline(always)]
    pub fn is_mg_1(&self) -> bool {
        *self == MG_A::MG_1
    }
}
#[doc = "Field `MG` writer - These bits mask the Mailbox filter bits as shown in the figure above"]
pub type MG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXMGMASK_SPEC, u32, MG_A, 32, O>;
impl<'a, const O: u8> MG_W<'a, O> {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn mg_0(self) -> &'a mut W {
        self.variant(MG_A::MG_0)
    }
    #[doc = "The corresponding bit in the filter is checked against the one received"]
    #[inline(always)]
    pub fn mg_1(self) -> &'a mut W {
        self.variant(MG_A::MG_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - These bits mask the Mailbox filter bits as shown in the figure above"]
    #[inline(always)]
    pub fn mg(&self) -> MG_R {
        MG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits mask the Mailbox filter bits as shown in the figure above"]
    #[inline(always)]
    #[must_use]
    pub fn mg(&mut self) -> MG_W<0> {
        MG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Mailboxes Global Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmgmask](index.html) module"]
pub struct RXMGMASK_SPEC;
impl crate::RegisterSpec for RXMGMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmgmask::R](R) reader structure"]
impl crate::Readable for RXMGMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmgmask::W](W) writer structure"]
impl crate::Writable for RXMGMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXMGMASK to value 0xffff_ffff"]
impl crate::Resettable for RXMGMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
