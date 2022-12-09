#[doc = "Register `FORCE_EVENT` reader"]
pub struct R(crate::R<FORCE_EVENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORCE_EVENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORCE_EVENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORCE_EVENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FORCE_EVENT` writer"]
pub struct W(crate::W<FORCE_EVENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCE_EVENT_SPEC>;
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
impl From<crate::W<FORCE_EVENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCE_EVENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEVTAC12NE` reader - Force event auto command 12 not executed"]
pub type FEVTAC12NE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTAC12NE` writer - Force event auto command 12 not executed"]
pub type FEVTAC12NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTAC12TOE` reader - Force event auto command 12 time out error"]
pub type FEVTAC12TOE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTAC12TOE` writer - Force event auto command 12 time out error"]
pub type FEVTAC12TOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTAC12CE` reader - Force event auto command 12 CRC error"]
pub type FEVTAC12CE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTAC12CE` writer - Force event auto command 12 CRC error"]
pub type FEVTAC12CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTAC12EBE` reader - Force event Auto Command 12 end bit error"]
pub type FEVTAC12EBE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTAC12EBE` writer - Force event Auto Command 12 end bit error"]
pub type FEVTAC12EBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTAC12IE` reader - Force event Auto Command 12 index error"]
pub type FEVTAC12IE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTAC12IE` writer - Force event Auto Command 12 index error"]
pub type FEVTAC12IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTCNIBAC12E` reader - Force event command not executed by Auto Command 12 error"]
pub type FEVTCNIBAC12E_R = crate::BitReader<bool>;
#[doc = "Field `FEVTCNIBAC12E` writer - Force event command not executed by Auto Command 12 error"]
pub type FEVTCNIBAC12E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTCTOE` reader - Force event command time out error"]
pub type FEVTCTOE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTCTOE` writer - Force event command time out error"]
pub type FEVTCTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTCCE` reader - Force event command CRC error"]
pub type FEVTCCE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTCCE` writer - Force event command CRC error"]
pub type FEVTCCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTCEBE` reader - Force event command end bit error"]
pub type FEVTCEBE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTCEBE` writer - Force event command end bit error"]
pub type FEVTCEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTCIE` reader - Force event command index error"]
pub type FEVTCIE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTCIE` writer - Force event command index error"]
pub type FEVTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTDTOE` reader - Force event data time out error"]
pub type FEVTDTOE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTDTOE` writer - Force event data time out error"]
pub type FEVTDTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTDCE` reader - Force event data CRC error"]
pub type FEVTDCE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTDCE` writer - Force event data CRC error"]
pub type FEVTDCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTDEBE` reader - Force event data end bit error"]
pub type FEVTDEBE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTDEBE` writer - Force event data end bit error"]
pub type FEVTDEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTAC12E` reader - Force event Auto Command 12 error"]
pub type FEVTAC12E_R = crate::BitReader<bool>;
#[doc = "Field `FEVTAC12E` writer - Force event Auto Command 12 error"]
pub type FEVTAC12E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTTNE` reader - Force tuning error"]
pub type FEVTTNE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTTNE` writer - Force tuning error"]
pub type FEVTTNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTDMAE` reader - Force event DMA error"]
pub type FEVTDMAE_R = crate::BitReader<bool>;
#[doc = "Field `FEVTDMAE` writer - Force event DMA error"]
pub type FEVTDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FEVTCINT` reader - Force event card interrupt"]
pub type FEVTCINT_R = crate::BitReader<bool>;
#[doc = "Field `FEVTCINT` writer - Force event card interrupt"]
pub type FEVTCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Force event auto command 12 not executed"]
    #[inline(always)]
    pub fn fevtac12ne(&self) -> FEVTAC12NE_R {
        FEVTAC12NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force event auto command 12 time out error"]
    #[inline(always)]
    pub fn fevtac12toe(&self) -> FEVTAC12TOE_R {
        FEVTAC12TOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force event auto command 12 CRC error"]
    #[inline(always)]
    pub fn fevtac12ce(&self) -> FEVTAC12CE_R {
        FEVTAC12CE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force event Auto Command 12 end bit error"]
    #[inline(always)]
    pub fn fevtac12ebe(&self) -> FEVTAC12EBE_R {
        FEVTAC12EBE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force event Auto Command 12 index error"]
    #[inline(always)]
    pub fn fevtac12ie(&self) -> FEVTAC12IE_R {
        FEVTAC12IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Force event command not executed by Auto Command 12 error"]
    #[inline(always)]
    pub fn fevtcnibac12e(&self) -> FEVTCNIBAC12E_R {
        FEVTCNIBAC12E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Force event command time out error"]
    #[inline(always)]
    pub fn fevtctoe(&self) -> FEVTCTOE_R {
        FEVTCTOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force event command CRC error"]
    #[inline(always)]
    pub fn fevtcce(&self) -> FEVTCCE_R {
        FEVTCCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force event command end bit error"]
    #[inline(always)]
    pub fn fevtcebe(&self) -> FEVTCEBE_R {
        FEVTCEBE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force event command index error"]
    #[inline(always)]
    pub fn fevtcie(&self) -> FEVTCIE_R {
        FEVTCIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force event data time out error"]
    #[inline(always)]
    pub fn fevtdtoe(&self) -> FEVTDTOE_R {
        FEVTDTOE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Force event data CRC error"]
    #[inline(always)]
    pub fn fevtdce(&self) -> FEVTDCE_R {
        FEVTDCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Force event data end bit error"]
    #[inline(always)]
    pub fn fevtdebe(&self) -> FEVTDEBE_R {
        FEVTDEBE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Force event Auto Command 12 error"]
    #[inline(always)]
    pub fn fevtac12e(&self) -> FEVTAC12E_R {
        FEVTAC12E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Force tuning error"]
    #[inline(always)]
    pub fn fevttne(&self) -> FEVTTNE_R {
        FEVTTNE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Force event DMA error"]
    #[inline(always)]
    pub fn fevtdmae(&self) -> FEVTDMAE_R {
        FEVTDMAE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Force event card interrupt"]
    #[inline(always)]
    pub fn fevtcint(&self) -> FEVTCINT_R {
        FEVTCINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force event auto command 12 not executed"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ne(&mut self) -> FEVTAC12NE_W<0> {
        FEVTAC12NE_W::new(self)
    }
    #[doc = "Bit 1 - Force event auto command 12 time out error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12toe(&mut self) -> FEVTAC12TOE_W<1> {
        FEVTAC12TOE_W::new(self)
    }
    #[doc = "Bit 2 - Force event auto command 12 CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ce(&mut self) -> FEVTAC12CE_W<2> {
        FEVTAC12CE_W::new(self)
    }
    #[doc = "Bit 3 - Force event Auto Command 12 end bit error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ebe(&mut self) -> FEVTAC12EBE_W<3> {
        FEVTAC12EBE_W::new(self)
    }
    #[doc = "Bit 4 - Force event Auto Command 12 index error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ie(&mut self) -> FEVTAC12IE_W<4> {
        FEVTAC12IE_W::new(self)
    }
    #[doc = "Bit 7 - Force event command not executed by Auto Command 12 error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcnibac12e(&mut self) -> FEVTCNIBAC12E_W<7> {
        FEVTCNIBAC12E_W::new(self)
    }
    #[doc = "Bit 16 - Force event command time out error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtctoe(&mut self) -> FEVTCTOE_W<16> {
        FEVTCTOE_W::new(self)
    }
    #[doc = "Bit 17 - Force event command CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcce(&mut self) -> FEVTCCE_W<17> {
        FEVTCCE_W::new(self)
    }
    #[doc = "Bit 18 - Force event command end bit error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcebe(&mut self) -> FEVTCEBE_W<18> {
        FEVTCEBE_W::new(self)
    }
    #[doc = "Bit 19 - Force event command index error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcie(&mut self) -> FEVTCIE_W<19> {
        FEVTCIE_W::new(self)
    }
    #[doc = "Bit 20 - Force event data time out error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdtoe(&mut self) -> FEVTDTOE_W<20> {
        FEVTDTOE_W::new(self)
    }
    #[doc = "Bit 21 - Force event data CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdce(&mut self) -> FEVTDCE_W<21> {
        FEVTDCE_W::new(self)
    }
    #[doc = "Bit 22 - Force event data end bit error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdebe(&mut self) -> FEVTDEBE_W<22> {
        FEVTDEBE_W::new(self)
    }
    #[doc = "Bit 24 - Force event Auto Command 12 error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12e(&mut self) -> FEVTAC12E_W<24> {
        FEVTAC12E_W::new(self)
    }
    #[doc = "Bit 26 - Force tuning error"]
    #[inline(always)]
    #[must_use]
    pub fn fevttne(&mut self) -> FEVTTNE_W<26> {
        FEVTTNE_W::new(self)
    }
    #[doc = "Bit 28 - Force event DMA error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdmae(&mut self) -> FEVTDMAE_W<28> {
        FEVTDMAE_W::new(self)
    }
    #[doc = "Bit 31 - Force event card interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcint(&mut self) -> FEVTCINT_W<31> {
        FEVTCINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_event](index.html) module"]
pub struct FORCE_EVENT_SPEC;
impl crate::RegisterSpec for FORCE_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [force_event::R](R) reader structure"]
impl crate::Readable for FORCE_EVENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [force_event::W](W) writer structure"]
impl crate::Writable for FORCE_EVENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORCE_EVENT to value 0"]
impl crate::Resettable for FORCE_EVENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
