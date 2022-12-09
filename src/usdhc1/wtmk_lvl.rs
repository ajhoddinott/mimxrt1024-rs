#[doc = "Register `WTMK_LVL` reader"]
pub struct R(crate::R<WTMK_LVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTMK_LVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTMK_LVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTMK_LVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTMK_LVL` writer"]
pub struct W(crate::W<WTMK_LVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTMK_LVL_SPEC>;
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
impl From<crate::W<WTMK_LVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTMK_LVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_WML` reader - Read watermark level"]
pub type RD_WML_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_WML` writer - Read watermark level"]
pub type RD_WML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTMK_LVL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RD_BRST_LEN` reader - Read burst length due to system restriction, the actual burst length might not exceed 16"]
pub type RD_BRST_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_BRST_LEN` writer - Read burst length due to system restriction, the actual burst length might not exceed 16"]
pub type RD_BRST_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTMK_LVL_SPEC, u8, u8, 5, O>;
#[doc = "Field `WR_WML` reader - Write watermark level"]
pub type WR_WML_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_WML` writer - Write watermark level"]
pub type WR_WML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTMK_LVL_SPEC, u8, u8, 8, O>;
#[doc = "Field `WR_BRST_LEN` reader - Write burst length due to system restriction, the actual burst length might not exceed 16"]
pub type WR_BRST_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_BRST_LEN` writer - Write burst length due to system restriction, the actual burst length might not exceed 16"]
pub type WR_BRST_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTMK_LVL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:7 - Read watermark level"]
    #[inline(always)]
    pub fn rd_wml(&self) -> RD_WML_R {
        RD_WML_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Read burst length due to system restriction, the actual burst length might not exceed 16"]
    #[inline(always)]
    pub fn rd_brst_len(&self) -> RD_BRST_LEN_R {
        RD_BRST_LEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Write watermark level"]
    #[inline(always)]
    pub fn wr_wml(&self) -> WR_WML_R {
        WR_WML_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Write burst length due to system restriction, the actual burst length might not exceed 16"]
    #[inline(always)]
    pub fn wr_brst_len(&self) -> WR_BRST_LEN_R {
        WR_BRST_LEN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read watermark level"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wml(&mut self) -> RD_WML_W<0> {
        RD_WML_W::new(self)
    }
    #[doc = "Bits 8:12 - Read burst length due to system restriction, the actual burst length might not exceed 16"]
    #[inline(always)]
    #[must_use]
    pub fn rd_brst_len(&mut self) -> RD_BRST_LEN_W<8> {
        RD_BRST_LEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Write watermark level"]
    #[inline(always)]
    #[must_use]
    pub fn wr_wml(&mut self) -> WR_WML_W<16> {
        WR_WML_W::new(self)
    }
    #[doc = "Bits 24:28 - Write burst length due to system restriction, the actual burst length might not exceed 16"]
    #[inline(always)]
    #[must_use]
    pub fn wr_brst_len(&mut self) -> WR_BRST_LEN_W<24> {
        WR_BRST_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watermark Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtmk_lvl](index.html) module"]
pub struct WTMK_LVL_SPEC;
impl crate::RegisterSpec for WTMK_LVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wtmk_lvl::R](R) reader structure"]
impl crate::Readable for WTMK_LVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtmk_lvl::W](W) writer structure"]
impl crate::Writable for WTMK_LVL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WTMK_LVL to value 0x0810_0810"]
impl crate::Resettable for WTMK_LVL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0810_0810;
}
