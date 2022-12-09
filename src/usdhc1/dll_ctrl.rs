#[doc = "Register `DLL_CTRL` reader"]
pub struct R(crate::R<DLL_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLL_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLL_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLL_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLL_CTRL` writer"]
pub struct W(crate::W<DLL_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLL_CTRL_SPEC>;
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
impl From<crate::W<DLL_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLL_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLL_CTRL_ENABLE` reader - DLL and delay chain"]
pub type DLL_CTRL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DLL_CTRL_ENABLE` writer - DLL and delay chain"]
pub type DLL_CTRL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `DLL_CTRL_RESET` reader - DLL reset"]
pub type DLL_CTRL_RESET_R = crate::BitReader<bool>;
#[doc = "Field `DLL_CTRL_RESET` writer - DLL reset"]
pub type DLL_CTRL_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `DLL_CTRL_SLV_FORCE_UPD` reader - DLL slave delay line"]
pub type DLL_CTRL_SLV_FORCE_UPD_R = crate::BitReader<bool>;
#[doc = "Field `DLL_CTRL_SLV_FORCE_UPD` writer - DLL slave delay line"]
pub type DLL_CTRL_SLV_FORCE_UPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET0` reader - DLL slave delay target0"]
pub type DLL_CTRL_SLV_DLY_TARGET0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET0` writer - DLL slave delay target0"]
pub type DLL_CTRL_SLV_DLY_TARGET0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLL_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLL_CTRL_GATE_UPDATE` reader - DLL gate update"]
pub type DLL_CTRL_GATE_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `DLL_CTRL_GATE_UPDATE` writer - DLL gate update"]
pub type DLL_CTRL_GATE_UPDATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE` reader - DLL slave override"]
pub type DLL_CTRL_SLV_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE` writer - DLL slave override"]
pub type DLL_CTRL_SLV_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE_VAL` reader - DLL slave override val"]
pub type DLL_CTRL_SLV_OVERRIDE_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE_VAL` writer - DLL slave override val"]
pub type DLL_CTRL_SLV_OVERRIDE_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLL_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET1` reader - DLL slave delay target1"]
pub type DLL_CTRL_SLV_DLY_TARGET1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET1` writer - DLL slave delay target1"]
pub type DLL_CTRL_SLV_DLY_TARGET1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLL_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLL_CTRL_SLV_UPDATE_INT` reader - Slave delay line update interval"]
pub type DLL_CTRL_SLV_UPDATE_INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL_CTRL_SLV_UPDATE_INT` writer - Slave delay line update interval"]
pub type DLL_CTRL_SLV_UPDATE_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLL_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLL_CTRL_REF_UPDATE_INT` reader - DLL control loop update interval"]
pub type DLL_CTRL_REF_UPDATE_INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL_CTRL_REF_UPDATE_INT` writer - DLL control loop update interval"]
pub type DLL_CTRL_REF_UPDATE_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLL_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - DLL and delay chain"]
    #[inline(always)]
    pub fn dll_ctrl_enable(&self) -> DLL_CTRL_ENABLE_R {
        DLL_CTRL_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL reset"]
    #[inline(always)]
    pub fn dll_ctrl_reset(&self) -> DLL_CTRL_RESET_R {
        DLL_CTRL_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DLL slave delay line"]
    #[inline(always)]
    pub fn dll_ctrl_slv_force_upd(&self) -> DLL_CTRL_SLV_FORCE_UPD_R {
        DLL_CTRL_SLV_FORCE_UPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - DLL slave delay target0"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target0(&self) -> DLL_CTRL_SLV_DLY_TARGET0_R {
        DLL_CTRL_SLV_DLY_TARGET0_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - DLL gate update"]
    #[inline(always)]
    pub fn dll_ctrl_gate_update(&self) -> DLL_CTRL_GATE_UPDATE_R {
        DLL_CTRL_GATE_UPDATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DLL slave override"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override(&self) -> DLL_CTRL_SLV_OVERRIDE_R {
        DLL_CTRL_SLV_OVERRIDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - DLL slave override val"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override_val(&self) -> DLL_CTRL_SLV_OVERRIDE_VAL_R {
        DLL_CTRL_SLV_OVERRIDE_VAL_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - DLL slave delay target1"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target1(&self) -> DLL_CTRL_SLV_DLY_TARGET1_R {
        DLL_CTRL_SLV_DLY_TARGET1_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:27 - Slave delay line update interval"]
    #[inline(always)]
    pub fn dll_ctrl_slv_update_int(&self) -> DLL_CTRL_SLV_UPDATE_INT_R {
        DLL_CTRL_SLV_UPDATE_INT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - DLL control loop update interval"]
    #[inline(always)]
    pub fn dll_ctrl_ref_update_int(&self) -> DLL_CTRL_REF_UPDATE_INT_R {
        DLL_CTRL_REF_UPDATE_INT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL and delay chain"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_enable(&mut self) -> DLL_CTRL_ENABLE_W<0> {
        DLL_CTRL_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - DLL reset"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_reset(&mut self) -> DLL_CTRL_RESET_W<1> {
        DLL_CTRL_RESET_W::new(self)
    }
    #[doc = "Bit 2 - DLL slave delay line"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_force_upd(&mut self) -> DLL_CTRL_SLV_FORCE_UPD_W<2> {
        DLL_CTRL_SLV_FORCE_UPD_W::new(self)
    }
    #[doc = "Bits 3:6 - DLL slave delay target0"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_dly_target0(&mut self) -> DLL_CTRL_SLV_DLY_TARGET0_W<3> {
        DLL_CTRL_SLV_DLY_TARGET0_W::new(self)
    }
    #[doc = "Bit 7 - DLL gate update"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_gate_update(&mut self) -> DLL_CTRL_GATE_UPDATE_W<7> {
        DLL_CTRL_GATE_UPDATE_W::new(self)
    }
    #[doc = "Bit 8 - DLL slave override"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_override(&mut self) -> DLL_CTRL_SLV_OVERRIDE_W<8> {
        DLL_CTRL_SLV_OVERRIDE_W::new(self)
    }
    #[doc = "Bits 9:15 - DLL slave override val"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_override_val(&mut self) -> DLL_CTRL_SLV_OVERRIDE_VAL_W<9> {
        DLL_CTRL_SLV_OVERRIDE_VAL_W::new(self)
    }
    #[doc = "Bits 16:18 - DLL slave delay target1"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_dly_target1(&mut self) -> DLL_CTRL_SLV_DLY_TARGET1_W<16> {
        DLL_CTRL_SLV_DLY_TARGET1_W::new(self)
    }
    #[doc = "Bits 20:27 - Slave delay line update interval"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_update_int(&mut self) -> DLL_CTRL_SLV_UPDATE_INT_W<20> {
        DLL_CTRL_SLV_UPDATE_INT_W::new(self)
    }
    #[doc = "Bits 28:31 - DLL control loop update interval"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_ref_update_int(&mut self) -> DLL_CTRL_REF_UPDATE_INT_W<28> {
        DLL_CTRL_REF_UPDATE_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLL (Delay Line) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll_ctrl](index.html) module"]
pub struct DLL_CTRL_SPEC;
impl crate::RegisterSpec for DLL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dll_ctrl::R](R) reader structure"]
impl crate::Readable for DLL_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dll_ctrl::W](W) writer structure"]
impl crate::Writable for DLL_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLL_CTRL to value 0"]
impl crate::Resettable for DLL_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
