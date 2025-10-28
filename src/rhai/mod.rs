use rhai::{Array, Dynamic, FnPtr, NativeCallContext};

pub mod osu;

/// Define an in-place sort
pub fn order(ctx: NativeCallContext, mut array: Vec<Dynamic>, comparer: FnPtr) -> Vec<Dynamic> {
    if array.len() <= 1 {
        return array;
    }
    array.sort_by(|x, y| {
        comparer
            .call_raw(&ctx, None, [x.clone(), y.clone()])
            .ok()
            .and_then(|v| {
                v.as_int()
                    .or_else(|_| v.as_bool().map(|v| if v { -1 } else { 1 }))
                    .ok()
            })
            .map_or_else(
                || x.type_id().cmp(&y.type_id()),
                |v| match v {
                    v if v > 0 => std::cmp::Ordering::Greater,
                    v if v < 0 => std::cmp::Ordering::Less,
                    0 => std::cmp::Ordering::Equal,
                    _ => unreachable!("v is {}", v),
                },
            )
    });
    array
}

pub fn register_helpers(engine: &mut rhai::Engine) -> &mut rhai::Engine {
    engine.register_fn("order", order)
}
