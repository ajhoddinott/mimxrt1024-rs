#[doc = "Register `PAGETABLE` reader"]
pub struct R(crate::R<PAGETABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAGETABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAGETABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAGETABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAGETABLE` writer"]
pub struct W(crate::W<PAGETABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAGETABLE_SPEC>;
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
impl From<crate::W<PAGETABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAGETABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Page table enable control"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Page table enable control"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAGETABLE_SPEC, bool, O>;
#[doc = "Field `FLUSH` reader - Page table flush control. To flush the TLB, write this bit to 1 and then back to 0."]
pub type FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH` writer - Page table flush control. To flush the TLB, write this bit to 1 and then back to 0."]
pub type FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAGETABLE_SPEC, bool, O>;
#[doc = "Field `BASE` reader - Page table base address"]
pub type BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASE` writer - Page table base address"]
pub type BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAGETABLE_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - Page table enable control"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page table flush control. To flush the TLB, write this bit to 1 and then back to 0."]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Page table base address"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Page table enable control"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Page table flush control. To flush the TLB, write this bit to 1 and then back to 0."]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<1> {
        FLUSH_W::new(self)
    }
    #[doc = "Bits 2:31 - Page table base address"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<2> {
        BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP page table register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pagetable](index.html) module"]
pub struct PAGETABLE_SPEC;
impl crate::RegisterSpec for PAGETABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pagetable::R](R) reader structure"]
impl crate::Readable for PAGETABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pagetable::W](W) writer structure"]
impl crate::Writable for PAGETABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAGETABLE to value 0"]
impl crate::Resettable for PAGETABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
