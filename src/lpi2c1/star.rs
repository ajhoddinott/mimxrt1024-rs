#[doc = "Register `STAR` reader"]
pub struct R(crate::R<STAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAR` writer"]
pub struct W(crate::W<STAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAR_SPEC>;
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
impl From<crate::W<STAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXNACK` reader - Transmit NACK"]
pub type TXNACK_R = crate::BitReader<TXNACK_A>;
#[doc = "Transmit NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXNACK_A {
    #[doc = "0: Write a Transmit ACK for each received word"]
    TRANSMIT_ACK = 0,
    #[doc = "1: Write a Transmit NACK for each received word"]
    TRANSMIT_NACK = 1,
}
impl From<TXNACK_A> for bool {
    #[inline(always)]
    fn from(variant: TXNACK_A) -> Self {
        variant as u8 != 0
    }
}
impl TXNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXNACK_A {
        match self.bits {
            false => TXNACK_A::TRANSMIT_ACK,
            true => TXNACK_A::TRANSMIT_NACK,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_ACK`"]
    #[inline(always)]
    pub fn is_transmit_ack(&self) -> bool {
        *self == TXNACK_A::TRANSMIT_ACK
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_NACK`"]
    #[inline(always)]
    pub fn is_transmit_nack(&self) -> bool {
        *self == TXNACK_A::TRANSMIT_NACK
    }
}
#[doc = "Field `TXNACK` writer - Transmit NACK"]
pub type TXNACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAR_SPEC, TXNACK_A, O>;
impl<'a, const O: u8> TXNACK_W<'a, O> {
    #[doc = "Write a Transmit ACK for each received word"]
    #[inline(always)]
    pub fn transmit_ack(self) -> &'a mut W {
        self.variant(TXNACK_A::TRANSMIT_ACK)
    }
    #[doc = "Write a Transmit NACK for each received word"]
    #[inline(always)]
    pub fn transmit_nack(self) -> &'a mut W {
        self.variant(TXNACK_A::TRANSMIT_NACK)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline(always)]
    pub fn txnack(&self) -> TXNACK_R {
        TXNACK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline(always)]
    #[must_use]
    pub fn txnack(&mut self) -> TXNACK_W<0> {
        TXNACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Transmit ACK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [star](index.html) module"]
pub struct STAR_SPEC;
impl crate::RegisterSpec for STAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [star::R](R) reader structure"]
impl crate::Readable for STAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [star::W](W) writer structure"]
impl crate::Writable for STAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAR to value 0"]
impl crate::Resettable for STAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
