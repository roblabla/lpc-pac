#[doc = "Reader of register MR[%s]"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR[%s]"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCH`"]
pub type MATCH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MATCH`"]
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer counter match value."]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer counter match value."]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
}
