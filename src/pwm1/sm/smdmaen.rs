#[doc = "Register `SMDMAEN` reader"]
pub struct R(crate::R<SMDMAEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMDMAEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMDMAEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMDMAEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMDMAEN` writer"]
pub struct W(crate::W<SMDMAEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMDMAEN_SPEC>;
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
impl From<crate::W<SMDMAEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMDMAEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CX0DE` reader - Capture X0 FIFO DMA Enable"]
pub type CX0DE_R = crate::BitReader<bool>;
#[doc = "Field `CX0DE` writer - Capture X0 FIFO DMA Enable"]
pub type CX0DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, bool, O>;
#[doc = "Field `CX1DE` reader - Capture X1 FIFO DMA Enable"]
pub type CX1DE_R = crate::BitReader<bool>;
#[doc = "Field `CX1DE` writer - Capture X1 FIFO DMA Enable"]
pub type CX1DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, bool, O>;
#[doc = "Field `CB0DE` reader - Capture B0 FIFO DMA Enable"]
pub type CB0DE_R = crate::BitReader<bool>;
#[doc = "Field `CB0DE` writer - Capture B0 FIFO DMA Enable"]
pub type CB0DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, bool, O>;
#[doc = "Field `CB1DE` reader - Capture B1 FIFO DMA Enable"]
pub type CB1DE_R = crate::BitReader<bool>;
#[doc = "Field `CB1DE` writer - Capture B1 FIFO DMA Enable"]
pub type CB1DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, bool, O>;
#[doc = "Field `CA0DE` reader - Capture A0 FIFO DMA Enable"]
pub type CA0DE_R = crate::BitReader<bool>;
#[doc = "Field `CA0DE` writer - Capture A0 FIFO DMA Enable"]
pub type CA0DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, bool, O>;
#[doc = "Field `CA1DE` reader - Capture A1 FIFO DMA Enable"]
pub type CA1DE_R = crate::BitReader<bool>;
#[doc = "Field `CA1DE` writer - Capture A1 FIFO DMA Enable"]
pub type CA1DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, bool, O>;
#[doc = "Field `CAPTDE` reader - Capture DMA Enable Source Select"]
pub type CAPTDE_R = crate::FieldReader<u8, CAPTDE_A>;
#[doc = "Capture DMA Enable Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTDE_A {
    #[doc = "0: Read DMA requests disabled."]
    DISABLED = 0,
    #[doc = "1: Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\]
to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 1,
    #[doc = "2: A local sync (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 2,
    #[doc = "3: A local reload (STS\\[RF\\]
being set) sets the read DMA request."]
    LOCAL_RELOAD = 3,
}
impl From<CAPTDE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTDE_A) -> Self {
        variant as _
    }
}
impl CAPTDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTDE_A {
        match self.bits {
            0 => CAPTDE_A::DISABLED,
            1 => CAPTDE_A::EXCEEDFIFO,
            2 => CAPTDE_A::LOCAL_SYNC,
            3 => CAPTDE_A::LOCAL_RELOAD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPTDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EXCEEDFIFO`"]
    #[inline(always)]
    pub fn is_exceedfifo(&self) -> bool {
        *self == CAPTDE_A::EXCEEDFIFO
    }
    #[doc = "Checks if the value of the field is `LOCAL_SYNC`"]
    #[inline(always)]
    pub fn is_local_sync(&self) -> bool {
        *self == CAPTDE_A::LOCAL_SYNC
    }
    #[doc = "Checks if the value of the field is `LOCAL_RELOAD`"]
    #[inline(always)]
    pub fn is_local_reload(&self) -> bool {
        *self == CAPTDE_A::LOCAL_RELOAD
    }
}
#[doc = "Field `CAPTDE` writer - Capture DMA Enable Source Select"]
pub type CAPTDE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMDMAEN_SPEC, u8, CAPTDE_A, 2, O>;
impl<'a, const O: u8> CAPTDE_W<'a, O> {
    #[doc = "Read DMA requests disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPTDE_A::DISABLED)
    }
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\]
to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    #[inline(always)]
    pub fn exceedfifo(self) -> &'a mut W {
        self.variant(CAPTDE_A::EXCEEDFIFO)
    }
    #[doc = "A local sync (VAL1 matches counter) sets the read DMA request."]
    #[inline(always)]
    pub fn local_sync(self) -> &'a mut W {
        self.variant(CAPTDE_A::LOCAL_SYNC)
    }
    #[doc = "A local reload (STS\\[RF\\]
being set) sets the read DMA request."]
    #[inline(always)]
    pub fn local_reload(self) -> &'a mut W {
        self.variant(CAPTDE_A::LOCAL_RELOAD)
    }
}
#[doc = "Field `FAND` reader - FIFO Watermark AND Control"]
pub type FAND_R = crate::BitReader<FAND_A>;
#[doc = "FIFO Watermark AND Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAND_A {
    #[doc = "0: Selected FIFO watermarks are OR'ed together."]
    OR = 0,
    #[doc = "1: Selected FIFO watermarks are AND'ed together."]
    AND = 1,
}
impl From<FAND_A> for bool {
    #[inline(always)]
    fn from(variant: FAND_A) -> Self {
        variant as u8 != 0
    }
}
impl FAND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAND_A {
        match self.bits {
            false => FAND_A::OR,
            true => FAND_A::AND,
        }
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == FAND_A::OR
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == FAND_A::AND
    }
}
#[doc = "Field `FAND` writer - FIFO Watermark AND Control"]
pub type FAND_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, FAND_A, O>;
impl<'a, const O: u8> FAND_W<'a, O> {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    #[inline(always)]
    pub fn or(self) -> &'a mut W {
        self.variant(FAND_A::OR)
    }
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    #[inline(always)]
    pub fn and(self) -> &'a mut W {
        self.variant(FAND_A::AND)
    }
}
#[doc = "Field `VALDE` reader - Value Registers DMA Enable"]
pub type VALDE_R = crate::BitReader<VALDE_A>;
#[doc = "Value Registers DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALDE_A {
    #[doc = "0: DMA write requests disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<VALDE_A> for bool {
    #[inline(always)]
    fn from(variant: VALDE_A) -> Self {
        variant as u8 != 0
    }
}
impl VALDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALDE_A {
        match self.bits {
            false => VALDE_A::DISABLED,
            true => VALDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VALDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VALDE_A::ENABLED
    }
}
#[doc = "Field `VALDE` writer - Value Registers DMA Enable"]
pub type VALDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMDMAEN_SPEC, VALDE_A, O>;
impl<'a, const O: u8> VALDE_W<'a, O> {
    #[doc = "DMA write requests disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VALDE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VALDE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cx0de(&self) -> CX0DE_R {
        CX0DE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cx1de(&self) -> CX1DE_R {
        CX1DE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cb0de(&self) -> CB0DE_R {
        CB0DE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cb1de(&self) -> CB1DE_R {
        CB1DE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn ca0de(&self) -> CA0DE_R {
        CA0DE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn ca1de(&self) -> CA1DE_R {
        CA1DE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Capture DMA Enable Source Select"]
    #[inline(always)]
    pub fn captde(&self) -> CAPTDE_R {
        CAPTDE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - FIFO Watermark AND Control"]
    #[inline(always)]
    pub fn fand(&self) -> FAND_R {
        FAND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Value Registers DMA Enable"]
    #[inline(always)]
    pub fn valde(&self) -> VALDE_R {
        VALDE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cx0de(&mut self) -> CX0DE_W<0> {
        CX0DE_W::new(self)
    }
    #[doc = "Bit 1 - Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cx1de(&mut self) -> CX1DE_W<1> {
        CX1DE_W::new(self)
    }
    #[doc = "Bit 2 - Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cb0de(&mut self) -> CB0DE_W<2> {
        CB0DE_W::new(self)
    }
    #[doc = "Bit 3 - Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cb1de(&mut self) -> CB1DE_W<3> {
        CB1DE_W::new(self)
    }
    #[doc = "Bit 4 - Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ca0de(&mut self) -> CA0DE_W<4> {
        CA0DE_W::new(self)
    }
    #[doc = "Bit 5 - Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ca1de(&mut self) -> CA1DE_W<5> {
        CA1DE_W::new(self)
    }
    #[doc = "Bits 6:7 - Capture DMA Enable Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn captde(&mut self) -> CAPTDE_W<6> {
        CAPTDE_W::new(self)
    }
    #[doc = "Bit 8 - FIFO Watermark AND Control"]
    #[inline(always)]
    #[must_use]
    pub fn fand(&mut self) -> FAND_W<8> {
        FAND_W::new(self)
    }
    #[doc = "Bit 9 - Value Registers DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn valde(&mut self) -> VALDE_W<9> {
        VALDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdmaen](index.html) module"]
pub struct SMDMAEN_SPEC;
impl crate::RegisterSpec for SMDMAEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smdmaen::R](R) reader structure"]
impl crate::Readable for SMDMAEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smdmaen::W](W) writer structure"]
impl crate::Writable for SMDMAEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMDMAEN to value 0"]
impl crate::Resettable for SMDMAEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
