#[doc = "Register `CNTR` reader"]
pub struct R(crate::R<CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTR` writer"]
pub struct W(crate::W<CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR_SPEC>;
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
impl From<crate::W<CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEGA_PDN_REQ` reader - MEGA domain (FlexRAM PDRAM1) power down request"]
pub type MEGA_PDN_REQ_R = crate::BitReader<MEGA_PDN_REQ_A>;
#[doc = "MEGA domain (FlexRAM PDRAM1) power down request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEGA_PDN_REQ_A {
    #[doc = "0: No Request"]
    MEGA_PDN_REQ_0 = 0,
    #[doc = "1: Request power down sequence"]
    MEGA_PDN_REQ_1 = 1,
}
impl From<MEGA_PDN_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: MEGA_PDN_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl MEGA_PDN_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEGA_PDN_REQ_A {
        match self.bits {
            false => MEGA_PDN_REQ_A::MEGA_PDN_REQ_0,
            true => MEGA_PDN_REQ_A::MEGA_PDN_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEGA_PDN_REQ_0`"]
    #[inline(always)]
    pub fn is_mega_pdn_req_0(&self) -> bool {
        *self == MEGA_PDN_REQ_A::MEGA_PDN_REQ_0
    }
    #[doc = "Checks if the value of the field is `MEGA_PDN_REQ_1`"]
    #[inline(always)]
    pub fn is_mega_pdn_req_1(&self) -> bool {
        *self == MEGA_PDN_REQ_A::MEGA_PDN_REQ_1
    }
}
#[doc = "Field `MEGA_PDN_REQ` writer - MEGA domain (FlexRAM PDRAM1) power down request"]
pub type MEGA_PDN_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, MEGA_PDN_REQ_A, O>;
impl<'a, const O: u8> MEGA_PDN_REQ_W<'a, O> {
    #[doc = "No Request"]
    #[inline(always)]
    pub fn mega_pdn_req_0(self) -> &'a mut W {
        self.variant(MEGA_PDN_REQ_A::MEGA_PDN_REQ_0)
    }
    #[doc = "Request power down sequence"]
    #[inline(always)]
    pub fn mega_pdn_req_1(self) -> &'a mut W {
        self.variant(MEGA_PDN_REQ_A::MEGA_PDN_REQ_1)
    }
}
#[doc = "Field `MEGA_PUP_REQ` reader - MEGA domain (FlexRAM PDRAM1) power up request"]
pub type MEGA_PUP_REQ_R = crate::BitReader<MEGA_PUP_REQ_A>;
#[doc = "MEGA domain (FlexRAM PDRAM1) power up request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEGA_PUP_REQ_A {
    #[doc = "0: No Request"]
    MEGA_PUP_REQ_0 = 0,
    #[doc = "1: Request power up sequence"]
    MEGA_PUP_REQ_1 = 1,
}
impl From<MEGA_PUP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: MEGA_PUP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl MEGA_PUP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEGA_PUP_REQ_A {
        match self.bits {
            false => MEGA_PUP_REQ_A::MEGA_PUP_REQ_0,
            true => MEGA_PUP_REQ_A::MEGA_PUP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEGA_PUP_REQ_0`"]
    #[inline(always)]
    pub fn is_mega_pup_req_0(&self) -> bool {
        *self == MEGA_PUP_REQ_A::MEGA_PUP_REQ_0
    }
    #[doc = "Checks if the value of the field is `MEGA_PUP_REQ_1`"]
    #[inline(always)]
    pub fn is_mega_pup_req_1(&self) -> bool {
        *self == MEGA_PUP_REQ_A::MEGA_PUP_REQ_1
    }
}
#[doc = "Field `MEGA_PUP_REQ` writer - MEGA domain (FlexRAM PDRAM1) power up request"]
pub type MEGA_PUP_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, MEGA_PUP_REQ_A, O>;
impl<'a, const O: u8> MEGA_PUP_REQ_W<'a, O> {
    #[doc = "No Request"]
    #[inline(always)]
    pub fn mega_pup_req_0(self) -> &'a mut W {
        self.variant(MEGA_PUP_REQ_A::MEGA_PUP_REQ_0)
    }
    #[doc = "Request power up sequence"]
    #[inline(always)]
    pub fn mega_pup_req_1(self) -> &'a mut W {
        self.variant(MEGA_PUP_REQ_A::MEGA_PUP_REQ_1)
    }
}
#[doc = "Field `PDRAM0_PGE` reader - FlexRAM PDRAM0 Power Gate Enable"]
pub type PDRAM0_PGE_R = crate::BitReader<PDRAM0_PGE_A>;
#[doc = "FlexRAM PDRAM0 Power Gate Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRAM0_PGE_A {
    #[doc = "0: FlexRAM PDRAM0 domain will keep power even if the CPU core is powered down."]
    PDRAM0_PGE_0 = 0,
    #[doc = "1: FlexRAM PDRAM0 domain will be powered down when the CPU core is powered down.."]
    PDRAM0_PGE_1 = 1,
}
impl From<PDRAM0_PGE_A> for bool {
    #[inline(always)]
    fn from(variant: PDRAM0_PGE_A) -> Self {
        variant as u8 != 0
    }
}
impl PDRAM0_PGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRAM0_PGE_A {
        match self.bits {
            false => PDRAM0_PGE_A::PDRAM0_PGE_0,
            true => PDRAM0_PGE_A::PDRAM0_PGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDRAM0_PGE_0`"]
    #[inline(always)]
    pub fn is_pdram0_pge_0(&self) -> bool {
        *self == PDRAM0_PGE_A::PDRAM0_PGE_0
    }
    #[doc = "Checks if the value of the field is `PDRAM0_PGE_1`"]
    #[inline(always)]
    pub fn is_pdram0_pge_1(&self) -> bool {
        *self == PDRAM0_PGE_A::PDRAM0_PGE_1
    }
}
#[doc = "Field `PDRAM0_PGE` writer - FlexRAM PDRAM0 Power Gate Enable"]
pub type PDRAM0_PGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, PDRAM0_PGE_A, O>;
impl<'a, const O: u8> PDRAM0_PGE_W<'a, O> {
    #[doc = "FlexRAM PDRAM0 domain will keep power even if the CPU core is powered down."]
    #[inline(always)]
    pub fn pdram0_pge_0(self) -> &'a mut W {
        self.variant(PDRAM0_PGE_A::PDRAM0_PGE_0)
    }
    #[doc = "FlexRAM PDRAM0 domain will be powered down when the CPU core is powered down.."]
    #[inline(always)]
    pub fn pdram0_pge_1(self) -> &'a mut W {
        self.variant(PDRAM0_PGE_A::PDRAM0_PGE_1)
    }
}
impl R {
    #[doc = "Bit 2 - MEGA domain (FlexRAM PDRAM1) power down request"]
    #[inline(always)]
    pub fn mega_pdn_req(&self) -> MEGA_PDN_REQ_R {
        MEGA_PDN_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MEGA domain (FlexRAM PDRAM1) power up request"]
    #[inline(always)]
    pub fn mega_pup_req(&self) -> MEGA_PUP_REQ_R {
        MEGA_PUP_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 22 - FlexRAM PDRAM0 Power Gate Enable"]
    #[inline(always)]
    pub fn pdram0_pge(&self) -> PDRAM0_PGE_R {
        PDRAM0_PGE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MEGA domain (FlexRAM PDRAM1) power down request"]
    #[inline(always)]
    #[must_use]
    pub fn mega_pdn_req(&mut self) -> MEGA_PDN_REQ_W<2> {
        MEGA_PDN_REQ_W::new(self)
    }
    #[doc = "Bit 3 - MEGA domain (FlexRAM PDRAM1) power up request"]
    #[inline(always)]
    #[must_use]
    pub fn mega_pup_req(&mut self) -> MEGA_PUP_REQ_W<3> {
        MEGA_PUP_REQ_W::new(self)
    }
    #[doc = "Bit 22 - FlexRAM PDRAM0 Power Gate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdram0_pge(&mut self) -> PDRAM0_PGE_W<22> {
        PDRAM0_PGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPC Interface control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](index.html) module"]
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntr::R](R) reader structure"]
impl crate::Readable for CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntr::W](W) writer structure"]
impl crate::Writable for CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTR to value 0x0052_0000"]
impl crate::Resettable for CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0052_0000;
}
