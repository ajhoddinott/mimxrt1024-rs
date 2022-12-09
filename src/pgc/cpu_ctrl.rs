#[doc = "Register `CPU_CTRL` reader"]
pub struct R(crate::R<CPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CTRL` writer"]
pub struct W(crate::W<CPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CTRL_SPEC>;
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
impl From<crate::W<CPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCR` reader - Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
pub type PCR_R = crate::BitReader<PCR_A>;
#[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCR_A {
    #[doc = "0: Do not switch off power even if pdn_req is asserted."]
    PCR_0 = 0,
    #[doc = "1: Switch off power when pdn_req is asserted."]
    PCR_1 = 1,
}
impl From<PCR_A> for bool {
    #[inline(always)]
    fn from(variant: PCR_A) -> Self {
        variant as u8 != 0
    }
}
impl PCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCR_A {
        match self.bits {
            false => PCR_A::PCR_0,
            true => PCR_A::PCR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCR_0`"]
    #[inline(always)]
    pub fn is_pcr_0(&self) -> bool {
        *self == PCR_A::PCR_0
    }
    #[doc = "Checks if the value of the field is `PCR_1`"]
    #[inline(always)]
    pub fn is_pcr_1(&self) -> bool {
        *self == PCR_A::PCR_1
    }
}
#[doc = "Field `PCR` writer - Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
pub type PCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CTRL_SPEC, PCR_A, O>;
impl<'a, const O: u8> PCR_W<'a, O> {
    #[doc = "Do not switch off power even if pdn_req is asserted."]
    #[inline(always)]
    pub fn pcr_0(self) -> &'a mut W {
        self.variant(PCR_A::PCR_0)
    }
    #[doc = "Switch off power when pdn_req is asserted."]
    #[inline(always)]
    pub fn pcr_1(self) -> &'a mut W {
        self.variant(PCR_A::PCR_1)
    }
}
impl R {
    #[doc = "Bit 0 - Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PCR_W<0> {
        PCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PGC CPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_ctrl](index.html) module"]
pub struct CPU_CTRL_SPEC;
impl crate::RegisterSpec for CPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_ctrl::R](R) reader structure"]
impl crate::Readable for CPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_ctrl::W](W) writer structure"]
impl crate::Writable for CPU_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_CTRL to value 0"]
impl crate::Resettable for CPU_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
