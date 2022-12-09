#[doc = "Register `SDRAMCR0` reader"]
pub struct R(crate::R<SDRAMCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRAMCR0` writer"]
pub struct W(crate::W<SDRAMCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRAMCR0_SPEC>;
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
impl From<crate::W<SDRAMCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRAMCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - Port Size"]
pub type PS_R = crate::BitReader<PS_A>;
#[doc = "Port Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS_A {
    #[doc = "0: 8bit"]
    PS_0 = 0,
    #[doc = "1: 16bit"]
    PS_1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::PS_0,
            true => PS_A::PS_1,
        }
    }
    #[doc = "Checks if the value of the field is `PS_0`"]
    #[inline(always)]
    pub fn is_ps_0(&self) -> bool {
        *self == PS_A::PS_0
    }
    #[doc = "Checks if the value of the field is `PS_1`"]
    #[inline(always)]
    pub fn is_ps_1(&self) -> bool {
        *self == PS_A::PS_1
    }
}
#[doc = "Field `PS` writer - Port Size"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRAMCR0_SPEC, PS_A, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "8bit"]
    #[inline(always)]
    pub fn ps_0(self) -> &'a mut W {
        self.variant(PS_A::PS_0)
    }
    #[doc = "16bit"]
    #[inline(always)]
    pub fn ps_1(self) -> &'a mut W {
        self.variant(PS_A::PS_1)
    }
}
#[doc = "Field `BL` reader - Burst Length"]
pub type BL_R = crate::FieldReader<u8, BL_A>;
#[doc = "Burst Length\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BL_A {
    #[doc = "0: 1"]
    BL_0 = 0,
    #[doc = "1: 2"]
    BL_1 = 1,
    #[doc = "2: 4"]
    BL_2 = 2,
    #[doc = "3: 8"]
    BL_3 = 3,
    #[doc = "4: 8"]
    BL_4 = 4,
    #[doc = "5: 8"]
    BL_5 = 5,
    #[doc = "6: 8"]
    BL_6 = 6,
    #[doc = "7: 8"]
    BL_7 = 7,
}
impl From<BL_A> for u8 {
    #[inline(always)]
    fn from(variant: BL_A) -> Self {
        variant as _
    }
}
impl BL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BL_A {
        match self.bits {
            0 => BL_A::BL_0,
            1 => BL_A::BL_1,
            2 => BL_A::BL_2,
            3 => BL_A::BL_3,
            4 => BL_A::BL_4,
            5 => BL_A::BL_5,
            6 => BL_A::BL_6,
            7 => BL_A::BL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BL_0`"]
    #[inline(always)]
    pub fn is_bl_0(&self) -> bool {
        *self == BL_A::BL_0
    }
    #[doc = "Checks if the value of the field is `BL_1`"]
    #[inline(always)]
    pub fn is_bl_1(&self) -> bool {
        *self == BL_A::BL_1
    }
    #[doc = "Checks if the value of the field is `BL_2`"]
    #[inline(always)]
    pub fn is_bl_2(&self) -> bool {
        *self == BL_A::BL_2
    }
    #[doc = "Checks if the value of the field is `BL_3`"]
    #[inline(always)]
    pub fn is_bl_3(&self) -> bool {
        *self == BL_A::BL_3
    }
    #[doc = "Checks if the value of the field is `BL_4`"]
    #[inline(always)]
    pub fn is_bl_4(&self) -> bool {
        *self == BL_A::BL_4
    }
    #[doc = "Checks if the value of the field is `BL_5`"]
    #[inline(always)]
    pub fn is_bl_5(&self) -> bool {
        *self == BL_A::BL_5
    }
    #[doc = "Checks if the value of the field is `BL_6`"]
    #[inline(always)]
    pub fn is_bl_6(&self) -> bool {
        *self == BL_A::BL_6
    }
    #[doc = "Checks if the value of the field is `BL_7`"]
    #[inline(always)]
    pub fn is_bl_7(&self) -> bool {
        *self == BL_A::BL_7
    }
}
#[doc = "Field `BL` writer - Burst Length"]
pub type BL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDRAMCR0_SPEC, u8, BL_A, 3, O>;
impl<'a, const O: u8> BL_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn bl_0(self) -> &'a mut W {
        self.variant(BL_A::BL_0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn bl_1(self) -> &'a mut W {
        self.variant(BL_A::BL_1)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn bl_2(self) -> &'a mut W {
        self.variant(BL_A::BL_2)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_3(self) -> &'a mut W {
        self.variant(BL_A::BL_3)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_4(self) -> &'a mut W {
        self.variant(BL_A::BL_4)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_5(self) -> &'a mut W {
        self.variant(BL_A::BL_5)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_6(self) -> &'a mut W {
        self.variant(BL_A::BL_6)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_7(self) -> &'a mut W {
        self.variant(BL_A::BL_7)
    }
}
#[doc = "Field `COL` reader - Column address bit number"]
pub type COL_R = crate::FieldReader<u8, COL_A>;
#[doc = "Column address bit number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COL_A {
    #[doc = "0: 12"]
    COL_0 = 0,
    #[doc = "1: 11"]
    COL_1 = 1,
    #[doc = "2: 10"]
    COL_2 = 2,
    #[doc = "3: 9"]
    COL_3 = 3,
}
impl From<COL_A> for u8 {
    #[inline(always)]
    fn from(variant: COL_A) -> Self {
        variant as _
    }
}
impl COL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_A {
        match self.bits {
            0 => COL_A::COL_0,
            1 => COL_A::COL_1,
            2 => COL_A::COL_2,
            3 => COL_A::COL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COL_0`"]
    #[inline(always)]
    pub fn is_col_0(&self) -> bool {
        *self == COL_A::COL_0
    }
    #[doc = "Checks if the value of the field is `COL_1`"]
    #[inline(always)]
    pub fn is_col_1(&self) -> bool {
        *self == COL_A::COL_1
    }
    #[doc = "Checks if the value of the field is `COL_2`"]
    #[inline(always)]
    pub fn is_col_2(&self) -> bool {
        *self == COL_A::COL_2
    }
    #[doc = "Checks if the value of the field is `COL_3`"]
    #[inline(always)]
    pub fn is_col_3(&self) -> bool {
        *self == COL_A::COL_3
    }
}
#[doc = "Field `COL` writer - Column address bit number"]
pub type COL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDRAMCR0_SPEC, u8, COL_A, 2, O>;
impl<'a, const O: u8> COL_W<'a, O> {
    #[doc = "12"]
    #[inline(always)]
    pub fn col_0(self) -> &'a mut W {
        self.variant(COL_A::COL_0)
    }
    #[doc = "11"]
    #[inline(always)]
    pub fn col_1(self) -> &'a mut W {
        self.variant(COL_A::COL_1)
    }
    #[doc = "10"]
    #[inline(always)]
    pub fn col_2(self) -> &'a mut W {
        self.variant(COL_A::COL_2)
    }
    #[doc = "9"]
    #[inline(always)]
    pub fn col_3(self) -> &'a mut W {
        self.variant(COL_A::COL_3)
    }
}
#[doc = "Field `CL` reader - CAS Latency"]
pub type CL_R = crate::FieldReader<u8, CL_A>;
#[doc = "CAS Latency\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CL_A {
    #[doc = "0: 1"]
    CL_0 = 0,
    #[doc = "1: 1"]
    CL_1 = 1,
    #[doc = "2: 2"]
    CL_2 = 2,
    #[doc = "3: 3"]
    CL_3 = 3,
}
impl From<CL_A> for u8 {
    #[inline(always)]
    fn from(variant: CL_A) -> Self {
        variant as _
    }
}
impl CL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CL_A {
        match self.bits {
            0 => CL_A::CL_0,
            1 => CL_A::CL_1,
            2 => CL_A::CL_2,
            3 => CL_A::CL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CL_0`"]
    #[inline(always)]
    pub fn is_cl_0(&self) -> bool {
        *self == CL_A::CL_0
    }
    #[doc = "Checks if the value of the field is `CL_1`"]
    #[inline(always)]
    pub fn is_cl_1(&self) -> bool {
        *self == CL_A::CL_1
    }
    #[doc = "Checks if the value of the field is `CL_2`"]
    #[inline(always)]
    pub fn is_cl_2(&self) -> bool {
        *self == CL_A::CL_2
    }
    #[doc = "Checks if the value of the field is `CL_3`"]
    #[inline(always)]
    pub fn is_cl_3(&self) -> bool {
        *self == CL_A::CL_3
    }
}
#[doc = "Field `CL` writer - CAS Latency"]
pub type CL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDRAMCR0_SPEC, u8, CL_A, 2, O>;
impl<'a, const O: u8> CL_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn cl_0(self) -> &'a mut W {
        self.variant(CL_A::CL_0)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn cl_1(self) -> &'a mut W {
        self.variant(CL_A::CL_1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn cl_2(self) -> &'a mut W {
        self.variant(CL_A::CL_2)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn cl_3(self) -> &'a mut W {
        self.variant(CL_A::CL_3)
    }
}
impl R {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Column address bit number"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CAS Latency"]
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<0> {
        PS_W::new(self)
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<4> {
        BL_W::new(self)
    }
    #[doc = "Bits 8:9 - Column address bit number"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<8> {
        COL_W::new(self)
    }
    #[doc = "Bits 10:11 - CAS Latency"]
    #[inline(always)]
    #[must_use]
    pub fn cl(&mut self) -> CL_W<10> {
        CL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr0](index.html) module"]
pub struct SDRAMCR0_SPEC;
impl crate::RegisterSpec for SDRAMCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramcr0::R](R) reader structure"]
impl crate::Readable for SDRAMCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdramcr0::W](W) writer structure"]
impl crate::Writable for SDRAMCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRAMCR0 to value 0x0c26"]
impl crate::Resettable for SDRAMCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c26;
}
