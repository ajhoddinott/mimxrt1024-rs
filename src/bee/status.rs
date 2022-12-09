#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_VEC` reader - bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
pub type IRQ_VEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQ_VEC` writer - bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
pub type IRQ_VEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 8, O>;
#[doc = "Field `BEE_IDLE` reader - 1'b1: BEE is idle; 1'b0: BEE is active"]
pub type BEE_IDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
    #[inline(always)]
    pub fn irq_vec(&self) -> IRQ_VEC_R {
        IRQ_VEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1'b1: BEE is idle; 1'b0: BEE is active"]
    #[inline(always)]
    pub fn bee_idle(&self) -> BEE_IDLE_R {
        BEE_IDLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
    #[inline(always)]
    #[must_use]
    pub fn irq_vec(&mut self) -> IRQ_VEC_W<0> {
        IRQ_VEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
