#[doc = "Register `DATA_BUFF_ACC_PORT` reader"]
pub struct R(crate::R<DATA_BUFF_ACC_PORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_BUFF_ACC_PORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_BUFF_ACC_PORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_BUFF_ACC_PORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_BUFF_ACC_PORT` writer"]
pub struct W(crate::W<DATA_BUFF_ACC_PORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_BUFF_ACC_PORT_SPEC>;
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
impl From<crate::W<DATA_BUFF_ACC_PORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_BUFF_ACC_PORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATCONT` reader - Data content"]
pub type DATCONT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATCONT` writer - Data content"]
pub type DATCONT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_BUFF_ACC_PORT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data content"]
    #[inline(always)]
    pub fn datcont(&self) -> DATCONT_R {
        DATCONT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data content"]
    #[inline(always)]
    #[must_use]
    pub fn datcont(&mut self) -> DATCONT_W<0> {
        DATCONT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Buffer Access Port\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_buff_acc_port](index.html) module"]
pub struct DATA_BUFF_ACC_PORT_SPEC;
impl crate::RegisterSpec for DATA_BUFF_ACC_PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_buff_acc_port::R](R) reader structure"]
impl crate::Readable for DATA_BUFF_ACC_PORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_buff_acc_port::W](W) writer structure"]
impl crate::Writable for DATA_BUFF_ACC_PORT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_BUFF_ACC_PORT to value 0"]
impl crate::Resettable for DATA_BUFF_ACC_PORT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
