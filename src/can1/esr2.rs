#[doc = "Register `ESR2` reader"]
pub struct R(crate::R<ESR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMB` reader - If ESR2\\[VPS\\]
is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)"]
pub type IMB_R = crate::BitReader<IMB_A>;
#[doc = "If ESR2\\[VPS\\]
is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMB_A {
    #[doc = "0: If ESR2\\[VPS\\]
is asserted, the ESR2\\[LPTM\\]
is not an inactive Mailbox."]
    IMB_0 = 0,
    #[doc = "1: If ESR2\\[VPS\\]
is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    IMB_1 = 1,
}
impl From<IMB_A> for bool {
    #[inline(always)]
    fn from(variant: IMB_A) -> Self {
        variant as u8 != 0
    }
}
impl IMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMB_A {
        match self.bits {
            false => IMB_A::IMB_0,
            true => IMB_A::IMB_1,
        }
    }
    #[doc = "Checks if the value of the field is `IMB_0`"]
    #[inline(always)]
    pub fn is_imb_0(&self) -> bool {
        *self == IMB_A::IMB_0
    }
    #[doc = "Checks if the value of the field is `IMB_1`"]
    #[inline(always)]
    pub fn is_imb_1(&self) -> bool {
        *self == IMB_A::IMB_1
    }
}
#[doc = "Field `VPS` reader - This bit indicates whether IMB and LPTM contents are currently valid or not"]
pub type VPS_R = crate::BitReader<VPS_A>;
#[doc = "This bit indicates whether IMB and LPTM contents are currently valid or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPS_A {
    #[doc = "0: Contents of IMB and LPTM are invalid"]
    VPS_0 = 0,
    #[doc = "1: Contents of IMB and LPTM are valid"]
    VPS_1 = 1,
}
impl From<VPS_A> for bool {
    #[inline(always)]
    fn from(variant: VPS_A) -> Self {
        variant as u8 != 0
    }
}
impl VPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPS_A {
        match self.bits {
            false => VPS_A::VPS_0,
            true => VPS_A::VPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `VPS_0`"]
    #[inline(always)]
    pub fn is_vps_0(&self) -> bool {
        *self == VPS_A::VPS_0
    }
    #[doc = "Checks if the value of the field is `VPS_1`"]
    #[inline(always)]
    pub fn is_vps_1(&self) -> bool {
        *self == VPS_A::VPS_1
    }
}
#[doc = "Field `LPTM` reader - If ESR2\\[VPS\\]
is asserted, his 7-bit field indicates the lowest number inactive Mailbox (refer to IMB bit description)"]
pub type LPTM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 13 - If ESR2\\[VPS\\]
is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)"]
    #[inline(always)]
    pub fn imb(&self) -> IMB_R {
        IMB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit indicates whether IMB and LPTM contents are currently valid or not"]
    #[inline(always)]
    pub fn vps(&self) -> VPS_R {
        VPS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - If ESR2\\[VPS\\]
is asserted, his 7-bit field indicates the lowest number inactive Mailbox (refer to IMB bit description)"]
    #[inline(always)]
    pub fn lptm(&self) -> LPTM_R {
        LPTM_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Error and Status 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr2](index.html) module"]
pub struct ESR2_SPEC;
impl crate::RegisterSpec for ESR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr2::R](R) reader structure"]
impl crate::Readable for ESR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESR2 to value 0"]
impl crate::Resettable for ESR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
