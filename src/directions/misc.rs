// /// Get the relative direction from the target to the current [`Rectangle`](crate::math::collision::Rectangle)
// #[must_use]
// pub fn direction_rect_to_rect<
//     T: Copy
//         + mirl_extensions::One
//         + core::ops::Add<Output = T>
//         + core::ops::Div<Output = T>
//         + ConstNumbers128
//         + core::cmp::PartialOrd
//         + core::ops::Sub<Output = T>
//         + core::ops::Mul<Output = T>
//         + mirl_extensions::Zero
//         + mirl_extensions::Abs,
//     const CS: bool,
// >(
//     current: &crate::math::geometry::Pos2D<
//         T,
//         crate::math::geometry::d2::rectangle::Rectangle<T, CS>,
//     >,
//     target: &crate::math::geometry::Pos2D<
//         T,
//         crate::math::geometry::d2::rectangle::Rectangle<T, CS>,
//     >,
// ) -> crate::directions::Directions {
//     direction_point_to_point::<T, CS>(
//         current.center(),
//         target.center(),
//         target.get_ratio(),
//     )
// }
