#[doc = "Writer for register INTEN_CLR"]
pub type W = crate::W<u32, super::INTEN_CLR>;
#[doc = "Register INTEN_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ILIM0_CLR`"]
pub struct ILIM0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM0_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `IMAT0_CLR`"]
pub struct IMAT0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT0_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `ICAP0_CLR`"]
pub struct ICAP0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP0_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `ILIM1_CLR`"]
pub struct ILIM1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM1_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `IMAT1_CLR`"]
pub struct IMAT1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT1_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `ICAP1_CLR`"]
pub struct ICAP1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP1_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `ILIM2_CLR`"]
pub struct ILIM2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM2_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `IMAT2_CLR`"]
pub struct IMAT2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT2_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `ICAP2_CLR`"]
pub struct ICAP2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP2_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `ABORT_CLR`"]
pub struct ABORT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim0_clr(&mut self) -> ILIM0_CLR_W {
        ILIM0_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat0_clr(&mut self) -> IMAT0_CLR_W {
        IMAT0_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap0_clr(&mut self) -> ICAP0_CLR_W {
        ICAP0_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim1_clr(&mut self) -> ILIM1_CLR_W {
        ILIM1_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat1_clr(&mut self) -> IMAT1_CLR_W {
        IMAT1_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap1_clr(&mut self) -> ICAP1_CLR_W {
        ICAP1_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim2_clr(&mut self) -> ILIM2_CLR_W {
        ILIM2_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat2_clr(&mut self) -> IMAT2_CLR_W {
        IMAT2_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap2_clr(&mut self) -> ICAP2_CLR_W {
        ICAP2_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn abort_clr(&mut self) -> ABORT_CLR_W {
        ABORT_CLR_W { w: self }
    }
}
