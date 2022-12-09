#[doc = "Register `ENT[%s]` reader"]
pub struct R(crate::R<ENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENT` reader - Entropy Value"]
pub type ENT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Entropy Value"]
    #[inline(always)]
    pub fn ent(&self) -> ENT_R {
        ENT_R::new(self.bits)
    }
}
#[doc = "Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ent](index.html) module"]
pub struct ENT_SPEC;
impl crate::RegisterSpec for ENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ent::R](R) reader structure"]
impl crate::Readable for ENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENT[%s]
to value 0"]
impl crate::Resettable for ENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
