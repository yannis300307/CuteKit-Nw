pub mod circle;
pub mod line;
pub mod nine_parts_rectangle;
pub mod pixel;
pub mod rectangle;
pub mod rectangle_outline;
pub mod rounded_rectangle;
pub mod sprite;
pub mod sprite_tiled;
pub mod text;

pub use circle::DrawCircle;
pub use line::DrawHorizontalLine;
pub use nine_parts_rectangle::DrawNinePartsRectangle;
pub use pixel::DrawPixel;
pub use rectangle::DrawRectangle;
pub use rectangle_outline::DrawRectangleOutline;
pub use rounded_rectangle::DrawRoundedRectangle;
pub use sprite::DrawSprite;
pub use sprite_tiled::DrawSpriteTiled;
pub use text::DrawText;

/// Used by Draw elements to quickly implement Add and Sub from AddAssign and SubAssign.
macro_rules! impl_add_sub {
	($( $name:ident $(< $( $(#$c: tt)? $($lt:lifetime)? $($t:ident)? $(: $t1:tt $(+ $t2:tt )* )? ),+ >)? for $type: ty ),+) => {
		$(
			impl $(< $( $($c)? $($lt)? $($t)? $( : $t1 $(+ $t2 )* )? ),+ >)? core::ops::Add<$type> for $name $(< $( $($lt)? $($t)? ),+ >)? {
				type Output = Self;

				fn add(mut self, rhs: $type) -> Self::Output {
					self += rhs;
					self
				}
			}

			impl $(< $( $($c)? $($lt)? $($t)? $( : $t1 $(+ $t2 )* )? ),+ >)? core::ops::Sub<$type> for $name $(< $( $($lt)? $($t)? ),+ >)? {
				type Output = Self;

				fn sub(mut self, rhs: $type) -> Self::Output {
					self -= rhs;
					self
				}
			}
		)+
	};
}
pub(self) use impl_add_sub;
