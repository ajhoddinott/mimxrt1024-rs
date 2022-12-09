#[doc = "Register `CCGR0` reader"]
pub struct R(crate::R<CCGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCGR0` writer"]
pub struct W(crate::W<CCGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCGR0_SPEC>;
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
impl From<crate::W<CCGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CG0` reader - aips_tz1 clocks (aips_tz1_clk_enable)"]
pub type CG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG0` writer - aips_tz1 clocks (aips_tz1_clk_enable)"]
pub type CG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG1` reader - aips_tz2 clocks (aips_tz2_clk_enable)"]
pub type CG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG1` writer - aips_tz2 clocks (aips_tz2_clk_enable)"]
pub type CG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG2` reader - mqs clock ( mqs_hmclk_clock_enable)"]
pub type CG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG2` writer - mqs clock ( mqs_hmclk_clock_enable)"]
pub type CG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG3` reader - Reserved"]
pub type CG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG3` writer - Reserved"]
pub type CG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG4` reader - sim_m_clk_r_clk_enable"]
pub type CG4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG4` writer - sim_m_clk_r_clk_enable"]
pub type CG4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG5` reader - dcp clock (dcp_clk_enable)"]
pub type CG5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG5` writer - dcp clock (dcp_clk_enable)"]
pub type CG5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG6` reader - lpuart3 clock (lpuart3_clk_enable)"]
pub type CG6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG6` writer - lpuart3 clock (lpuart3_clk_enable)"]
pub type CG6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG7` reader - can1 clock (can1_clk_enable)"]
pub type CG7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG7` writer - can1 clock (can1_clk_enable)"]
pub type CG7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG8` reader - can1_serial clock (can1_serial_clk_enable)"]
pub type CG8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG8` writer - can1_serial clock (can1_serial_clk_enable)"]
pub type CG8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG9` reader - can2 clock (can2_clk_enable)"]
pub type CG9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG9` writer - can2 clock (can2_clk_enable)"]
pub type CG9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG10` reader - can2_serial clock (can2_serial_clk_enable)"]
pub type CG10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG10` writer - can2_serial clock (can2_serial_clk_enable)"]
pub type CG10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG11` reader - trace clock (trace_clk_enable)"]
pub type CG11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG11` writer - trace clock (trace_clk_enable)"]
pub type CG11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG12` reader - gpt2 bus clocks (gpt2_bus_clk_enable)"]
pub type CG12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG12` writer - gpt2 bus clocks (gpt2_bus_clk_enable)"]
pub type CG12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG13` reader - gpt2 serial clocks (gpt2_serial_clk_enable)"]
pub type CG13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG13` writer - gpt2 serial clocks (gpt2_serial_clk_enable)"]
pub type CG13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG14` reader - lpuart2 clock (lpuart2_clk_enable)"]
pub type CG14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG14` writer - lpuart2 clock (lpuart2_clk_enable)"]
pub type CG14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG15` reader - gpio2_clocks (gpio2_clk_enable)"]
pub type CG15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG15` writer - gpio2_clocks (gpio2_clk_enable)"]
pub type CG15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - aips_tz1 clocks (aips_tz1_clk_enable)"]
    #[inline(always)]
    pub fn cg0(&self) -> CG0_R {
        CG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - aips_tz2 clocks (aips_tz2_clk_enable)"]
    #[inline(always)]
    pub fn cg1(&self) -> CG1_R {
        CG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - mqs clock ( mqs_hmclk_clock_enable)"]
    #[inline(always)]
    pub fn cg2(&self) -> CG2_R {
        CG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn cg3(&self) -> CG3_R {
        CG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - sim_m_clk_r_clk_enable"]
    #[inline(always)]
    pub fn cg4(&self) -> CG4_R {
        CG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - dcp clock (dcp_clk_enable)"]
    #[inline(always)]
    pub fn cg5(&self) -> CG5_R {
        CG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - lpuart3 clock (lpuart3_clk_enable)"]
    #[inline(always)]
    pub fn cg6(&self) -> CG6_R {
        CG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - can1 clock (can1_clk_enable)"]
    #[inline(always)]
    pub fn cg7(&self) -> CG7_R {
        CG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - can1_serial clock (can1_serial_clk_enable)"]
    #[inline(always)]
    pub fn cg8(&self) -> CG8_R {
        CG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - can2 clock (can2_clk_enable)"]
    #[inline(always)]
    pub fn cg9(&self) -> CG9_R {
        CG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - can2_serial clock (can2_serial_clk_enable)"]
    #[inline(always)]
    pub fn cg10(&self) -> CG10_R {
        CG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - trace clock (trace_clk_enable)"]
    #[inline(always)]
    pub fn cg11(&self) -> CG11_R {
        CG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - gpt2 bus clocks (gpt2_bus_clk_enable)"]
    #[inline(always)]
    pub fn cg12(&self) -> CG12_R {
        CG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - gpt2 serial clocks (gpt2_serial_clk_enable)"]
    #[inline(always)]
    pub fn cg13(&self) -> CG13_R {
        CG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - lpuart2 clock (lpuart2_clk_enable)"]
    #[inline(always)]
    pub fn cg14(&self) -> CG14_R {
        CG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - gpio2_clocks (gpio2_clk_enable)"]
    #[inline(always)]
    pub fn cg15(&self) -> CG15_R {
        CG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - aips_tz1 clocks (aips_tz1_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg0(&mut self) -> CG0_W<0> {
        CG0_W::new(self)
    }
    #[doc = "Bits 2:3 - aips_tz2 clocks (aips_tz2_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg1(&mut self) -> CG1_W<2> {
        CG1_W::new(self)
    }
    #[doc = "Bits 4:5 - mqs clock ( mqs_hmclk_clock_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg2(&mut self) -> CG2_W<4> {
        CG2_W::new(self)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cg3(&mut self) -> CG3_W<6> {
        CG3_W::new(self)
    }
    #[doc = "Bits 8:9 - sim_m_clk_r_clk_enable"]
    #[inline(always)]
    #[must_use]
    pub fn cg4(&mut self) -> CG4_W<8> {
        CG4_W::new(self)
    }
    #[doc = "Bits 10:11 - dcp clock (dcp_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg5(&mut self) -> CG5_W<10> {
        CG5_W::new(self)
    }
    #[doc = "Bits 12:13 - lpuart3 clock (lpuart3_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg6(&mut self) -> CG6_W<12> {
        CG6_W::new(self)
    }
    #[doc = "Bits 14:15 - can1 clock (can1_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg7(&mut self) -> CG7_W<14> {
        CG7_W::new(self)
    }
    #[doc = "Bits 16:17 - can1_serial clock (can1_serial_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg8(&mut self) -> CG8_W<16> {
        CG8_W::new(self)
    }
    #[doc = "Bits 18:19 - can2 clock (can2_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg9(&mut self) -> CG9_W<18> {
        CG9_W::new(self)
    }
    #[doc = "Bits 20:21 - can2_serial clock (can2_serial_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg10(&mut self) -> CG10_W<20> {
        CG10_W::new(self)
    }
    #[doc = "Bits 22:23 - trace clock (trace_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg11(&mut self) -> CG11_W<22> {
        CG11_W::new(self)
    }
    #[doc = "Bits 24:25 - gpt2 bus clocks (gpt2_bus_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg12(&mut self) -> CG12_W<24> {
        CG12_W::new(self)
    }
    #[doc = "Bits 26:27 - gpt2 serial clocks (gpt2_serial_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg13(&mut self) -> CG13_W<26> {
        CG13_W::new(self)
    }
    #[doc = "Bits 28:29 - lpuart2 clock (lpuart2_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg14(&mut self) -> CG14_W<28> {
        CG14_W::new(self)
    }
    #[doc = "Bits 30:31 - gpio2_clocks (gpio2_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg15(&mut self) -> CG15_W<30> {
        CG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Clock Gating Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccgr0](index.html) module"]
pub struct CCGR0_SPEC;
impl crate::RegisterSpec for CCGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccgr0::R](R) reader structure"]
impl crate::Readable for CCGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccgr0::W](W) writer structure"]
impl crate::Writable for CCGR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCGR0 to value 0xffff_ffff"]
impl crate::Resettable for CCGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
