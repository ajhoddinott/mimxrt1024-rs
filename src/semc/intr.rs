#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPCMDDONE` reader - IP command normal done interrupt"]
pub type IPCMDDONE_R = crate::BitReader<IPCMDDONE_A>;
#[doc = "IP command normal done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCMDDONE_A {
    #[doc = "0: IP command is not done."]
    IPCMDDONE_0 = 0,
    #[doc = "1: IP command is done."]
    IPCMDDONE_1 = 1,
}
impl From<IPCMDDONE_A> for bool {
    #[inline(always)]
    fn from(variant: IPCMDDONE_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCMDDONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCMDDONE_A {
        match self.bits {
            false => IPCMDDONE_A::IPCMDDONE_0,
            true => IPCMDDONE_A::IPCMDDONE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPCMDDONE_0`"]
    #[inline(always)]
    pub fn is_ipcmddone_0(&self) -> bool {
        *self == IPCMDDONE_A::IPCMDDONE_0
    }
    #[doc = "Checks if the value of the field is `IPCMDDONE_1`"]
    #[inline(always)]
    pub fn is_ipcmddone_1(&self) -> bool {
        *self == IPCMDDONE_A::IPCMDDONE_1
    }
}
#[doc = "Field `IPCMDDONE` writer - IP command normal done interrupt"]
pub type IPCMDDONE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, IPCMDDONE_A, O>;
impl<'a, const O: u8> IPCMDDONE_W<'a, O> {
    #[doc = "IP command is not done."]
    #[inline(always)]
    pub fn ipcmddone_0(self) -> &'a mut W {
        self.variant(IPCMDDONE_A::IPCMDDONE_0)
    }
    #[doc = "IP command is done."]
    #[inline(always)]
    pub fn ipcmddone_1(self) -> &'a mut W {
        self.variant(IPCMDDONE_A::IPCMDDONE_1)
    }
}
#[doc = "Field `IPCMDERR` reader - IP command error done interrupt"]
pub type IPCMDERR_R = crate::BitReader<IPCMDERR_A>;
#[doc = "IP command error done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCMDERR_A {
    #[doc = "0: No IP command error."]
    IPCMDERR_0 = 0,
    #[doc = "1: IP command error occurs."]
    IPCMDERR_1 = 1,
}
impl From<IPCMDERR_A> for bool {
    #[inline(always)]
    fn from(variant: IPCMDERR_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCMDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCMDERR_A {
        match self.bits {
            false => IPCMDERR_A::IPCMDERR_0,
            true => IPCMDERR_A::IPCMDERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPCMDERR_0`"]
    #[inline(always)]
    pub fn is_ipcmderr_0(&self) -> bool {
        *self == IPCMDERR_A::IPCMDERR_0
    }
    #[doc = "Checks if the value of the field is `IPCMDERR_1`"]
    #[inline(always)]
    pub fn is_ipcmderr_1(&self) -> bool {
        *self == IPCMDERR_A::IPCMDERR_1
    }
}
#[doc = "Field `IPCMDERR` writer - IP command error done interrupt"]
pub type IPCMDERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, IPCMDERR_A, O>;
impl<'a, const O: u8> IPCMDERR_W<'a, O> {
    #[doc = "No IP command error."]
    #[inline(always)]
    pub fn ipcmderr_0(self) -> &'a mut W {
        self.variant(IPCMDERR_A::IPCMDERR_0)
    }
    #[doc = "IP command error occurs."]
    #[inline(always)]
    pub fn ipcmderr_1(self) -> &'a mut W {
        self.variant(IPCMDERR_A::IPCMDERR_1)
    }
}
#[doc = "Field `AXICMDERR` reader - AXI command error interrupt"]
pub type AXICMDERR_R = crate::BitReader<AXICMDERR_A>;
#[doc = "AXI command error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXICMDERR_A {
    #[doc = "0: No AXI command error."]
    AXICMDERR_0 = 0,
    #[doc = "1: AXI command error occurs."]
    AXICMDERR_1 = 1,
}
impl From<AXICMDERR_A> for bool {
    #[inline(always)]
    fn from(variant: AXICMDERR_A) -> Self {
        variant as u8 != 0
    }
}
impl AXICMDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXICMDERR_A {
        match self.bits {
            false => AXICMDERR_A::AXICMDERR_0,
            true => AXICMDERR_A::AXICMDERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXICMDERR_0`"]
    #[inline(always)]
    pub fn is_axicmderr_0(&self) -> bool {
        *self == AXICMDERR_A::AXICMDERR_0
    }
    #[doc = "Checks if the value of the field is `AXICMDERR_1`"]
    #[inline(always)]
    pub fn is_axicmderr_1(&self) -> bool {
        *self == AXICMDERR_A::AXICMDERR_1
    }
}
#[doc = "Field `AXICMDERR` writer - AXI command error interrupt"]
pub type AXICMDERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, AXICMDERR_A, O>;
impl<'a, const O: u8> AXICMDERR_W<'a, O> {
    #[doc = "No AXI command error."]
    #[inline(always)]
    pub fn axicmderr_0(self) -> &'a mut W {
        self.variant(AXICMDERR_A::AXICMDERR_0)
    }
    #[doc = "AXI command error occurs."]
    #[inline(always)]
    pub fn axicmderr_1(self) -> &'a mut W {
        self.variant(AXICMDERR_A::AXICMDERR_1)
    }
}
#[doc = "Field `AXIBUSERR` reader - AXI bus error interrupt"]
pub type AXIBUSERR_R = crate::BitReader<AXIBUSERR_A>;
#[doc = "AXI bus error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXIBUSERR_A {
    #[doc = "0: No AXI bus error."]
    AXIBUSERR_0 = 0,
    #[doc = "1: AXI bus error occurs."]
    AXIBUSERR_1 = 1,
}
impl From<AXIBUSERR_A> for bool {
    #[inline(always)]
    fn from(variant: AXIBUSERR_A) -> Self {
        variant as u8 != 0
    }
}
impl AXIBUSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIBUSERR_A {
        match self.bits {
            false => AXIBUSERR_A::AXIBUSERR_0,
            true => AXIBUSERR_A::AXIBUSERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXIBUSERR_0`"]
    #[inline(always)]
    pub fn is_axibuserr_0(&self) -> bool {
        *self == AXIBUSERR_A::AXIBUSERR_0
    }
    #[doc = "Checks if the value of the field is `AXIBUSERR_1`"]
    #[inline(always)]
    pub fn is_axibuserr_1(&self) -> bool {
        *self == AXIBUSERR_A::AXIBUSERR_1
    }
}
#[doc = "Field `AXIBUSERR` writer - AXI bus error interrupt"]
pub type AXIBUSERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, AXIBUSERR_A, O>;
impl<'a, const O: u8> AXIBUSERR_W<'a, O> {
    #[doc = "No AXI bus error."]
    #[inline(always)]
    pub fn axibuserr_0(self) -> &'a mut W {
        self.variant(AXIBUSERR_A::AXIBUSERR_0)
    }
    #[doc = "AXI bus error occurs."]
    #[inline(always)]
    pub fn axibuserr_1(self) -> &'a mut W {
        self.variant(AXIBUSERR_A::AXIBUSERR_1)
    }
}
#[doc = "Field `NDPAGEEND` reader - NAND page end interrupt"]
pub type NDPAGEEND_R = crate::BitReader<NDPAGEEND_A>;
#[doc = "NAND page end interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDPAGEEND_A {
    #[doc = "0: The last address of main space in the NAND is not written by AXI command."]
    NDPAGEEND_0 = 0,
    #[doc = "1: The last address of main space in the NAND is written by AXI command."]
    NDPAGEEND_1 = 1,
}
impl From<NDPAGEEND_A> for bool {
    #[inline(always)]
    fn from(variant: NDPAGEEND_A) -> Self {
        variant as u8 != 0
    }
}
impl NDPAGEEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDPAGEEND_A {
        match self.bits {
            false => NDPAGEEND_A::NDPAGEEND_0,
            true => NDPAGEEND_A::NDPAGEEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDPAGEEND_0`"]
    #[inline(always)]
    pub fn is_ndpageend_0(&self) -> bool {
        *self == NDPAGEEND_A::NDPAGEEND_0
    }
    #[doc = "Checks if the value of the field is `NDPAGEEND_1`"]
    #[inline(always)]
    pub fn is_ndpageend_1(&self) -> bool {
        *self == NDPAGEEND_A::NDPAGEEND_1
    }
}
#[doc = "Field `NDPAGEEND` writer - NAND page end interrupt"]
pub type NDPAGEEND_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, NDPAGEEND_A, O>;
impl<'a, const O: u8> NDPAGEEND_W<'a, O> {
    #[doc = "The last address of main space in the NAND is not written by AXI command."]
    #[inline(always)]
    pub fn ndpageend_0(self) -> &'a mut W {
        self.variant(NDPAGEEND_A::NDPAGEEND_0)
    }
    #[doc = "The last address of main space in the NAND is written by AXI command."]
    #[inline(always)]
    pub fn ndpageend_1(self) -> &'a mut W {
        self.variant(NDPAGEEND_A::NDPAGEEND_1)
    }
}
#[doc = "Field `NDNOPEND` reader - NAND no pending AXI write transaction interrupt"]
pub type NDNOPEND_R = crate::BitReader<NDNOPEND_A>;
#[doc = "NAND no pending AXI write transaction interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDNOPEND_A {
    #[doc = "0: At least one NAND AXI write transaction is pending or no NAND write transaction is sent to the queue."]
    NDNOPEND_0 = 0,
    #[doc = "1: All NAND AXI write pending transactions are finished."]
    NDNOPEND_1 = 1,
}
impl From<NDNOPEND_A> for bool {
    #[inline(always)]
    fn from(variant: NDNOPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl NDNOPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDNOPEND_A {
        match self.bits {
            false => NDNOPEND_A::NDNOPEND_0,
            true => NDNOPEND_A::NDNOPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDNOPEND_0`"]
    #[inline(always)]
    pub fn is_ndnopend_0(&self) -> bool {
        *self == NDNOPEND_A::NDNOPEND_0
    }
    #[doc = "Checks if the value of the field is `NDNOPEND_1`"]
    #[inline(always)]
    pub fn is_ndnopend_1(&self) -> bool {
        *self == NDNOPEND_A::NDNOPEND_1
    }
}
#[doc = "Field `NDNOPEND` writer - NAND no pending AXI write transaction interrupt"]
pub type NDNOPEND_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, NDNOPEND_A, O>;
impl<'a, const O: u8> NDNOPEND_W<'a, O> {
    #[doc = "At least one NAND AXI write transaction is pending or no NAND write transaction is sent to the queue."]
    #[inline(always)]
    pub fn ndnopend_0(self) -> &'a mut W {
        self.variant(NDNOPEND_A::NDNOPEND_0)
    }
    #[doc = "All NAND AXI write pending transactions are finished."]
    #[inline(always)]
    pub fn ndnopend_1(self) -> &'a mut W {
        self.variant(NDNOPEND_A::NDNOPEND_1)
    }
}
impl R {
    #[doc = "Bit 0 - IP command normal done interrupt"]
    #[inline(always)]
    pub fn ipcmddone(&self) -> IPCMDDONE_R {
        IPCMDDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP command error done interrupt"]
    #[inline(always)]
    pub fn ipcmderr(&self) -> IPCMDERR_R {
        IPCMDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI command error interrupt"]
    #[inline(always)]
    pub fn axicmderr(&self) -> AXICMDERR_R {
        AXICMDERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI bus error interrupt"]
    #[inline(always)]
    pub fn axibuserr(&self) -> AXIBUSERR_R {
        AXIBUSERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAND page end interrupt"]
    #[inline(always)]
    pub fn ndpageend(&self) -> NDPAGEEND_R {
        NDPAGEEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAND no pending AXI write transaction interrupt"]
    #[inline(always)]
    pub fn ndnopend(&self) -> NDNOPEND_R {
        NDNOPEND_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP command normal done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ipcmddone(&mut self) -> IPCMDDONE_W<0> {
        IPCMDDONE_W::new(self)
    }
    #[doc = "Bit 1 - IP command error done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ipcmderr(&mut self) -> IPCMDERR_W<1> {
        IPCMDERR_W::new(self)
    }
    #[doc = "Bit 2 - AXI command error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn axicmderr(&mut self) -> AXICMDERR_W<2> {
        AXICMDERR_W::new(self)
    }
    #[doc = "Bit 3 - AXI bus error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn axibuserr(&mut self) -> AXIBUSERR_W<3> {
        AXIBUSERR_W::new(self)
    }
    #[doc = "Bit 4 - NAND page end interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ndpageend(&mut self) -> NDPAGEEND_W<4> {
        NDPAGEEND_W::new(self)
    }
    #[doc = "Bit 5 - NAND no pending AXI write transaction interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ndnopend(&mut self) -> NDNOPEND_W<5> {
        NDNOPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x3f;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
