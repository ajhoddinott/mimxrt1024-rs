#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEN0` reader - DMA Enable for XBAR_OUT0"]
pub type DEN0_R = crate::BitReader<DEN0_A>;
#[doc = "DMA Enable for XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN0_A {
    #[doc = "0: DMA disabled"]
    DEN0_0 = 0,
    #[doc = "1: DMA enabled"]
    DEN0_1 = 1,
}
impl From<DEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN0_A {
        match self.bits {
            false => DEN0_A::DEN0_0,
            true => DEN0_A::DEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN0_0`"]
    #[inline(always)]
    pub fn is_den0_0(&self) -> bool {
        *self == DEN0_A::DEN0_0
    }
    #[doc = "Checks if the value of the field is `DEN0_1`"]
    #[inline(always)]
    pub fn is_den0_1(&self) -> bool {
        *self == DEN0_A::DEN0_1
    }
}
#[doc = "Field `DEN0` writer - DMA Enable for XBAR_OUT0"]
pub type DEN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, DEN0_A, O>;
impl<'a, const O: u8> DEN0_W<'a, O> {
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn den0_0(self) -> &'a mut W {
        self.variant(DEN0_A::DEN0_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn den0_1(self) -> &'a mut W {
        self.variant(DEN0_A::DEN0_1)
    }
}
#[doc = "Field `IEN0` reader - Interrupt Enable for XBAR_OUT0"]
pub type IEN0_R = crate::BitReader<IEN0_A>;
#[doc = "Interrupt Enable for XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN0_A {
    #[doc = "0: Interrupt disabled"]
    IEN0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    IEN0_1 = 1,
}
impl From<IEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN0_A {
        match self.bits {
            false => IEN0_A::IEN0_0,
            true => IEN0_A::IEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN0_0`"]
    #[inline(always)]
    pub fn is_ien0_0(&self) -> bool {
        *self == IEN0_A::IEN0_0
    }
    #[doc = "Checks if the value of the field is `IEN0_1`"]
    #[inline(always)]
    pub fn is_ien0_1(&self) -> bool {
        *self == IEN0_A::IEN0_1
    }
}
#[doc = "Field `IEN0` writer - Interrupt Enable for XBAR_OUT0"]
pub type IEN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, IEN0_A, O>;
impl<'a, const O: u8> IEN0_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ien0_0(self) -> &'a mut W {
        self.variant(IEN0_A::IEN0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ien0_1(self) -> &'a mut W {
        self.variant(IEN0_A::IEN0_1)
    }
}
#[doc = "Field `EDGE0` reader - Active edge for edge detection on XBAR_OUT0"]
pub type EDGE0_R = crate::FieldReader<u8, EDGE0_A>;
#[doc = "Active edge for edge detection on XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGE0_A {
    #[doc = "0: STS0 never asserts"]
    EDGE0_0 = 0,
    #[doc = "1: STS0 asserts on rising edges of XBAR_OUT0"]
    EDGE0_1 = 1,
    #[doc = "2: STS0 asserts on falling edges of XBAR_OUT0"]
    EDGE0_2 = 2,
    #[doc = "3: STS0 asserts on rising and falling edges of XBAR_OUT0"]
    EDGE0_3 = 3,
}
impl From<EDGE0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE0_A) -> Self {
        variant as _
    }
}
impl EDGE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE0_A {
        match self.bits {
            0 => EDGE0_A::EDGE0_0,
            1 => EDGE0_A::EDGE0_1,
            2 => EDGE0_A::EDGE0_2,
            3 => EDGE0_A::EDGE0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE0_0`"]
    #[inline(always)]
    pub fn is_edge0_0(&self) -> bool {
        *self == EDGE0_A::EDGE0_0
    }
    #[doc = "Checks if the value of the field is `EDGE0_1`"]
    #[inline(always)]
    pub fn is_edge0_1(&self) -> bool {
        *self == EDGE0_A::EDGE0_1
    }
    #[doc = "Checks if the value of the field is `EDGE0_2`"]
    #[inline(always)]
    pub fn is_edge0_2(&self) -> bool {
        *self == EDGE0_A::EDGE0_2
    }
    #[doc = "Checks if the value of the field is `EDGE0_3`"]
    #[inline(always)]
    pub fn is_edge0_3(&self) -> bool {
        *self == EDGE0_A::EDGE0_3
    }
}
#[doc = "Field `EDGE0` writer - Active edge for edge detection on XBAR_OUT0"]
pub type EDGE0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CTRL0_SPEC, u8, EDGE0_A, 2, O>;
impl<'a, const O: u8> EDGE0_W<'a, O> {
    #[doc = "STS0 never asserts"]
    #[inline(always)]
    pub fn edge0_0(self) -> &'a mut W {
        self.variant(EDGE0_A::EDGE0_0)
    }
    #[doc = "STS0 asserts on rising edges of XBAR_OUT0"]
    #[inline(always)]
    pub fn edge0_1(self) -> &'a mut W {
        self.variant(EDGE0_A::EDGE0_1)
    }
    #[doc = "STS0 asserts on falling edges of XBAR_OUT0"]
    #[inline(always)]
    pub fn edge0_2(self) -> &'a mut W {
        self.variant(EDGE0_A::EDGE0_2)
    }
    #[doc = "STS0 asserts on rising and falling edges of XBAR_OUT0"]
    #[inline(always)]
    pub fn edge0_3(self) -> &'a mut W {
        self.variant(EDGE0_A::EDGE0_3)
    }
}
#[doc = "Field `STS0` reader - Edge detection status for XBAR_OUT0"]
pub type STS0_R = crate::BitReader<STS0_A>;
#[doc = "Edge detection status for XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STS0_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT0"]
    STS0_0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT0"]
    STS0_1 = 1,
}
impl From<STS0_A> for bool {
    #[inline(always)]
    fn from(variant: STS0_A) -> Self {
        variant as u8 != 0
    }
}
impl STS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS0_A {
        match self.bits {
            false => STS0_A::STS0_0,
            true => STS0_A::STS0_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS0_0`"]
    #[inline(always)]
    pub fn is_sts0_0(&self) -> bool {
        *self == STS0_A::STS0_0
    }
    #[doc = "Checks if the value of the field is `STS0_1`"]
    #[inline(always)]
    pub fn is_sts0_1(&self) -> bool {
        *self == STS0_A::STS0_1
    }
}
#[doc = "Field `STS0` writer - Edge detection status for XBAR_OUT0"]
pub type STS0_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL0_SPEC, STS0_A, O>;
impl<'a, const O: u8> STS0_W<'a, O> {
    #[doc = "Active edge not yet detected on XBAR_OUT0"]
    #[inline(always)]
    pub fn sts0_0(self) -> &'a mut W {
        self.variant(STS0_A::STS0_0)
    }
    #[doc = "Active edge detected on XBAR_OUT0"]
    #[inline(always)]
    pub fn sts0_1(self) -> &'a mut W {
        self.variant(STS0_A::STS0_1)
    }
}
#[doc = "Field `DEN1` reader - DMA Enable for XBAR_OUT1"]
pub type DEN1_R = crate::BitReader<DEN1_A>;
#[doc = "DMA Enable for XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN1_A {
    #[doc = "0: DMA disabled"]
    DEN1_0 = 0,
    #[doc = "1: DMA enabled"]
    DEN1_1 = 1,
}
impl From<DEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl DEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN1_A {
        match self.bits {
            false => DEN1_A::DEN1_0,
            true => DEN1_A::DEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN1_0`"]
    #[inline(always)]
    pub fn is_den1_0(&self) -> bool {
        *self == DEN1_A::DEN1_0
    }
    #[doc = "Checks if the value of the field is `DEN1_1`"]
    #[inline(always)]
    pub fn is_den1_1(&self) -> bool {
        *self == DEN1_A::DEN1_1
    }
}
#[doc = "Field `DEN1` writer - DMA Enable for XBAR_OUT1"]
pub type DEN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, DEN1_A, O>;
impl<'a, const O: u8> DEN1_W<'a, O> {
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn den1_0(self) -> &'a mut W {
        self.variant(DEN1_A::DEN1_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn den1_1(self) -> &'a mut W {
        self.variant(DEN1_A::DEN1_1)
    }
}
#[doc = "Field `IEN1` reader - Interrupt Enable for XBAR_OUT1"]
pub type IEN1_R = crate::BitReader<IEN1_A>;
#[doc = "Interrupt Enable for XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN1_A {
    #[doc = "0: Interrupt disabled"]
    IEN1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    IEN1_1 = 1,
}
impl From<IEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN1_A {
        match self.bits {
            false => IEN1_A::IEN1_0,
            true => IEN1_A::IEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN1_0`"]
    #[inline(always)]
    pub fn is_ien1_0(&self) -> bool {
        *self == IEN1_A::IEN1_0
    }
    #[doc = "Checks if the value of the field is `IEN1_1`"]
    #[inline(always)]
    pub fn is_ien1_1(&self) -> bool {
        *self == IEN1_A::IEN1_1
    }
}
#[doc = "Field `IEN1` writer - Interrupt Enable for XBAR_OUT1"]
pub type IEN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, IEN1_A, O>;
impl<'a, const O: u8> IEN1_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ien1_0(self) -> &'a mut W {
        self.variant(IEN1_A::IEN1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ien1_1(self) -> &'a mut W {
        self.variant(IEN1_A::IEN1_1)
    }
}
#[doc = "Field `EDGE1` reader - Active edge for edge detection on XBAR_OUT1"]
pub type EDGE1_R = crate::FieldReader<u8, EDGE1_A>;
#[doc = "Active edge for edge detection on XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGE1_A {
    #[doc = "0: STS1 never asserts"]
    EDGE1_0 = 0,
    #[doc = "1: STS1 asserts on rising edges of XBAR_OUT1"]
    EDGE1_1 = 1,
    #[doc = "2: STS1 asserts on falling edges of XBAR_OUT1"]
    EDGE1_2 = 2,
    #[doc = "3: STS1 asserts on rising and falling edges of XBAR_OUT1"]
    EDGE1_3 = 3,
}
impl From<EDGE1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE1_A) -> Self {
        variant as _
    }
}
impl EDGE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE1_A {
        match self.bits {
            0 => EDGE1_A::EDGE1_0,
            1 => EDGE1_A::EDGE1_1,
            2 => EDGE1_A::EDGE1_2,
            3 => EDGE1_A::EDGE1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE1_0`"]
    #[inline(always)]
    pub fn is_edge1_0(&self) -> bool {
        *self == EDGE1_A::EDGE1_0
    }
    #[doc = "Checks if the value of the field is `EDGE1_1`"]
    #[inline(always)]
    pub fn is_edge1_1(&self) -> bool {
        *self == EDGE1_A::EDGE1_1
    }
    #[doc = "Checks if the value of the field is `EDGE1_2`"]
    #[inline(always)]
    pub fn is_edge1_2(&self) -> bool {
        *self == EDGE1_A::EDGE1_2
    }
    #[doc = "Checks if the value of the field is `EDGE1_3`"]
    #[inline(always)]
    pub fn is_edge1_3(&self) -> bool {
        *self == EDGE1_A::EDGE1_3
    }
}
#[doc = "Field `EDGE1` writer - Active edge for edge detection on XBAR_OUT1"]
pub type EDGE1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CTRL0_SPEC, u8, EDGE1_A, 2, O>;
impl<'a, const O: u8> EDGE1_W<'a, O> {
    #[doc = "STS1 never asserts"]
    #[inline(always)]
    pub fn edge1_0(self) -> &'a mut W {
        self.variant(EDGE1_A::EDGE1_0)
    }
    #[doc = "STS1 asserts on rising edges of XBAR_OUT1"]
    #[inline(always)]
    pub fn edge1_1(self) -> &'a mut W {
        self.variant(EDGE1_A::EDGE1_1)
    }
    #[doc = "STS1 asserts on falling edges of XBAR_OUT1"]
    #[inline(always)]
    pub fn edge1_2(self) -> &'a mut W {
        self.variant(EDGE1_A::EDGE1_2)
    }
    #[doc = "STS1 asserts on rising and falling edges of XBAR_OUT1"]
    #[inline(always)]
    pub fn edge1_3(self) -> &'a mut W {
        self.variant(EDGE1_A::EDGE1_3)
    }
}
#[doc = "Field `STS1` reader - Edge detection status for XBAR_OUT1"]
pub type STS1_R = crate::BitReader<STS1_A>;
#[doc = "Edge detection status for XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STS1_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT1"]
    STS1_0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT1"]
    STS1_1 = 1,
}
impl From<STS1_A> for bool {
    #[inline(always)]
    fn from(variant: STS1_A) -> Self {
        variant as u8 != 0
    }
}
impl STS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS1_A {
        match self.bits {
            false => STS1_A::STS1_0,
            true => STS1_A::STS1_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS1_0`"]
    #[inline(always)]
    pub fn is_sts1_0(&self) -> bool {
        *self == STS1_A::STS1_0
    }
    #[doc = "Checks if the value of the field is `STS1_1`"]
    #[inline(always)]
    pub fn is_sts1_1(&self) -> bool {
        *self == STS1_A::STS1_1
    }
}
#[doc = "Field `STS1` writer - Edge detection status for XBAR_OUT1"]
pub type STS1_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL0_SPEC, STS1_A, O>;
impl<'a, const O: u8> STS1_W<'a, O> {
    #[doc = "Active edge not yet detected on XBAR_OUT1"]
    #[inline(always)]
    pub fn sts1_0(self) -> &'a mut W {
        self.variant(STS1_A::STS1_0)
    }
    #[doc = "Active edge detected on XBAR_OUT1"]
    #[inline(always)]
    pub fn sts1_1(self) -> &'a mut W {
        self.variant(STS1_A::STS1_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT0"]
    #[inline(always)]
    pub fn den0(&self) -> DEN0_R {
        DEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT0"]
    #[inline(always)]
    pub fn ien0(&self) -> IEN0_R {
        IEN0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT0"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT0"]
    #[inline(always)]
    pub fn sts0(&self) -> STS0_R {
        STS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT1"]
    #[inline(always)]
    pub fn den1(&self) -> DEN1_R {
        DEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT1"]
    #[inline(always)]
    pub fn ien1(&self) -> IEN1_R {
        IEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT1"]
    #[inline(always)]
    pub fn sts1(&self) -> STS1_R {
        STS1_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn den0(&mut self) -> DEN0_W<0> {
        DEN0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn ien0(&mut self) -> IEN0_W<1> {
        IEN0_W::new(self)
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn edge0(&mut self) -> EDGE0_W<2> {
        EDGE0_W::new(self)
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn sts0(&mut self) -> STS0_W<4> {
        STS0_W::new(self)
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn den1(&mut self) -> DEN1_W<8> {
        DEN1_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn ien1(&mut self) -> IEN1_W<9> {
        IEN1_W::new(self)
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn edge1(&mut self) -> EDGE1_W<10> {
        EDGE1_W::new(self)
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn sts1(&mut self) -> STS1_W<12> {
        STS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1010;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
