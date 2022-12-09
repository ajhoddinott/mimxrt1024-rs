#[doc = "Register `RX15MASK` reader"]
pub struct R(crate::R<RX15MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX15MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX15MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX15MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX15MASK` writer"]
pub struct W(crate::W<RX15MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX15MASK_SPEC>;
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
impl From<crate::W<RX15MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX15MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX15M` reader - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
pub type RX15M_R = crate::FieldReader<u32, RX15M_A>;
#[doc = "These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RX15M_A {
    #[doc = "0: the corresponding bit in the filter is \"don't care\""]
    RX15M_0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    RX15M_1 = 1,
}
impl From<RX15M_A> for u32 {
    #[inline(always)]
    fn from(variant: RX15M_A) -> Self {
        variant as _
    }
}
impl RX15M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX15M_A> {
        match self.bits {
            0 => Some(RX15M_A::RX15M_0),
            1 => Some(RX15M_A::RX15M_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RX15M_0`"]
    #[inline(always)]
    pub fn is_rx15m_0(&self) -> bool {
        *self == RX15M_A::RX15M_0
    }
    #[doc = "Checks if the value of the field is `RX15M_1`"]
    #[inline(always)]
    pub fn is_rx15m_1(&self) -> bool {
        *self == RX15M_A::RX15M_1
    }
}
#[doc = "Field `RX15M` writer - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
pub type RX15M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX15MASK_SPEC, u32, RX15M_A, 32, O>;
impl<'a, const O: u8> RX15M_W<'a, O> {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn rx15m_0(self) -> &'a mut W {
        self.variant(RX15M_A::RX15M_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn rx15m_1(self) -> &'a mut W {
        self.variant(RX15M_A::RX15M_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline(always)]
    pub fn rx15m(&self) -> RX15M_R {
        RX15M_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline(always)]
    #[must_use]
    pub fn rx15m(&mut self) -> RX15M_W<0> {
        RX15M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Buffer 15 Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx15mask](index.html) module"]
pub struct RX15MASK_SPEC;
impl crate::RegisterSpec for RX15MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx15mask::R](R) reader structure"]
impl crate::Readable for RX15MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx15mask::W](W) writer structure"]
impl crate::Writable for RX15MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX15MASK to value 0xffff_ffff"]
impl crate::Resettable for RX15MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
