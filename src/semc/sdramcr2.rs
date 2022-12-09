#[doc = "Register `SDRAMCR2` reader"]
pub struct R(crate::R<SDRAMCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRAMCR2` writer"]
pub struct W(crate::W<SDRAMCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRAMCR2_SPEC>;
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
impl From<crate::W<SDRAMCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRAMCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRRC` reader - SELF REFRESH recovery time"]
pub type SRRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRRC` writer - SELF REFRESH recovery time"]
pub type SRRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `REF2REF` reader - REFRESH to REFRESH delay"]
pub type REF2REF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF2REF` writer - REFRESH to REFRESH delay"]
pub type REF2REF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ACT2ACT` reader - ACTIVE to ACTIVE delay"]
pub type ACT2ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT2ACT` writer - ACTIVE to ACTIVE delay"]
pub type ACT2ACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ITO` reader - SDRAM idle timeout"]
pub type ITO_R = crate::FieldReader<u8, ITO_A>;
#[doc = "SDRAM idle timeout\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITO_A {
    #[doc = "0: IDLE timeout period is 256*Prescale period."]
    ITO_0 = 0,
    #[doc = "1: IDLE timeout period is ITO*Prescale period."]
    ITO_1 = 1,
    #[doc = "2: IDLE timeout period is ITO*Prescale period."]
    ITO_2 = 2,
    #[doc = "3: IDLE timeout period is ITO*Prescale period."]
    ITO_3 = 3,
    #[doc = "4: IDLE timeout period is ITO*Prescale period."]
    ITO_4 = 4,
    #[doc = "5: IDLE timeout period is ITO*Prescale period."]
    ITO_5 = 5,
    #[doc = "6: IDLE timeout period is ITO*Prescale period."]
    ITO_6 = 6,
    #[doc = "7: IDLE timeout period is ITO*Prescale period."]
    ITO_7 = 7,
    #[doc = "8: IDLE timeout period is ITO*Prescale period."]
    ITO_8 = 8,
    #[doc = "9: IDLE timeout period is ITO*Prescale period."]
    ITO_9 = 9,
}
impl From<ITO_A> for u8 {
    #[inline(always)]
    fn from(variant: ITO_A) -> Self {
        variant as _
    }
}
impl ITO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ITO_A> {
        match self.bits {
            0 => Some(ITO_A::ITO_0),
            1 => Some(ITO_A::ITO_1),
            2 => Some(ITO_A::ITO_2),
            3 => Some(ITO_A::ITO_3),
            4 => Some(ITO_A::ITO_4),
            5 => Some(ITO_A::ITO_5),
            6 => Some(ITO_A::ITO_6),
            7 => Some(ITO_A::ITO_7),
            8 => Some(ITO_A::ITO_8),
            9 => Some(ITO_A::ITO_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ITO_0`"]
    #[inline(always)]
    pub fn is_ito_0(&self) -> bool {
        *self == ITO_A::ITO_0
    }
    #[doc = "Checks if the value of the field is `ITO_1`"]
    #[inline(always)]
    pub fn is_ito_1(&self) -> bool {
        *self == ITO_A::ITO_1
    }
    #[doc = "Checks if the value of the field is `ITO_2`"]
    #[inline(always)]
    pub fn is_ito_2(&self) -> bool {
        *self == ITO_A::ITO_2
    }
    #[doc = "Checks if the value of the field is `ITO_3`"]
    #[inline(always)]
    pub fn is_ito_3(&self) -> bool {
        *self == ITO_A::ITO_3
    }
    #[doc = "Checks if the value of the field is `ITO_4`"]
    #[inline(always)]
    pub fn is_ito_4(&self) -> bool {
        *self == ITO_A::ITO_4
    }
    #[doc = "Checks if the value of the field is `ITO_5`"]
    #[inline(always)]
    pub fn is_ito_5(&self) -> bool {
        *self == ITO_A::ITO_5
    }
    #[doc = "Checks if the value of the field is `ITO_6`"]
    #[inline(always)]
    pub fn is_ito_6(&self) -> bool {
        *self == ITO_A::ITO_6
    }
    #[doc = "Checks if the value of the field is `ITO_7`"]
    #[inline(always)]
    pub fn is_ito_7(&self) -> bool {
        *self == ITO_A::ITO_7
    }
    #[doc = "Checks if the value of the field is `ITO_8`"]
    #[inline(always)]
    pub fn is_ito_8(&self) -> bool {
        *self == ITO_A::ITO_8
    }
    #[doc = "Checks if the value of the field is `ITO_9`"]
    #[inline(always)]
    pub fn is_ito_9(&self) -> bool {
        *self == ITO_A::ITO_9
    }
}
#[doc = "Field `ITO` writer - SDRAM idle timeout"]
pub type ITO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR2_SPEC, u8, ITO_A, 8, O>;
impl<'a, const O: u8> ITO_W<'a, O> {
    #[doc = "IDLE timeout period is 256*Prescale period."]
    #[inline(always)]
    pub fn ito_0(self) -> &'a mut W {
        self.variant(ITO_A::ITO_0)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_1(self) -> &'a mut W {
        self.variant(ITO_A::ITO_1)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_2(self) -> &'a mut W {
        self.variant(ITO_A::ITO_2)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_3(self) -> &'a mut W {
        self.variant(ITO_A::ITO_3)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_4(self) -> &'a mut W {
        self.variant(ITO_A::ITO_4)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_5(self) -> &'a mut W {
        self.variant(ITO_A::ITO_5)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_6(self) -> &'a mut W {
        self.variant(ITO_A::ITO_6)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_7(self) -> &'a mut W {
        self.variant(ITO_A::ITO_7)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_8(self) -> &'a mut W {
        self.variant(ITO_A::ITO_8)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_9(self) -> &'a mut W {
        self.variant(ITO_A::ITO_9)
    }
}
impl R {
    #[doc = "Bits 0:7 - SELF REFRESH recovery time"]
    #[inline(always)]
    pub fn srrc(&self) -> SRRC_R {
        SRRC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REFRESH to REFRESH delay"]
    #[inline(always)]
    pub fn ref2ref(&self) -> REF2REF_R {
        REF2REF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ACTIVE to ACTIVE delay"]
    #[inline(always)]
    pub fn act2act(&self) -> ACT2ACT_R {
        ACT2ACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SDRAM idle timeout"]
    #[inline(always)]
    pub fn ito(&self) -> ITO_R {
        ITO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SELF REFRESH recovery time"]
    #[inline(always)]
    #[must_use]
    pub fn srrc(&mut self) -> SRRC_W<0> {
        SRRC_W::new(self)
    }
    #[doc = "Bits 8:15 - REFRESH to REFRESH delay"]
    #[inline(always)]
    #[must_use]
    pub fn ref2ref(&mut self) -> REF2REF_W<8> {
        REF2REF_W::new(self)
    }
    #[doc = "Bits 16:23 - ACTIVE to ACTIVE delay"]
    #[inline(always)]
    #[must_use]
    pub fn act2act(&mut self) -> ACT2ACT_W<16> {
        ACT2ACT_W::new(self)
    }
    #[doc = "Bits 24:31 - SDRAM idle timeout"]
    #[inline(always)]
    #[must_use]
    pub fn ito(&mut self) -> ITO_W<24> {
        ITO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr2](index.html) module"]
pub struct SDRAMCR2_SPEC;
impl crate::RegisterSpec for SDRAMCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramcr2::R](R) reader structure"]
impl crate::Readable for SDRAMCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdramcr2::W](W) writer structure"]
impl crate::Writable for SDRAMCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRAMCR2 to value 0x8000_0eee"]
impl crate::Resettable for SDRAMCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0eee;
}
