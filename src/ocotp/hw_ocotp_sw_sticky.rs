#[doc = "Register `HW_OCOTP_SW_STICKY` reader"]
pub struct R(crate::R<HW_OCOTP_SW_STICKY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_SW_STICKY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_SW_STICKY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_SW_STICKY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_SW_STICKY` writer"]
pub struct W(crate::W<HW_OCOTP_SW_STICKY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_SW_STICKY_SPEC>;
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
impl From<crate::W<HW_OCOTP_SW_STICKY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_SW_STICKY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRK_REVOKE_LOCK` reader - SRK Revoke Lock"]
pub type SRK_REVOKE_LOCK_R = crate::BitReader<SRK_REVOKE_LOCK_A>;
#[doc = "SRK Revoke Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRK_REVOKE_LOCK_A {
    #[doc = "0: The writing of this region's shadow register and OTP fuse word are not blocked."]
    NOTBLOCKED = 0,
    #[doc = "1: The writing of this region's shadow register and OTP fuse word are blocked. Once this bit is set, it is always high unless a POR is issued."]
    BLOCKED = 1,
}
impl From<SRK_REVOKE_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SRK_REVOKE_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl SRK_REVOKE_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRK_REVOKE_LOCK_A {
        match self.bits {
            false => SRK_REVOKE_LOCK_A::NOTBLOCKED,
            true => SRK_REVOKE_LOCK_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBLOCKED`"]
    #[inline(always)]
    pub fn is_notblocked(&self) -> bool {
        *self == SRK_REVOKE_LOCK_A::NOTBLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SRK_REVOKE_LOCK_A::BLOCKED
    }
}
#[doc = "Field `SRK_REVOKE_LOCK` writer - SRK Revoke Lock"]
pub type SRK_REVOKE_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HW_OCOTP_SW_STICKY_SPEC, SRK_REVOKE_LOCK_A, O>;
impl<'a, const O: u8> SRK_REVOKE_LOCK_W<'a, O> {
    #[doc = "The writing of this region's shadow register and OTP fuse word are not blocked."]
    #[inline(always)]
    pub fn notblocked(self) -> &'a mut W {
        self.variant(SRK_REVOKE_LOCK_A::NOTBLOCKED)
    }
    #[doc = "The writing of this region's shadow register and OTP fuse word are blocked. Once this bit is set, it is always high unless a POR is issued."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SRK_REVOKE_LOCK_A::BLOCKED)
    }
}
#[doc = "Field `FIELD_RETURN_LOCK` reader - Field Return Lock"]
pub type FIELD_RETURN_LOCK_R = crate::BitReader<FIELD_RETURN_LOCK_A>;
#[doc = "Field Return Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELD_RETURN_LOCK_A {
    #[doc = "0: Writing to this region's shadow register and OTP fuse word are not blocked."]
    NOTBLOCKED = 0,
    #[doc = "1: Writing to this region's shadow register and OTP fuse word are blocked. Once this bit is set, it is always high unless a POR is issued."]
    BLOCKED = 1,
}
impl From<FIELD_RETURN_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: FIELD_RETURN_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELD_RETURN_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELD_RETURN_LOCK_A {
        match self.bits {
            false => FIELD_RETURN_LOCK_A::NOTBLOCKED,
            true => FIELD_RETURN_LOCK_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBLOCKED`"]
    #[inline(always)]
    pub fn is_notblocked(&self) -> bool {
        *self == FIELD_RETURN_LOCK_A::NOTBLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == FIELD_RETURN_LOCK_A::BLOCKED
    }
}
#[doc = "Field `FIELD_RETURN_LOCK` writer - Field Return Lock"]
pub type FIELD_RETURN_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HW_OCOTP_SW_STICKY_SPEC, FIELD_RETURN_LOCK_A, O>;
impl<'a, const O: u8> FIELD_RETURN_LOCK_W<'a, O> {
    #[doc = "Writing to this region's shadow register and OTP fuse word are not blocked."]
    #[inline(always)]
    pub fn notblocked(self) -> &'a mut W {
        self.variant(FIELD_RETURN_LOCK_A::NOTBLOCKED)
    }
    #[doc = "Writing to this region's shadow register and OTP fuse word are blocked. Once this bit is set, it is always high unless a POR is issued."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(FIELD_RETURN_LOCK_A::BLOCKED)
    }
}
impl R {
    #[doc = "Bit 1 - SRK Revoke Lock"]
    #[inline(always)]
    pub fn srk_revoke_lock(&self) -> SRK_REVOKE_LOCK_R {
        SRK_REVOKE_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Field Return Lock"]
    #[inline(always)]
    pub fn field_return_lock(&self) -> FIELD_RETURN_LOCK_R {
        FIELD_RETURN_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRK Revoke Lock"]
    #[inline(always)]
    #[must_use]
    pub fn srk_revoke_lock(&mut self) -> SRK_REVOKE_LOCK_W<1> {
        SRK_REVOKE_LOCK_W::new(self)
    }
    #[doc = "Bit 2 - Field Return Lock"]
    #[inline(always)]
    #[must_use]
    pub fn field_return_lock(&mut self) -> FIELD_RETURN_LOCK_W<2> {
        FIELD_RETURN_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sticky bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_sw_sticky](index.html) module"]
pub struct HW_OCOTP_SW_STICKY_SPEC;
impl crate::RegisterSpec for HW_OCOTP_SW_STICKY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_sw_sticky::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_SW_STICKY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_sw_sticky::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_SW_STICKY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_SW_STICKY to value 0"]
impl crate::Resettable for HW_OCOTP_SW_STICKY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
