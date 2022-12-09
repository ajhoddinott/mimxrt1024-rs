#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EACEN` reader - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
pub type EACEN_R = crate::BitReader<EACEN_A>;
#[doc = "This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EACEN_A {
    #[doc = "0: Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    EACEN_0 = 0,
    #[doc = "1: Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    EACEN_1 = 1,
}
impl From<EACEN_A> for bool {
    #[inline(always)]
    fn from(variant: EACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EACEN_A {
        match self.bits {
            false => EACEN_A::EACEN_0,
            true => EACEN_A::EACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EACEN_0`"]
    #[inline(always)]
    pub fn is_eacen_0(&self) -> bool {
        *self == EACEN_A::EACEN_0
    }
    #[doc = "Checks if the value of the field is `EACEN_1`"]
    #[inline(always)]
    pub fn is_eacen_1(&self) -> bool {
        *self == EACEN_A::EACEN_1
    }
}
#[doc = "Field `EACEN` writer - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
pub type EACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, EACEN_A, O>;
impl<'a, const O: u8> EACEN_W<'a, O> {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn eacen_0(self) -> &'a mut W {
        self.variant(EACEN_A::EACEN_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn eacen_1(self) -> &'a mut W {
        self.variant(EACEN_A::EACEN_1)
    }
}
#[doc = "Field `RRS` reader - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
pub type RRS_R = crate::BitReader<RRS_A>;
#[doc = "If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRS_A {
    #[doc = "0: Remote Response Frame is generated"]
    RRS_0 = 0,
    #[doc = "1: Remote Request Frame is stored"]
    RRS_1 = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::RRS_0,
            true => RRS_A::RRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRS_0`"]
    #[inline(always)]
    pub fn is_rrs_0(&self) -> bool {
        *self == RRS_A::RRS_0
    }
    #[doc = "Checks if the value of the field is `RRS_1`"]
    #[inline(always)]
    pub fn is_rrs_1(&self) -> bool {
        *self == RRS_A::RRS_1
    }
}
#[doc = "Field `RRS` writer - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
pub type RRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, RRS_A, O>;
impl<'a, const O: u8> RRS_W<'a, O> {
    #[doc = "Remote Response Frame is generated"]
    #[inline(always)]
    pub fn rrs_0(self) -> &'a mut W {
        self.variant(RRS_A::RRS_0)
    }
    #[doc = "Remote Request Frame is stored"]
    #[inline(always)]
    pub fn rrs_1(self) -> &'a mut W {
        self.variant(RRS_A::RRS_1)
    }
}
#[doc = "Field `MRP` reader - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
pub type MRP_R = crate::BitReader<MRP_A>;
#[doc = "If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRP_A {
    #[doc = "0: Matching starts from Rx FIFO and continues on Mailboxes"]
    MRP_0 = 0,
    #[doc = "1: Matching starts from Mailboxes and continues on Rx FIFO"]
    MRP_1 = 1,
}
impl From<MRP_A> for bool {
    #[inline(always)]
    fn from(variant: MRP_A) -> Self {
        variant as u8 != 0
    }
}
impl MRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRP_A {
        match self.bits {
            false => MRP_A::MRP_0,
            true => MRP_A::MRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRP_0`"]
    #[inline(always)]
    pub fn is_mrp_0(&self) -> bool {
        *self == MRP_A::MRP_0
    }
    #[doc = "Checks if the value of the field is `MRP_1`"]
    #[inline(always)]
    pub fn is_mrp_1(&self) -> bool {
        *self == MRP_A::MRP_1
    }
}
#[doc = "Field `MRP` writer - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
pub type MRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, MRP_A, O>;
impl<'a, const O: u8> MRP_W<'a, O> {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes"]
    #[inline(always)]
    pub fn mrp_0(self) -> &'a mut W {
        self.variant(MRP_A::MRP_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO"]
    #[inline(always)]
    pub fn mrp_1(self) -> &'a mut W {
        self.variant(MRP_A::MRP_1)
    }
}
#[doc = "Field `TASD` reader - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
pub type TASD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TASD` writer - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
pub type TASD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RFFN` reader - This 4-bit field defines the number of Rx FIFO filters according to"]
pub type RFFN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFFN` writer - This 4-bit field defines the number of Rx FIFO filters according to"]
pub type RFFN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `WRMFRZ` reader - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
pub type WRMFRZ_R = crate::BitReader<WRMFRZ_A>;
#[doc = "Enable unrestricted write access to FlexCAN memory in Freeze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRMFRZ_A {
    #[doc = "0: Keep the write access restricted in some regions of FlexCAN memory"]
    WRMFRZ_0 = 0,
    #[doc = "1: Enable unrestricted write access to FlexCAN memory"]
    WRMFRZ_1 = 1,
}
impl From<WRMFRZ_A> for bool {
    #[inline(always)]
    fn from(variant: WRMFRZ_A) -> Self {
        variant as u8 != 0
    }
}
impl WRMFRZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRMFRZ_A {
        match self.bits {
            false => WRMFRZ_A::WRMFRZ_0,
            true => WRMFRZ_A::WRMFRZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `WRMFRZ_0`"]
    #[inline(always)]
    pub fn is_wrmfrz_0(&self) -> bool {
        *self == WRMFRZ_A::WRMFRZ_0
    }
    #[doc = "Checks if the value of the field is `WRMFRZ_1`"]
    #[inline(always)]
    pub fn is_wrmfrz_1(&self) -> bool {
        *self == WRMFRZ_A::WRMFRZ_1
    }
}
#[doc = "Field `WRMFRZ` writer - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
pub type WRMFRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, WRMFRZ_A, O>;
impl<'a, const O: u8> WRMFRZ_W<'a, O> {
    #[doc = "Keep the write access restricted in some regions of FlexCAN memory"]
    #[inline(always)]
    pub fn wrmfrz_0(self) -> &'a mut W {
        self.variant(WRMFRZ_A::WRMFRZ_0)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory"]
    #[inline(always)]
    pub fn wrmfrz_1(self) -> &'a mut W {
        self.variant(WRMFRZ_A::WRMFRZ_1)
    }
}
impl R {
    #[doc = "Bit 16 - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[inline(always)]
    pub fn eacen(&self) -> EACEN_R {
        EACEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[inline(always)]
    pub fn mrp(&self) -> MRP_R {
        MRP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[inline(always)]
    pub fn tasd(&self) -> TASD_R {
        TASD_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - This 4-bit field defines the number of Rx FIFO filters according to"]
    #[inline(always)]
    pub fn rffn(&self) -> RFFN_R {
        RFFN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[inline(always)]
    pub fn wrmfrz(&self) -> WRMFRZ_R {
        WRMFRZ_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[inline(always)]
    #[must_use]
    pub fn eacen(&mut self) -> EACEN_W<16> {
        EACEN_W::new(self)
    }
    #[doc = "Bit 17 - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<17> {
        RRS_W::new(self)
    }
    #[doc = "Bit 18 - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn mrp(&mut self) -> MRP_W<18> {
        MRP_W::new(self)
    }
    #[doc = "Bits 19:23 - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[inline(always)]
    #[must_use]
    pub fn tasd(&mut self) -> TASD_W<19> {
        TASD_W::new(self)
    }
    #[doc = "Bits 24:27 - This 4-bit field defines the number of Rx FIFO filters according to"]
    #[inline(always)]
    #[must_use]
    pub fn rffn(&mut self) -> RFFN_W<24> {
        RFFN_W::new(self)
    }
    #[doc = "Bit 28 - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[inline(always)]
    #[must_use]
    pub fn wrmfrz(&mut self) -> WRMFRZ_W<28> {
        WRMFRZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
