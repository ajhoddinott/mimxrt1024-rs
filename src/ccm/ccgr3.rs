#[doc = "Register `CCGR3` reader"]
pub struct R(crate::R<CCGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCGR3` writer"]
pub struct W(crate::W<CCGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCGR3_SPEC>;
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
impl From<crate::W<CCGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CG0` reader - Reserved"]
pub type CG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG0` writer - Reserved"]
pub type CG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG1` reader - lpuart5 clock (lpuart5_clk_enable)"]
pub type CG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG1` writer - lpuart5 clock (lpuart5_clk_enable)"]
pub type CG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG2` reader - semc clocks (semc_clk_enable)"]
pub type CG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG2` writer - semc clocks (semc_clk_enable)"]
pub type CG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG3` reader - lpuart6 clock (lpuart6_clk_enable)"]
pub type CG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG3` writer - lpuart6 clock (lpuart6_clk_enable)"]
pub type CG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG4` reader - aoi1 clock (aoi1_clk_enable)"]
pub type CG4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG4` writer - aoi1 clock (aoi1_clk_enable)"]
pub type CG4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG5` reader - Reserved"]
pub type CG5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG5` writer - Reserved"]
pub type CG5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG6` reader - Reserved"]
pub type CG6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG6` writer - Reserved"]
pub type CG6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG7` reader - ewm clocks (ewm_clk_enable)"]
pub type CG7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG7` writer - ewm clocks (ewm_clk_enable)"]
pub type CG7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG8` reader - wdog1 clock (wdog1_clk_enable)"]
pub type CG8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG8` writer - wdog1 clock (wdog1_clk_enable)"]
pub type CG8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG9` reader - flexram clock (flexram_clk_enable)"]
pub type CG9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG9` writer - flexram clock (flexram_clk_enable)"]
pub type CG9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG10` reader - acmp1 clocks (acmp1_clk_enable)"]
pub type CG10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG10` writer - acmp1 clocks (acmp1_clk_enable)"]
pub type CG10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG11` reader - acmp2 clocks (acmp2_clk_enable)"]
pub type CG11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG11` writer - acmp2 clocks (acmp2_clk_enable)"]
pub type CG11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG12` reader - acmp3 clocks (acmp3_clk_enable)"]
pub type CG12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG12` writer - acmp3 clocks (acmp3_clk_enable)"]
pub type CG12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG13` reader - acmp4 clocks (acmp4_clk_enable)"]
pub type CG13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG13` writer - acmp4 clocks (acmp4_clk_enable)"]
pub type CG13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG14` reader - The OCRAM clock cannot be turned off when the CM cache is running on this device."]
pub type CG14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG14` writer - The OCRAM clock cannot be turned off when the CM cache is running on this device."]
pub type CG14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CG15` reader - iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
pub type CG15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CG15` writer - iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
pub type CG15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCGR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn cg0(&self) -> CG0_R {
        CG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - lpuart5 clock (lpuart5_clk_enable)"]
    #[inline(always)]
    pub fn cg1(&self) -> CG1_R {
        CG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - semc clocks (semc_clk_enable)"]
    #[inline(always)]
    pub fn cg2(&self) -> CG2_R {
        CG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - lpuart6 clock (lpuart6_clk_enable)"]
    #[inline(always)]
    pub fn cg3(&self) -> CG3_R {
        CG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - aoi1 clock (aoi1_clk_enable)"]
    #[inline(always)]
    pub fn cg4(&self) -> CG4_R {
        CG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Reserved"]
    #[inline(always)]
    pub fn cg5(&self) -> CG5_R {
        CG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn cg6(&self) -> CG6_R {
        CG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ewm clocks (ewm_clk_enable)"]
    #[inline(always)]
    pub fn cg7(&self) -> CG7_R {
        CG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - wdog1 clock (wdog1_clk_enable)"]
    #[inline(always)]
    pub fn cg8(&self) -> CG8_R {
        CG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - flexram clock (flexram_clk_enable)"]
    #[inline(always)]
    pub fn cg9(&self) -> CG9_R {
        CG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - acmp1 clocks (acmp1_clk_enable)"]
    #[inline(always)]
    pub fn cg10(&self) -> CG10_R {
        CG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - acmp2 clocks (acmp2_clk_enable)"]
    #[inline(always)]
    pub fn cg11(&self) -> CG11_R {
        CG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - acmp3 clocks (acmp3_clk_enable)"]
    #[inline(always)]
    pub fn cg12(&self) -> CG12_R {
        CG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - acmp4 clocks (acmp4_clk_enable)"]
    #[inline(always)]
    pub fn cg13(&self) -> CG13_R {
        CG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The OCRAM clock cannot be turned off when the CM cache is running on this device."]
    #[inline(always)]
    pub fn cg14(&self) -> CG14_R {
        CG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
    #[inline(always)]
    pub fn cg15(&self) -> CG15_R {
        CG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cg0(&mut self) -> CG0_W<0> {
        CG0_W::new(self)
    }
    #[doc = "Bits 2:3 - lpuart5 clock (lpuart5_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg1(&mut self) -> CG1_W<2> {
        CG1_W::new(self)
    }
    #[doc = "Bits 4:5 - semc clocks (semc_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg2(&mut self) -> CG2_W<4> {
        CG2_W::new(self)
    }
    #[doc = "Bits 6:7 - lpuart6 clock (lpuart6_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg3(&mut self) -> CG3_W<6> {
        CG3_W::new(self)
    }
    #[doc = "Bits 8:9 - aoi1 clock (aoi1_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg4(&mut self) -> CG4_W<8> {
        CG4_W::new(self)
    }
    #[doc = "Bits 10:11 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cg5(&mut self) -> CG5_W<10> {
        CG5_W::new(self)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cg6(&mut self) -> CG6_W<12> {
        CG6_W::new(self)
    }
    #[doc = "Bits 14:15 - ewm clocks (ewm_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg7(&mut self) -> CG7_W<14> {
        CG7_W::new(self)
    }
    #[doc = "Bits 16:17 - wdog1 clock (wdog1_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg8(&mut self) -> CG8_W<16> {
        CG8_W::new(self)
    }
    #[doc = "Bits 18:19 - flexram clock (flexram_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg9(&mut self) -> CG9_W<18> {
        CG9_W::new(self)
    }
    #[doc = "Bits 20:21 - acmp1 clocks (acmp1_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg10(&mut self) -> CG10_W<20> {
        CG10_W::new(self)
    }
    #[doc = "Bits 22:23 - acmp2 clocks (acmp2_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg11(&mut self) -> CG11_W<22> {
        CG11_W::new(self)
    }
    #[doc = "Bits 24:25 - acmp3 clocks (acmp3_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg12(&mut self) -> CG12_W<24> {
        CG12_W::new(self)
    }
    #[doc = "Bits 26:27 - acmp4 clocks (acmp4_clk_enable)"]
    #[inline(always)]
    #[must_use]
    pub fn cg13(&mut self) -> CG13_W<26> {
        CG13_W::new(self)
    }
    #[doc = "Bits 28:29 - The OCRAM clock cannot be turned off when the CM cache is running on this device."]
    #[inline(always)]
    #[must_use]
    pub fn cg14(&mut self) -> CG14_W<28> {
        CG14_W::new(self)
    }
    #[doc = "Bits 30:31 - iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
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
#[doc = "CCM Clock Gating Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccgr3](index.html) module"]
pub struct CCGR3_SPEC;
impl crate::RegisterSpec for CCGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccgr3::R](R) reader structure"]
impl crate::Readable for CCGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccgr3::W](W) writer structure"]
impl crate::Writable for CCGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCGR3 to value 0xffff_ffcf"]
impl crate::Resettable for CCGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffcf;
}
