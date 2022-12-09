#[doc = "Register `IPCR2` reader"]
pub struct R(crate::R<IPCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCR2` writer"]
pub struct W(crate::W<IPCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCR2_SPEC>;
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
impl From<crate::W<IPCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BM0` reader - Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
pub type BM0_R = crate::BitReader<BM0_A>;
#[doc = "Byte Mask for Byte 0 (IPTXDAT bit 7:0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BM0_A {
    #[doc = "0: Byte is unmasked"]
    BM0_0 = 0,
    #[doc = "1: Byte is masked"]
    BM0_1 = 1,
}
impl From<BM0_A> for bool {
    #[inline(always)]
    fn from(variant: BM0_A) -> Self {
        variant as u8 != 0
    }
}
impl BM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM0_A {
        match self.bits {
            false => BM0_A::BM0_0,
            true => BM0_A::BM0_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM0_0`"]
    #[inline(always)]
    pub fn is_bm0_0(&self) -> bool {
        *self == BM0_A::BM0_0
    }
    #[doc = "Checks if the value of the field is `BM0_1`"]
    #[inline(always)]
    pub fn is_bm0_1(&self) -> bool {
        *self == BM0_A::BM0_1
    }
}
#[doc = "Field `BM0` writer - Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
pub type BM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCR2_SPEC, BM0_A, O>;
impl<'a, const O: u8> BM0_W<'a, O> {
    #[doc = "Byte is unmasked"]
    #[inline(always)]
    pub fn bm0_0(self) -> &'a mut W {
        self.variant(BM0_A::BM0_0)
    }
    #[doc = "Byte is masked"]
    #[inline(always)]
    pub fn bm0_1(self) -> &'a mut W {
        self.variant(BM0_A::BM0_1)
    }
}
#[doc = "Field `BM1` reader - Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
pub type BM1_R = crate::BitReader<BM1_A>;
#[doc = "Byte Mask for Byte 1 (IPTXDAT bit 15:8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BM1_A {
    #[doc = "0: Byte is unmasked"]
    BM1_0 = 0,
    #[doc = "1: Byte is masked"]
    BM1_1 = 1,
}
impl From<BM1_A> for bool {
    #[inline(always)]
    fn from(variant: BM1_A) -> Self {
        variant as u8 != 0
    }
}
impl BM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM1_A {
        match self.bits {
            false => BM1_A::BM1_0,
            true => BM1_A::BM1_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM1_0`"]
    #[inline(always)]
    pub fn is_bm1_0(&self) -> bool {
        *self == BM1_A::BM1_0
    }
    #[doc = "Checks if the value of the field is `BM1_1`"]
    #[inline(always)]
    pub fn is_bm1_1(&self) -> bool {
        *self == BM1_A::BM1_1
    }
}
#[doc = "Field `BM1` writer - Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
pub type BM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCR2_SPEC, BM1_A, O>;
impl<'a, const O: u8> BM1_W<'a, O> {
    #[doc = "Byte is unmasked"]
    #[inline(always)]
    pub fn bm1_0(self) -> &'a mut W {
        self.variant(BM1_A::BM1_0)
    }
    #[doc = "Byte is masked"]
    #[inline(always)]
    pub fn bm1_1(self) -> &'a mut W {
        self.variant(BM1_A::BM1_1)
    }
}
#[doc = "Field `BM2` reader - Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
pub type BM2_R = crate::BitReader<BM2_A>;
#[doc = "Byte Mask for Byte 2 (IPTXDAT bit 23:16)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BM2_A {
    #[doc = "0: Byte is unmasked"]
    BM2_0 = 0,
    #[doc = "1: Byte is masked"]
    BM2_1 = 1,
}
impl From<BM2_A> for bool {
    #[inline(always)]
    fn from(variant: BM2_A) -> Self {
        variant as u8 != 0
    }
}
impl BM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM2_A {
        match self.bits {
            false => BM2_A::BM2_0,
            true => BM2_A::BM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM2_0`"]
    #[inline(always)]
    pub fn is_bm2_0(&self) -> bool {
        *self == BM2_A::BM2_0
    }
    #[doc = "Checks if the value of the field is `BM2_1`"]
    #[inline(always)]
    pub fn is_bm2_1(&self) -> bool {
        *self == BM2_A::BM2_1
    }
}
#[doc = "Field `BM2` writer - Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
pub type BM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCR2_SPEC, BM2_A, O>;
impl<'a, const O: u8> BM2_W<'a, O> {
    #[doc = "Byte is unmasked"]
    #[inline(always)]
    pub fn bm2_0(self) -> &'a mut W {
        self.variant(BM2_A::BM2_0)
    }
    #[doc = "Byte is masked"]
    #[inline(always)]
    pub fn bm2_1(self) -> &'a mut W {
        self.variant(BM2_A::BM2_1)
    }
}
#[doc = "Field `BM3` reader - Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
pub type BM3_R = crate::BitReader<BM3_A>;
#[doc = "Byte Mask for Byte 3 (IPTXDAT bit 31:24)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BM3_A {
    #[doc = "0: Byte is unmasked"]
    BM3_0 = 0,
    #[doc = "1: Byte is masked"]
    BM3_1 = 1,
}
impl From<BM3_A> for bool {
    #[inline(always)]
    fn from(variant: BM3_A) -> Self {
        variant as u8 != 0
    }
}
impl BM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM3_A {
        match self.bits {
            false => BM3_A::BM3_0,
            true => BM3_A::BM3_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM3_0`"]
    #[inline(always)]
    pub fn is_bm3_0(&self) -> bool {
        *self == BM3_A::BM3_0
    }
    #[doc = "Checks if the value of the field is `BM3_1`"]
    #[inline(always)]
    pub fn is_bm3_1(&self) -> bool {
        *self == BM3_A::BM3_1
    }
}
#[doc = "Field `BM3` writer - Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
pub type BM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCR2_SPEC, BM3_A, O>;
impl<'a, const O: u8> BM3_W<'a, O> {
    #[doc = "Byte is unmasked"]
    #[inline(always)]
    pub fn bm3_0(self) -> &'a mut W {
        self.variant(BM3_A::BM3_0)
    }
    #[doc = "Byte is masked"]
    #[inline(always)]
    pub fn bm3_1(self) -> &'a mut W {
        self.variant(BM3_A::BM3_1)
    }
}
impl R {
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
    #[inline(always)]
    pub fn bm0(&self) -> BM0_R {
        BM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
    #[inline(always)]
    pub fn bm1(&self) -> BM1_R {
        BM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
    #[inline(always)]
    pub fn bm2(&self) -> BM2_R {
        BM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
    #[inline(always)]
    pub fn bm3(&self) -> BM3_R {
        BM3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
    #[inline(always)]
    #[must_use]
    pub fn bm0(&mut self) -> BM0_W<0> {
        BM0_W::new(self)
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
    #[inline(always)]
    #[must_use]
    pub fn bm1(&mut self) -> BM1_W<1> {
        BM1_W::new(self)
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
    #[inline(always)]
    #[must_use]
    pub fn bm2(&mut self) -> BM2_W<2> {
        BM2_W::new(self)
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
    #[inline(always)]
    #[must_use]
    pub fn bm3(&mut self) -> BM3_W<3> {
        BM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Command Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr2](index.html) module"]
pub struct IPCR2_SPEC;
impl crate::RegisterSpec for IPCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcr2::R](R) reader structure"]
impl crate::Readable for IPCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcr2::W](W) writer structure"]
impl crate::Writable for IPCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCR2 to value 0"]
impl crate::Resettable for IPCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
