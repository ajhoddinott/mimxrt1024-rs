#[doc = "Register `MCR1` reader"]
pub struct R(crate::R<MCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR1` writer"]
pub struct W(crate::W<MCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR1_SPEC>;
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
impl From<crate::W<MCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBBUSWAIT` reader - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
pub type AHBBUSWAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AHBBUSWAIT` writer - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
pub type AHBBUSWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `SEQWAIT` reader - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
pub type SEQWAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEQWAIT` writer - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
pub type SEQWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    pub fn ahbbuswait(&self) -> AHBBUSWAIT_R {
        AHBBUSWAIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    pub fn seqwait(&self) -> SEQWAIT_R {
        SEQWAIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    #[must_use]
    pub fn ahbbuswait(&mut self) -> AHBBUSWAIT_W<0> {
        AHBBUSWAIT_W::new(self)
    }
    #[doc = "Bits 16:31 - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn seqwait(&mut self) -> SEQWAIT_W<16> {
        SEQWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr1](index.html) module"]
pub struct MCR1_SPEC;
impl crate::RegisterSpec for MCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr1::R](R) reader structure"]
impl crate::Readable for MCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr1::W](W) writer structure"]
impl crate::Writable for MCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR1 to value 0xffff_ffff"]
impl crate::Resettable for MCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
