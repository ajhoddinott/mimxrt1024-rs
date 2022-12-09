#[doc = "Register `LUT[%s]` reader"]
pub struct R(crate::R<LUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT[%s]` writer"]
pub struct W(crate::W<LUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_SPEC>;
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
impl From<crate::W<LUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPERAND0` reader - OPERAND0"]
pub type OPERAND0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPERAND0` writer - OPERAND0"]
pub type OPERAND0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `NUM_PADS0` reader - NUM_PADS0"]
pub type NUM_PADS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_PADS0` writer - NUM_PADS0"]
pub type NUM_PADS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OPCODE0` reader - OPCODE"]
pub type OPCODE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCODE0` writer - OPCODE"]
pub type OPCODE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u8, u8, 6, O>;
#[doc = "Field `OPERAND1` reader - OPERAND1"]
pub type OPERAND1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPERAND1` writer - OPERAND1"]
pub type OPERAND1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `NUM_PADS1` reader - NUM_PADS1"]
pub type NUM_PADS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_PADS1` writer - NUM_PADS1"]
pub type NUM_PADS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OPCODE1` reader - OPCODE1"]
pub type OPCODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCODE1` writer - OPCODE1"]
pub type OPCODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline(always)]
    pub fn operand0(&self) -> OPERAND0_R {
        OPERAND0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline(always)]
    pub fn num_pads0(&self) -> NUM_PADS0_R {
        NUM_PADS0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline(always)]
    pub fn opcode0(&self) -> OPCODE0_R {
        OPCODE0_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline(always)]
    pub fn operand1(&self) -> OPERAND1_R {
        OPERAND1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline(always)]
    pub fn num_pads1(&self) -> NUM_PADS1_R {
        NUM_PADS1_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline(always)]
    pub fn opcode1(&self) -> OPCODE1_R {
        OPCODE1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline(always)]
    #[must_use]
    pub fn operand0(&mut self) -> OPERAND0_W<0> {
        OPERAND0_W::new(self)
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline(always)]
    #[must_use]
    pub fn num_pads0(&mut self) -> NUM_PADS0_W<8> {
        NUM_PADS0_W::new(self)
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0(&mut self) -> OPCODE0_W<10> {
        OPCODE0_W::new(self)
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline(always)]
    #[must_use]
    pub fn operand1(&mut self) -> OPERAND1_W<16> {
        OPERAND1_W::new(self)
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline(always)]
    #[must_use]
    pub fn num_pads1(&mut self) -> NUM_PADS1_W<24> {
        NUM_PADS1_W::new(self)
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline(always)]
    #[must_use]
    pub fn opcode1(&mut self) -> OPCODE1_W<26> {
        OPCODE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut](index.html) module"]
pub struct LUT_SPEC;
impl crate::RegisterSpec for LUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut::R](R) reader structure"]
impl crate::Readable for LUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut::W](W) writer structure"]
impl crate::Writable for LUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT[%s]
to value 0"]
impl crate::Resettable for LUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
